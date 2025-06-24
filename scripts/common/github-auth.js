/**
 * GitHub CLI Authentication Helper
 * 
 * Provides standardized GitHub authentication using GitHub CLI (gh)
 * This approach:
 * 1. Is more secure (no hardcoded credentials)
 * 2. Allows for DAO contribution tracking
 * 3. Aligns with Web5 and Lightning auth flows
 * 4. Integrates with the GitHub CLI
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

/**
 * Checks if GitHub CLI is installed and available
 * @returns {boolean} True if GitHub CLI is available
 */
function isGitHubCliAvailable() {
    try {
        execSync('gh --version', { stdio: ['pipe', 'pipe', 'ignore'] });
        return true;
    } catch (error) {
        return false;
    }
}

/**
 * Checks if GitHub CLI is authenticated
 * @param {boolean} autoRun If true, attempts to login automatically if not authenticated
 * @param {boolean} yesAll If true, uses default options for automated login
 * @returns {boolean} True if authenticated
 */
function isGitHubAuthenticated(autoRun = false, yesAll = false) {
    try {
        execSync('gh auth status', { stdio: ['pipe', 'pipe', 'ignore'] });
        return true;
    } catch (error) {
        // If autoRun is enabled, attempt automatic login
        if (autoRun) {
            console.log('Not authenticated. Attempting automatic login with GitHub CLI...');
            try {
                // Build login command based on yesAll flag
                const loginCmd = yesAll ? 'gh auth login --web' : 'gh auth login --web';
                execSync(loginCmd, { stdio: 'inherit' });

                // Check if login was successful
                try {
                    execSync('gh auth status', { stdio: ['pipe', 'pipe', 'ignore'] });
                    console.log('Successfully authenticated with GitHub CLI.');
                    return true;
                } catch (innerError) {
                    console.error('Automatic login failed.');
                    return false;
                }
            } catch (loginError) {
                console.error('Automatic login failed. Please run "gh auth login" manually.');
                return false;
            }
        } else {
            return false;
        }
    }
}

/**
 * Gets authentication info from GitHub CLI
 * @param {boolean} autoRun If true, attempts to login automatically if not authenticated
 * @param {boolean} yesAll If true, uses default options for automated login
 * @returns {Object} Object containing username and token
 * @throws {Error} If GitHub CLI is not installed or not authenticated
 */
function getGitHubAuthInfo(autoRun = false, yesAll = false) {
    if (!isGitHubCliAvailable()) {
        throw new Error('GitHub CLI (gh) is not installed. Please install it first.');
    }

    if (!isGitHubAuthenticated(autoRun, yesAll)) {
        throw new Error('Not authenticated with GitHub CLI. Please run "gh auth login".');
    }

    try {
        // Get authenticated user info
        const authStatus = execSync('gh auth status --hostname github.com', { stdio: ['pipe', 'pipe', 'ignore'] }).toString();
        const usernameMatch = authStatus.match(/Logged in to github.com account (\S+)/);
        const username = usernameMatch ? usernameMatch[1] : null;

        // Get auth token from GitHub CLI
        const token = execSync('gh auth token', { stdio: ['pipe', 'pipe', 'ignore'] }).toString().trim();

        // Get user email if needed
        const userApiResult = execSync('gh api user', { stdio: ['pipe', 'pipe', 'ignore'] }).toString();
        let email;

        try {
            const userData = JSON.parse(userApiResult);
            email = userData.email;

            // If email is not public, try to get from email API
            if (!email) {
                const emailApiResult = execSync('gh api user/emails', { stdio: ['pipe', 'pipe', 'ignore'] }).toString();
                const emailData = JSON.parse(emailApiResult);
                const primaryEmail = emailData.find(email => email.primary);
                if (primaryEmail) {
                    email = primaryEmail.email;
                }
            }
        } catch (e) {
            // Fallback to git config if GitHub API fails
            try {
                email = execSync('git config --get user.email', { stdio: ['pipe', 'pipe', 'ignore'] }).toString().trim();
            } catch (e) {
                email = null;
            }
        }

        return { username, token, email };
    } catch (error) {
        throw new Error(`Error getting GitHub authentication: ${error.message}`);
    }
}

/**
 * Sets up MCP environment variables for GitHub
 * @param {Object} config Configuration object
 * @param {string} config.defaultOwner Default repository owner
 * @param {string} config.defaultRepo Default repository name
 * @param {boolean} config.autoRun If true, attempts to login automatically if not authenticated
 * @param {boolean} config.yesAll If true, uses default options for automated login
 * @returns {Object} Environment variables object
 */
function setupMcpEnvironment(config = {}) {
    // Default config values
    const defaultConfig = {
        defaultOwner: 'anya-org',
        defaultRepo: 'anya-core',
        autoRun: false,
        yesAll: false
    };

    // Merge with provided config
    const finalConfig = { ...defaultConfig, ...config };

    // Get auth info with auto-run if specified
    const auth = getGitHubAuthInfo(finalConfig.autoRun, finalConfig.yesAll);

    // Create environment variables
    const env = {
        MCP_GITHUB_USERNAME: auth.username,
        MCP_GITHUB_EMAIL: auth.email,
        MCP_GITHUB_DEFAULT_OWNER: finalConfig.defaultOwner,
        MCP_GITHUB_DEFAULT_REPO: finalConfig.defaultRepo,
        GITHUB_TOKEN: auth.token
    };

    // Set process environment variables
    Object.entries(env).forEach(([key, value]) => {
        if (value) {
            process.env[key] = value;
        }
    });

    return env;
}

/**
 * Creates a configuration file for MCP GitHub tools
 * @param {string} configPath Path to save the configuration file
 * @param {Object} config Configuration object
 * @param {string} config.defaultOwner Default repository owner
 * @param {string} config.defaultRepo Default repository name
 * @param {boolean} config.autoRun If true, attempts to login automatically if not authenticated
 * @param {boolean} config.yesAll If true, uses default options for automated login
 * @returns {Object} Configuration object that was created
 */
function createMcpGithubConfig(configPath, config = {}) {
    // Default config values
    const defaultConfig = {
        defaultOwner: 'anya-org',
        defaultRepo: 'anya-core',
        autoRun: false,
        yesAll: false
    };

    // Merge with provided config
    const finalConfig = { ...defaultConfig, ...config };

    // Get auth info with auto-run if specified
    const auth = getGitHubAuthInfo(finalConfig.autoRun, finalConfig.yesAll);

    // Create MCP config object
    const mcpConfig = {
        github: {
            username: auth.username,
            email: auth.email,
            auth_method: 'github-cli',
            default_owner: finalConfig.defaultOwner,
            default_repo: finalConfig.defaultRepo
        },
        user_preferences: {
            log_level: 'INFO',
            auto_update: true,
            auto_run: finalConfig.autoRun,
            yes_all: finalConfig.yesAll
        },
        bitcoin_core: {
            principles: ['decentralization', 'security', 'immutability', 'transparency'],
            version: '24.0'
        }
    };

    // Create directory if it doesn't exist
    const configDir = path.dirname(configPath);
    if (!fs.existsSync(configDir)) {
        fs.mkdirSync(configDir, { recursive: true });
    }

    // Write config file
    fs.writeFileSync(configPath, JSON.stringify(mcpConfig, null, 2));

    return mcpConfig;
}

/**
 * Parses command line arguments for GitHub CLI automation flags
 * @param {Array} args Command line arguments (typically process.argv)
 * @returns {Object} Object containing autoRun and yesAll flags
 */
function parseGitHubCliArgs(args = process.argv) {
    const options = {
        autoRun: false,
        yesAll: false
    };

    // Check for automation flags
    args.forEach(arg => {
        if (arg === '--auto-run') {
            options.autoRun = true;
        } else if (arg === '--yes-all') {
            options.yesAll = true;
        }
    });

    return options;
}

module.exports = {
    isGitHubCliAvailable,
    isGitHubAuthenticated,
    getGitHubAuthInfo,
    setupMcpEnvironment,
    createMcpGithubConfig,
    parseGitHubCliArgs
};
