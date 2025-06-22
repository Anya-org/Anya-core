#!/usr/bin/env node

/**
 * Direct GitHub API Layer2 Status Reporter
 * 
 * This script uses the GitHub API directly to report on the Layer2 
 * implementation status, bypassing the need for a running MCP server.
 */

const https = require('https');
const fs = require('fs').promises;

// GitHub repository details
const owner = 'anya-org';
const repo = 'anya-core';

// Function to make a GitHub API request
async function githubApiRequest(path, options = {}) {
  return new Promise((resolve, reject) => {
    const requestOptions = {
      hostname: 'api.github.com',
      path,
      headers: {
        'User-Agent': 'Anya-Layer2-Status-Reporter',
        'Accept': 'application/vnd.github.v3+json',
        // Add authorization if you have a token
        // 'Authorization': `token ${process.env.GITHUB_TOKEN}`
      },
      ...options
    };

    const req = https.request(requestOptions, (res) => {
      let data = '';
      res.on('data', (chunk) => {
        data += chunk;
      });
      res.on('end', () => {
        if (res.statusCode === 200) {
          try {
            resolve(JSON.parse(data));
          } catch (e) {
            reject(new Error(`Failed to parse response: ${e.message}`));
          }
        } else {
          reject(new Error(`API request failed with status code ${res.statusCode}: ${data}`));
        }
      });
    });

    req.on('error', (error) => {
      reject(error);
    });

    req.end();
  });
}

// Function to search for code in the repository
async function searchCode(query) {
  const encodedQuery = encodeURIComponent(`${query} repo:${owner}/${repo}`);
  try {
    return await githubApiRequest(`/search/code?q=${encodedQuery}`);
  } catch (error) {
    console.error(`Search failed for query "${query}": ${error.message}`);
    return { total_count: 0, items: [] };
  }
}

// Function to generate a status report based on search results
async function generateStatusReport() {
  console.log('Generating Layer2 implementation status report using GitHub API...');

  // Define the Layer2 protocols we're checking
  const layer2Protocols = [
    'BobClient',
    'LiquidModule',
    'RskClient',
    'StacksClient',
    'TaprootAssetsProtocol',
    'LightningNetwork',
    'StateChannel'
  ];

  // Results array to store findings
  const results = [];

  // Check for implementation and tests for each protocol
  for (const protocol of layer2Protocols) {
    console.log(`Checking ${protocol}...`);

    // Search for async implementation
    const implQuery = `async fn for ${protocol} language:rust`;
    const implResults = await searchCode(implQuery);

    // Search for tests
    const testQuery = `tokio::test async fn test ${protocol.toLowerCase()} language:rust`;
    const testResults = await searchCode(testQuery);

    results.push({
      protocol,
      implementationFound: implResults.total_count > 0,
      testsFound: testResults.total_count > 0,
      implementationCount: implResults.total_count,
      testCount: testResults.total_count
    });
  }

  // Generate report
  const reportDate = new Date().toISOString().split('T')[0];
  let report = `# Async Layer2 Implementation Status\n\n`;
  report += `*Generated on ${reportDate} using GitHub API*\n\n`;
  report += `## Implementation Status\n\n`;
  report += `| Protocol | Async Implementation | Tests | Status |\n`;
  report += `|----------|----------------------|-------|--------|\n`;

  for (const result of results) {
    const status = result.implementationFound && result.testsFound ? '✅ Complete' :
      result.implementationFound ? '🟡 Partial' : '❌ Missing';
    report += `| ${result.protocol} | ${result.implementationFound ? `✅ Yes (${result.implementationCount})` : '❌ No'} | ${result.testsFound ? `✅ Yes (${result.testCount})` : '❌ No'} | ${status} |\n`;
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
  report += `This report was generated using direct GitHub API access to analyze the codebase. `;
  report += `The analysis searched for async trait implementations and corresponding tests for each Layer2 protocol.\n\n`;
  report += `To update this report, run the \`github-api-status.js\` script.\n`;

  // Return the report
  return {
    report,
    summary: {
      complete: completeCount,
      partial: partialCount,
      missing: missingCount,
      percentage: completionPercentage,
      total: results.length
    }
  };
}

// Main function
async function main() {
  try {
    // Generate the status report
    const { report, summary } = await generateStatusReport();

    // Save the report to a file
    await fs.writeFile('GITHUB_API_LAYER2_STATUS.md', report);

    console.log(`\nReport saved to GITHUB_API_LAYER2_STATUS.md`);
    console.log(`Summary: ${summary.complete}/${summary.total} protocols fully implemented (${summary.percentage}%)`);

    // Print the report
    console.log('\n--- REPORT PREVIEW ---\n');
    console.log(report);

  } catch (error) {
    console.error('Error generating report:', error);
  }
}

// Run the script
main();
