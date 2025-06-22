#!/usr/bin/env node

/**
 * Local Async Layer2 Implementation Status Reporter
 * 
 * This script analyzes the Layer2 implementation status from local files
 * and generates a report on the async implementation across all protocols.
 */

const fs = require('fs');
const path = require('path');
const util = require('util');
const { exec } = require('child_process');
const execPromise = util.promisify(exec);
const readFile = util.promisify(fs.readFile);
const writeFile = util.promisify(fs.writeFile);

// Define the Layer2 protocols we're tracking
const layer2Protocols = [
    'BobClient',
    'LiquidModule',
    'RskClient',
    'StacksClient',
    'TaprootAssetsProtocol',
    'LightningNetwork',
    'StateChannel'
];

// Project root directory
const projectRoot = '/workspaces/Anya-core';

// Helper function to check if a file contains a specific string
async function fileContainsString(filePath, searchString) {
    try {
        const content = await readFile(filePath, 'utf8');
        return content.includes(searchString);
    } catch (error) {
        return false;
    }
}

// Helper function to search for string patterns in project files using grep
async function searchInFiles(pattern, directory = projectRoot) {
    try {
        const { stdout } = await execPromise(`grep -r "${pattern}" ${directory} --include="*.rs" | wc -l`);
        return parseInt(stdout.trim(), 10);
    } catch (error) {
        return 0;
    }
}

// Helper function to check file existence
async function fileExists(filePath) {
    try {
        await fs.promises.access(filePath);
        return true;
    } catch {
        return false;
    }
}

// Main function to analyze and generate report
async function analyzeAsyncLayer2Implementation() {
    try {
        console.log('Starting local analysis of async Layer2 implementation...');

        // Check implementation status for each protocol
        const results = [];
        for (const protocol of layer2Protocols) {
            console.log(`Checking ${protocol}...`);

            // Search for async implementation
            const implPattern = `async fn.*for ${protocol}`;
            const implementationCount = await searchInFiles(implPattern, `${projectRoot}/src`);

            // Search for tests
            const testPattern = `#\\[tokio::test\\].*async fn.*${protocol.toLowerCase()}`;
            const testCount = await searchInFiles(testPattern, `${projectRoot}/tests`);

            results.push({
                protocol,
                implementationFound: implementationCount > 0,
                testsFound: testCount > 0,
                implementationCount,
                testCount
            });
        }

        // Check for the presence of key async files
        const asyncFilesCheck = [
            { name: 'Async Layer2 Implementation Guide', path: `${projectRoot}/ASYNC_LAYER2_IMPLEMENTATION_GUIDE.md` },
            { name: 'Async Layer2 Implementation Status', path: `${projectRoot}/ASYNC_LAYER2_IMPLEMENTATION_STATUS.md` },
            { name: 'Async Layer2 Implementation Complete', path: `${projectRoot}/ASYNC_LAYER2_IMPLEMENTATION_COMPLETE.md` },
            { name: 'Async Layer2 Benchmarks', path: `${projectRoot}/ASYNC_LAYER2_BENCHMARKS.md` },
            { name: 'Layer2 Manager Async Tests', path: `${projectRoot}/tests/layer2_manager_async_tests.rs` },
            { name: 'Layer2 Real World Tests', path: `${projectRoot}/tests/layer2_real_world_tests.rs` },
            { name: 'Layer2 Performance Benchmarks', path: `${projectRoot}/tests/layer2_performance_benchmarks.rs` }
        ];

        for (const file of asyncFilesCheck) {
            file.exists = await fileExists(file.path);
        }

        // Generate report
        const reportDate = new Date().toISOString().split('T')[0];
        let report = `# Async Layer2 Implementation Status\n\n`;
        report += `*Generated on ${reportDate} using local file analysis*\n\n`;
        report += `## Implementation Status\n\n`;
        report += `| Protocol | Async Implementation | Tests | Status |\n`;
        report += `|----------|----------------------|-------|--------|\n`;

        for (const result of results) {
            const status = result.implementationFound && result.testsFound ? '✅ Complete' :
                result.implementationFound ? '🟡 Partial' : '❌ Missing';
            report += `| ${result.protocol} | ${result.implementationFound ? `✅ Yes (${result.implementationCount})` : '❌ No'} | ${result.testsFound ? `✅ Yes (${result.testCount})` : '❌ No'} | ${status} |\n`;
        }

        // Add document status
        report += `\n## Documentation and Test Files\n\n`;
        report += `| File | Status |\n`;
        report += `|------|--------|\n`;

        for (const file of asyncFilesCheck) {
            report += `| ${file.name} | ${file.exists ? '✅ Present' : '❌ Missing'} |\n`;
        }

        // Add summary
        const completeCount = results.filter(r => r.implementationFound && r.testsFound).length;
        const partialCount = results.filter(r => r.implementationFound && !r.testsFound).length;
        const missingCount = results.filter(r => !r.implementationFound).length;

        report += `\n## Summary\n\n`;
        report += `- **Complete implementations**: ${completeCount}/${results.length}\n`;
        report += `- **Partial implementations**: ${partialCount}/${results.length}\n`;
        report += `- **Missing implementations**: ${missingCount}/${results.length}\n`;

        // Calculate completion percentage
        const completionPercentage = Math.round((completeCount / results.length) * 100);
        report += `- **Overall completion**: ${completionPercentage}%\n\n`;

        // Add notes
        report += `## Notes\n\n`;
        report += `This report was generated using local file analysis to examine the codebase. `;
        report += `The analysis searched for async trait implementations and corresponding tests for each Layer2 protocol.\n\n`;
        report += `To update this report, run the \`async-layer2-local-status.js\` script in the MCP toolbox.\n`;

        // Save report to file
        const reportPath = path.join(projectRoot, 'ASYNC_LAYER2_STATUS_REPORT.md');
        await writeFile(reportPath, report);

        console.log(`\nReport generated and saved to: ${reportPath}`);
        console.log(`Summary: ${completeCount}/${results.length} protocols fully implemented (${completionPercentage}%)`);

        // Print the report to console
        console.log('\n--- REPORT PREVIEW ---\n');
        console.log(report);

    } catch (error) {
        console.error('Error analyzing async Layer2 implementation:', error.message);
    }
}

// Run the analysis
analyzeAsyncLayer2Implementation();
