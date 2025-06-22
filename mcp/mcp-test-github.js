#!/usr/bin/env node

/**
 * Simple MCP GitHub API Test
 * Verifies that basic GitHub API functionality works
 */

// Simple GitHub API client
class GitHubClient {
  async getRepoInfo(owner, repo) {
    try {
      const response = await fetch(`https://api.github.com/repos/${owner}/${repo}`);
      if (!response.ok) {
        throw new Error(`HTTP error! Status: ${response.status}`);
      }
      return await response.json();
    } catch (error) {
      console.error('Error fetching repository information:', error);
      throw error;
    }
  }
}

// Test function
async function testGitHubApi() {
  console.log('Testing GitHub API access...');
  
  // Get owner and repo from environment variables or use defaults
  const owner = process.env.MCP_GITHUB_DEFAULT_OWNER || 'anya-org';
  const repo = process.env.MCP_GITHUB_DEFAULT_REPO || 'anya-core';
  
  console.log(`Looking up repository: ${owner}/${repo}`);
  
  const client = new GitHubClient();
  try {
    const repoInfo = await client.getRepoInfo(owner, repo);
    console.log('SUCCESS! Repository information retrieved:');
    console.log(`Name: ${repoInfo.name}`);
    console.log(`Description: ${repoInfo.description}`);
    console.log(`Stars: ${repoInfo.stargazers_count}`);
    console.log(`Forks: ${repoInfo.forks_count}`);
    console.log(`Default Branch: ${repoInfo.default_branch}`);
    return true;
  } catch (error) {
    console.error('FAILED to retrieve repository information.');
    return false;
  }
}

// Run the test
testGitHubApi()
  .then(success => {
    console.log(success ? 'MCP GitHub API test completed successfully!' : 'MCP GitHub API test failed.');
    process.exit(success ? 0 : 1);
  })
  .catch(error => {
    console.error('Unexpected error during test:', error);
    process.exit(1);
  });
