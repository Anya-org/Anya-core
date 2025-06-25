// GitHub CLI Authentication CI/CD Integration Example (JavaScript)
// This script demonstrates how to use the GitHub CLI authentication in CI/CD pipelines with Node.js

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');


// Import GitHub auth helper
const githubAuth = require('../scripts/common/github-auth');

// Parse command line arguments (with CI environment defaults)
const cliArgs = githubAuth.parseGitHubCliArgs();
const config = {
    autoRun: cliArgs.autoRun !== undefined ? cliArgs.autoRun : true,
    yesAll: cliArgs.yesAll !== undefined ? cliArgs.yesAll : true
};

// Main function
async function runCiTasks() {
    console.log('GitHub CLI Authentication CI/CD Integration (JavaScript)');
    console.log('======================================================');
    console.log(`Auto Run: ${config.autoRun}`);
    console.log(`Yes All: ${config.yesAll}`);

    try {
        // Check GitHub CLI availability
        if (!githubAuth.isGitHubCliAvailable()) {
            console.error('GitHub CLI is not installed. Please configure your CI environment with GitHub CLI.');
            process.exit(1);
        }

        // Check GitHub authentication with auto-run
        if (!githubAuth.isGitHubAuthenticated(config.autoRun, config.yesAll)) {
            console.error('Not authenticated with GitHub CLI and automatic login failed.');
            console.error('For CI environments, configure authentication tokens or use GITHUB_TOKEN environment variable.');
            process.exit(1);
        }

        // Get GitHub auth info
        const authInfo = githubAuth.getGitHubAuthInfo(config.autoRun, config.yesAll);
        console.log(`Authenticated as: ${authInfo.username}`);

        // Setup MCP environment
        const env = githubAuth.setupMcpEnvironment({
            autoRun: config.autoRun,
            yesAll: config.yesAll
        });

        // Run additional CI tasks that require authentication
        console.log('Running CI tasks that require GitHub authentication...');

        // Example: Create necessary configuration files
        const ciConfigPath = '/tmp/ci-github-config.json';
        githubAuth.createMcpGithubConfig(ciConfigPath, {
            autoRun: config.autoRun,
            yesAll: config.yesAll
        });
        console.log(`CI configuration created at: ${ciConfigPath}`);

        // Example: Run contribution tracker if available
        const contributionTrackerPath = path.join(__dirname, '../dao/tools/contribution-tracker.js');
        if (fs.existsSync(contributionTrackerPath)) {
            console.log('Running contribution tracker...');
            const contributionArgs = [];
            if (config.autoRun) contributionArgs.push('--auto-run');
            if (config.yesAll) contributionArgs.push('--yes-all');

            try {
                execSync(`node ${contributionTrackerPath} ${contributionArgs.join(' ')}`, {
                    stdio: 'inherit'
                });
            } catch (error) {
                console.error(`Error running contribution tracker: ${error.message}`);
            }
        }

        console.log('CI tasks completed successfully!');
    } catch (error) {
        console.error(`Error: ${error.message}`);
        process.exit(1);
    }
}

// Run the main function
runCiTasks();
