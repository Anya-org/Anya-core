#!/usr/bin/env node

/**
 * MCP GitHub Tools Demo Script
 * 
 * Demonstrates GitHub API interactions using GitHub CLI (gh) for authentication.
 * This approach:
 * 1. Is more secure (no hardcoded credentials)
 * 2. Allows for DAO contribution tracking
 * 3. Aligns with Web5 and Lightning auth flows
 * 4. Integrates with the GitHub CLI
 */

const https = require('https');
const { execSync } = require('child_process');
const util = require('util');
const exec = util.promisify(require('child_process').exec);

// Get GitHub auth info from GitHub CLI
function getGitHubAuthInfo() {
    try {
        // Check if GitHub CLI is authenticated
        const authStatus = execSync('gh auth status --hostname github.com', { stdio: ['pipe', 'pipe', 'ignore'] }).toString();

        // Extract username from auth status
        const usernameMatch = authStatus.match(/Logged in to github.com account (\S+)/);
        const username = usernameMatch ? usernameMatch[1] : null;

        // Get the auth token from GitHub CLI
        const token = execSync('gh auth token', { stdio: ['pipe', 'pipe', 'ignore'] }).toString().trim();

        return { username, token };
    } catch (error) {
        console.error('Error getting GitHub authentication:', error.message);
        console.error('Please run "gh auth login" to authenticate with GitHub CLI');
        process.exit(1);
    }
}

// Config for MCP GitHub tools - using GitHub CLI for authentication
const auth = getGitHubAuthInfo();
const config = {
    defaultOwner: 'anya-org',
    defaultRepo: 'anya-core',
    username: auth.username,
    token: auth.token
};

// GitHub API request helper - now with authentication from GitHub CLI
function githubApiRequest(endpoint, options = {}) {
    const requestOptions = {
        hostname: 'api.github.com',
        path: endpoint,
        headers: {
            'User-Agent': 'MCP-GitHub-Demo-Script',
            'Authorization': `token ${config.token}`
        },
        method: 'GET',
        ...options
    };

    return new Promise((resolve, reject) => {
        const req = https.request(requestOptions, (res) => {
            let data = '';
            res.on('data', chunk => data += chunk);
            res.on('end', () => {
                try {
                    resolve(JSON.parse(data));
                } catch (e) {
                    reject(new Error(`Failed to parse GitHub API response: ${e.message}`));
                }
            });
        });
        req.on('error', reject);
        req.end();
    });
}

// GitHub tool simulations - enhanced with auth token
const searchRepositories = async (query) => {
    console.log(`Searching GitHub repositories for: ${query}...`);
    return await githubApiRequest(`/search/repositories?q=${encodeURIComponent(query)}`);
};

const getRepoDetails = async (owner, repo) => {
    console.log(`Getting details for ${owner}/${repo}...`);
    return await githubApiRequest(`/repos/${owner}/${repo}`);
};

const listPullRequests = async (owner, repo, state = 'open') => {
    console.log(`Listing ${state} pull requests for ${owner}/${repo}...`);
    return await githubApiRequest(`/repos/${owner}/${repo}/pulls?state=${state}`);
};

// Main demo
async function runDemo() {
    try {
        console.log('-----------------------------------------');
        console.log('MCP GitHub Tools Demonstration');
        console.log('-----------------------------------------');
        console.log(`Authenticated as: ${config.username}`);
        console.log(`Default Repo: ${config.defaultOwner}/${config.defaultRepo}`);
        console.log('-----------------------------------------');

        // Search repositories
        const searchResults = await searchRepositories('async layer2protocol');
        console.log(`\nFound ${searchResults.total_count} repositories in search`);
        if (searchResults.items?.length) {
            console.log('Top 3 search results:');
            searchResults.items.slice(0, 3).forEach((repo, i) => {
                console.log(`${i + 1}. ${repo.full_name} (${repo.stargazers_count} stars)`);
                console.log(`   Description: ${repo.description || 'No description'}`);
            });
        }

        // Check if the Anya-core repo exists
        const owner = config.defaultOwner;
        const repo = config.defaultRepo;
        try {
            const repoDetails = await getRepoDetails(owner, repo);
            console.log(`\nRepository ${owner}/${repo} exists:`);
            console.log(`Name: ${repoDetails.name}`);
            console.log(`Stars: ${repoDetails.stargazers_count}`);
            console.log(`Forks: ${repoDetails.forks_count}`);
            console.log(`Default Branch: ${repoDetails.default_branch}`);
        } catch {
            console.log(`\nRepository ${owner}/${repo} not found.`);
        }

        console.log('\n-----------------------------------------');
        console.log('MCP GitHub Tools Demo completed successfully!');
        console.log('To fully integrate with MCP, install and configure:');
        console.log('1. @modelcontextprotocol/server-github package');
        console.log('2. Set up proper GitHub authentication');
        console.log('3. Configure the MCP server in your environment');
        console.log('-----------------------------------------');
    } catch (error) {
        console.error('Error running MCP GitHub tools demo:', error.message);
    }
}

runDemo();
