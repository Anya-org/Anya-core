// Simple MCP GitHub server test script
const { exec } = require('child_process');

// Set environment variables
process.env.MCP_GITHUB_USERNAME = 'Bo_theBig';
process.env.MCP_GITHUB_EMAIL = 'botshelomokoka@gmail.com';
process.env.MCP_GITHUB_DEFAULT_OWNER = 'anya-org';
process.env.MCP_GITHUB_DEFAULT_REPO = 'anya-core';

console.log('MCP GitHub environment variables set');
console.log('Testing GitHub search...');

// Test a simple GitHub search without requiring authentication
exec('curl -s https://api.github.com/search/repositories?q=anya-core', (error, stdout, stderr) => {
    if (error) {
        console.error(`Error: ${error.message}`);
        return;
    }

    if (stderr) {
        console.error(`Stderr: ${stderr}`);
        return;
    }

    try {
        const result = JSON.parse(stdout);
        console.log('GitHub API responded successfully');
        console.log(`Total count: ${result.total_count}`);
        if (result.items && result.items.length > 0) {
            console.log(`First result: ${result.items[0].full_name}`);
        }
        console.log('MCP tools test complete');
    } catch (e) {
        console.error('Failed to parse GitHub API response', e);
    }
});
