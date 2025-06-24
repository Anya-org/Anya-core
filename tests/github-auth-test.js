#!/usr/bin/env node

/**
 * Test script for GitHub CLI authentication module
 * Tests the functionality of the github-auth.js module
 */

const githubAuth = require('../scripts/common/github-auth');

// Parse command line arguments for automation flags
const cliArgs = githubAuth.parseGitHubCliArgs();

async function runTests() {
    console.log('Testing GitHub CLI Authentication Module');
    console.log('======================================');
    console.log(`Auto Run: ${cliArgs.autoRun}`);
    console.log(`Yes All: ${cliArgs.yesAll}`);

    try {
        // Test GitHub CLI availability
        const cliAvailable = githubAuth.isGitHubCliAvailable();
        console.log(`GitHub CLI available: ${cliAvailable}`);

        if (!cliAvailable) {
            console.error('GitHub CLI is not installed. Please install it first.');
            process.exit(1);
        }

        // Test GitHub authentication with potential auto-run
        const isAuthenticated = githubAuth.isGitHubAuthenticated(cliArgs.autoRun, cliArgs.yesAll);
        console.log(`GitHub authenticated: ${isAuthenticated}`);

        if (!isAuthenticated) {
            console.error('Not authenticated with GitHub CLI. Please run "gh auth login".');
            process.exit(1);
        }

        // Test getting GitHub auth info
        const authInfo = githubAuth.getGitHubAuthInfo(cliArgs.autoRun, cliArgs.yesAll);
        console.log('\nGitHub Authentication Info:');
        console.log(`- Username: ${authInfo.username}`);
        console.log(`- Email: ${authInfo.email || '(not available)'}`);
        console.log(`- Token: ${authInfo.token ? '***' + authInfo.token.slice(-4) : '(not available)'}`);

        // Test setting up MCP environment
        const env = githubAuth.setupMcpEnvironment({
            autoRun: cliArgs.autoRun,
            yesAll: cliArgs.yesAll
        });
        console.log('\nMCP Environment Variables:');
        console.log(`- MCP_GITHUB_USERNAME: ${env.MCP_GITHUB_USERNAME}`);
        console.log(`- MCP_GITHUB_EMAIL: ${env.MCP_GITHUB_EMAIL || '(not available)'}`);
        console.log(`- MCP_GITHUB_DEFAULT_OWNER: ${env.MCP_GITHUB_DEFAULT_OWNER}`);
        console.log(`- MCP_GITHUB_DEFAULT_REPO: ${env.MCP_GITHUB_DEFAULT_REPO}`);

        // Test creating MCP GitHub config
        const configPath = '/tmp/mcp-github-config-test.json';
        const config = githubAuth.createMcpGithubConfig(configPath, {
            autoRun: cliArgs.autoRun,
            yesAll: cliArgs.yesAll
        });
        console.log(`\nMCP GitHub config created at: ${configPath}`);

        console.log('\nAll tests completed successfully!');
    } catch (error) {
        console.error(`Error: ${error.message}`);
        process.exit(1);
    }
}

runTests();
