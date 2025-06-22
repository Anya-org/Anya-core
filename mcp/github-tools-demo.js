#!/usr/bin/env node

/**
 * MCP GitHub Tools Demo Script
 * 
 * This script demonstrates how to interact with GitHub using
 * the available tools in the Anya-core project.
 * 
 * It doesn't require a running MCP server, but simulates the
 * functionality that would be available through the MCP tools.
 */

const fs = require('fs');
const path = require('path');
const https = require('https');
const util = require('util');
const exec = util.promisify(require('child_process').exec);

// Set environment variables for MCP GitHub tools
process.env.MCP_GITHUB_USERNAME = 'Bo_theBig';
process.env.MCP_GITHUB_EMAIL = 'botshelomokoka@gmail.com';
process.env.MCP_GITHUB_DEFAULT_OWNER = 'anya-org';
process.env.MCP_GITHUB_DEFAULT_REPO = 'anya-core';

// GitHub API helper function
async function githubApiRequest(endpoint, options = {}) {
    const defaultOptions = {
        hostname: 'api.github.com',
        path: endpoint,
        headers: {
            'User-Agent': 'MCP-GitHub-Demo-Script'
        },
        method: 'GET'
    };

    const requestOptions = { ...defaultOptions, ...options };

    return new Promise((resolve, reject) => {
        const req = https.request(requestOptions, (res) => {
            let data = '';
            res.on('data', (chunk) => {
                data += chunk;
            });
            res.on('end', () => {
                try {
                    const parsedData = JSON.parse(data);
                    resolve(parsedData);
                } catch (e) {
                    reject(new Error(`Failed to parse GitHub API response: ${e.message}`));
                }
            });
        });

        req.on('error', (error) => {
            reject(error);
        });

        req.end();
    });
}

// MCP GitHub tool simulations
async function searchRepositories(query) {
    console.log(`Searching for repositories matching: "${query}"...`);
    const endpoint = `/search/repositories?q=${encodeURIComponent(query)}`;
    return await githubApiRequest(endpoint);
}

async function listIssues(owner, repo) {
    console.log(`Listing issues for ${owner}/${repo}...`);
    const endpoint = `/repos/${owner}/${repo}/issues`;
    return await githubApiRequest(endpoint);
}

async function getRepoDetails(owner, repo) {
    console.log(`Getting details for repository ${owner}/${repo}...`);
    const endpoint = `/repos/${owner}/${repo}`;
    return await githubApiRequest(endpoint);
}

// Main demo function
async function runDemo() {
    try {
        console.log('-----------------------------------------');
        console.log('MCP GitHub Tools Demonstration');
        console.log('-----------------------------------------');
        console.log(`GitHub User: ${process.env.MCP_GITHUB_USERNAME}`);
        console.log(`Default Repo: ${process.env.MCP_GITHUB_DEFAULT_OWNER}/${process.env.MCP_GITHUB_DEFAULT_REPO}`);
        console.log('-----------------------------------------');

        // Demonstrate search repositories
        const searchResults = await searchRepositories('async layer2protocol');
        console.log(`\nFound ${searchResults.total_count} repositories in search`);
        if (searchResults.items && searchResults.items.length > 0) {
            console.log('Top 3 search results:');
            searchResults.items.slice(0, 3).forEach((repo, index) => {
                console.log(`${index + 1}. ${repo.full_name} (${repo.stargazers_count} stars)`);
                console.log(`   Description: ${repo.description || 'No description'}`);
            });
        }

        // Check if the Anya-core repo exists
        let owner = 'anya-org';
        let repo = 'anya-core';
        let repoDetails;

        try {
            repoDetails = await getRepoDetails(owner, repo);
            console.log(`\nRepository ${owner}/${repo} exists:`);
            console.log(`Name: ${repoDetails.name}`);
            console.log(`Stars: ${repoDetails.stargazers_count}`);
            console.log(`Forks: ${repoDetails.forks_count}`);
            console.log(`Default Branch: ${repoDetails.default_branch}`);
        } catch (e) {
            console.log(`\nRepository ${owner}/${repo} not found. Trying alternate repository...`);

            // Try with the username we have
            owner = 'botshelomokoka';
            try {
                repoDetails = await getRepoDetails(owner, repo);
                console.log(`\nAlternate repository ${owner}/${repo} exists:`);
                console.log(`Name: ${repoDetails.name}`);
                console.log(`Stars: ${repoDetails.stargazers_count}`);
                console.log(`Forks: ${repoDetails.forks_count}`);
                console.log(`Default Branch: ${repoDetails.default_branch}`);
            } catch (e2) {
                console.log(`\nCould not find repository ${owner}/${repo} either.`);
            }
        }

        // Success message
        console.log('\n-----------------------------------------');
        console.log('MCP GitHub Tools Demo completed successfully!');
        console.log('This demonstrates the functionality that would be available');
        console.log('through the actual MCP GitHub tools.\n');
        console.log('To fully integrate with MCP, install and configure:');
        console.log('1. @modelcontextprotocol/server-github package');
        console.log('2. Set up proper GitHub authentication');
        console.log('3. Configure the MCP server in your environment');
        console.log('-----------------------------------------');
    } catch (error) {
        console.error('Error running MCP GitHub tools demo:', error.message);
    }
}

// Run the demo
runDemo();
