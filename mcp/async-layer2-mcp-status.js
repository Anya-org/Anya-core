#!/usr/bin/env node

/**
 * Local Async Layer2 Implementation Status Reporter (Improved)
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
  { name: 'BobClient', implementation: 'bob.rs' },
  { name: 'LiquidModule', implementation: 'liquid.rs' },
  { name: 'RskClient', implementation: 'rsk.rs' },
  { name: 'StacksClient', implementation: 'stacks.rs' },
  { name: 'TaprootAssetsProtocol', implementation: 'taproot_assets.rs' },
  { name: 'LightningNetwork', implementation: 'lightning/mod.rs' },
  { name: 'StateChannel', implementation: 'state_channels/mod.rs' }
];

// Project root directory
const projectRoot = '/workspaces/Anya-core';

// Helper function to search for string patterns in project files using grep
async function searchInFiles(pattern, directory = projectRoot) {
  try {
    const { stdout } = await execPromise(`cd ${directory} && grep -r "${pattern}" --include="*.rs" . | wc -l`);
    return parseInt(stdout.trim(), 10);
  } catch (error) {
    console.error(`Error searching for "${pattern}":`, error.message);
    return 0;
  }
}

// Helper function to check specific files for patterns
async function searchInSpecificFile(pattern, filePath) {
  try {
    if (!fs.existsSync(filePath)) {
      return 0;
    }
    const { stdout } = await execPromise(`grep -c "${pattern}" ${filePath} || echo 0`);
    return parseInt(stdout.trim(), 10);
  } catch (error) {
    console.error(`Error searching in ${filePath} for "${pattern}":`, error.message);
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

    // Check for the trait definition
    const asyncTraitDef = await searchInFiles("#\\[async_trait\\]\\s*pub\\s+trait\\s+Layer2Protocol");
    console.log(`Found ${asyncTraitDef} async Layer2Protocol trait definitions`);

    // Check implementation status for each protocol
    const results = [];
    for (const protocol of layer2Protocols) {
      console.log(`Checking ${protocol.name}...`);

      const srcPath = path.join(projectRoot, 'src', 'layer2', protocol.implementation);
      const consolidatedPath = path.join(projectRoot, 'consolidated', 'bitcoin', 'layer2', protocol.implementation);

      // Search for async impl pattern - try different variants
      const implPattern1 = `#\\[async_trait\\]\\s*impl\\s+Layer2Protocol\\s+for\\s+${protocol.name}`;
      const implPattern2 = `async\\s+fn\\s+.*for\\s+${protocol.name}`;

      // Check in src directory, then in consolidated if not found
      let implCount = await searchInSpecificFile(implPattern1, srcPath);
      if (implCount === 0) {
        implCount = await searchInSpecificFile(implPattern2, srcPath);
      }
      if (implCount === 0) {
        implCount = await searchInSpecificFile(implPattern1, consolidatedPath);
      }
      if (implCount === 0) {
        implCount = await searchInSpecificFile(implPattern2, consolidatedPath);
      }

      // Also check for generic implementations in the overall codebase
      const genericImplCount = await searchInFiles(`impl.*for\\s+${protocol.name}.*async`);
      const asyncMethodCount = await searchInFiles(`pub\\s+async\\s+fn.*&self.*${protocol.name}`);

      // Search for tests
      const testPattern = `#\\[tokio::test\\]\\s*async\\s+fn\\s+test.*${protocol.name.toLowerCase()}`;
      const testCount = await searchInFiles(testPattern);

      results.push({
        protocol: protocol.name,
        implementationFound: (implCount > 0 || genericImplCount > 0 || asyncMethodCount > 0),
        testsFound: testCount > 0,
        implementationDetails: {
          asyncTraitImplCount: implCount,
          genericAsyncImplCount: genericImplCount,
          asyncMethodCount: asyncMethodCount
        },
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

    // Check for Layer2Manager with async support
    const layer2ManagerAsyncMethods = await searchInFiles("pub\\s+async\\s+fn", path.join(projectRoot, 'src', 'layer2', 'manager.rs'));

    // Generate report
    const reportDate = new Date().toISOString().split('T')[0];
    let report = `# Async Layer2 Implementation Status\n\n`;
    report += `*Generated on ${reportDate} using MCP-powered local file analysis*\n\n`;

    // Status overview
    report += `## Status Overview\n\n`;
    report += `| Component | Status |\n`;
    report += `|-----------|--------|\n`;
    report += `| Layer2Protocol trait | ${asyncTraitDef > 0 ? '✅ Async' : '❌ Not found/Not async'} |\n`;
    report += `| Layer2Manager | ${layer2ManagerAsyncMethods > 0 ? `✅ Has ${layer2ManagerAsyncMethods} async methods` : '❌ No async methods found'} |\n`;

    // Implementation status
    report += `\n## Implementation Status\n\n`;
    report += `| Protocol | Async Implementation | Async Tests | Status |\n`;
    report += `|----------|----------------------|-------------|--------|\n`;

    for (const result of results) {
      const status = result.implementationFound && result.testsFound ? '✅ Complete' :
        result.implementationFound ? '🟡 Partial' : '❌ Missing';

      const implDetails = `${result.implementationFound ? '✅' : '❌'} ${result.implementationDetails.asyncTraitImplCount > 0 ? 'Has trait impl' :
          (result.implementationDetails.genericAsyncImplCount > 0 ? 'Has async impl' :
            (result.implementationDetails.asyncMethodCount > 0 ? 'Has async methods' : 'None found'))
        }`;

      report += `| ${result.protocol} | ${implDetails} | ${result.testsFound ? `✅ ${result.testCount} tests` : '❌ None found'} | ${status} |\n`;
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
    report += `This report was generated using MCP-powered local file analysis to examine the codebase. `;
    report += `The analysis searched for async trait implementations and corresponding tests for each Layer2 protocol.\n\n`;
    report += `The search looked for:\n`;
    report += `- Async trait implementations (#[async_trait] impl Layer2Protocol for...)\n`;
    report += `- Generic async implementations (impl for... with async methods)\n`;
    report += `- Async test cases (#[tokio::test] async fn...)\n\n`;
    report += `To update this report, run the \`async-layer2-mcp-status.js\` script in the MCP toolbox.\n`;

    // Save report to file
    const reportPath = path.join(projectRoot, 'ASYNC_LAYER2_MCP_STATUS_REPORT.md');
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
