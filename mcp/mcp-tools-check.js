#!/usr/bin/env node

/**
 * MCP Tools Status Check
 * 
 * This script tests if MCP tools are available and working in the environment.
 */

const fs = require('fs');
const path = require('path');
const util = require('util');
const { exec } = require('child_process');
const execPromise = util.promisify(exec);

async function testMCPTools() {
  console.log('======================================');
  console.log('MCP Tools Status Check');
  console.log('======================================');

  try {
    // Check Node.js version
    const nodeVersion = await execPromise('node --version');
    console.log(`✅ Node.js version: ${nodeVersion.stdout.trim()}`);

    // Check npm version
    const npmVersion = await execPromise('npm --version');
    console.log(`✅ npm version: ${npmVersion.stdout.trim()}`);

    // Check for MCP packages
    console.log('\nChecking for MCP packages:');
    const mcpPackages = [
      '@modelcontextprotocol/server-github',
      '@modelcontextprotocol/server-filesystem',
      '@modelcontextprotocol/server-brave-search',
      '@modelcontextprotocol/server-sequential-thinking'
    ];

    for (const pkg of mcpPackages) {
      try {
        const { stdout } = await execPromise(`npm list -g ${pkg} || npm list ${pkg}`);
        if (stdout.includes(pkg)) {
          console.log(`✅ ${pkg} is installed`);
        } else {
          console.log(`❌ ${pkg} is not installed`);
        }
      } catch (e) {
        console.log(`❌ ${pkg} is not installed`);
      }
    }

    // Check for MCP environment variables
    console.log('\nChecking MCP environment variables:');
    const envVars = [
      'GITHUB_TOKEN',
      'MCP_GITHUB_USERNAME',
      'MCP_GITHUB_EMAIL',
      'MCP_GITHUB_DEFAULT_OWNER',
      'MCP_GITHUB_DEFAULT_REPO'
    ];

    for (const envVar of envVars) {
      if (process.env[envVar]) {
        const value = envVar === 'GITHUB_TOKEN' ? '********' : process.env[envVar];
        console.log(`✅ ${envVar} is set to: ${value}`);
      } else {
        console.log(`❌ ${envVar} is not set`);
      }
    }

    // Check if MCP server can be started
    console.log('\nTesting MCP server startup:');
    try {
      // Try to start the server briefly and check if it's working
      const testServerCmd = 'npx @modelcontextprotocol/server-github --help';
      const { stdout } = await execPromise(testServerCmd);
      if (stdout) {
        console.log('✅ MCP GitHub server can be started');
        console.log(stdout.slice(0, 200) + '...');
      } else {
        console.log('❓ MCP GitHub server response was empty');
      }
    } catch (error) {
      console.log('❌ Failed to start MCP GitHub server:');
      console.error(error.message);
    }

    // Check if we have access to GitHub API (doesn't need MCP server)
    console.log('\nTesting GitHub API access (without MCP):');
    try {
      const { stdout } = await execPromise('curl -s https://api.github.com/repos/anya-org/anya-core');
      const result = JSON.parse(stdout);
      if (result.name) {
        console.log(`✅ GitHub API access works (found repo: ${result.name})`);
      } else {
        console.log('❌ GitHub API access failed - unauthorized or repo not found');
      }
    } catch (error) {
      console.log('❌ GitHub API access failed:');
      console.error(error.message);
    }

    // Check for existing MCP configuration
    console.log('\nChecking for MCP configuration files:');
    const mcpConfigFiles = [
      '/workspaces/Anya-core/mcp/toolbox/mcp-tools-config.json',
      '/workspaces/Anya-core/.cursor/mcp.json'
    ];

    for (const configFile of mcpConfigFiles) {
      if (fs.existsSync(configFile)) {
        console.log(`✅ Found MCP config: ${configFile}`);
      } else {
        console.log(`❌ Missing MCP config: ${configFile}`);
      }
    }

    // Summary and recommendations
    console.log('\n======================================');
    console.log('MCP Tools Status Summary');
    console.log('======================================');
    console.log('MCP environment is partially set up but requires some adjustments.');
    console.log('\nRecommendations:');
    console.log('1. Install required MCP packages globally if needed');
    console.log('   npm install -g @modelcontextprotocol/server-github');
    console.log('2. Set up all required environment variables, especially GITHUB_TOKEN');
    console.log('3. Create proper MCP configuration files if missing');
    console.log('4. Check for any firewall or network issues that might block MCP server operation');

  } catch (error) {
    console.error('Error running MCP Tools test:', error.message);
  }
}

testMCPTools();
