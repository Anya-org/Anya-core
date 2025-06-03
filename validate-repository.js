#!/usr/bin/env node

/**
 * Comprehensive Repository Validation Test
 * Tests all MCP, Bitcoin, and Noble curves functionality after updates
 * 
 * [AIR-3][AIS-3][BPC-3][AIT-3][PFM-3][SCL-3][RES-3]
 */

const { spawn } = require('child_process');
const fs = require('fs');
const path = require('path');

// Test configuration
const config = {
  timeout: 30000,
  maxRetries: 3,
  testResults: {
    passed: 0,
    failed: 0,
    skipped: 0,
    total: 0
  }
};

// ANSI color codes for output
const colors = {
  green: '\x1b[32m',
  red: '\x1b[31m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  reset: '\x1b[0m',
  bold: '\x1b[1m'
};

function log(message, level = 'INFO', color = colors.reset) {
  const timestamp = new Date().toISOString();
  console.log(`${color}[${timestamp}] [${level}] ${message}${colors.reset}`);
}

function success(message) {
  log(`✅ ${message}`, 'PASS', colors.green);
  config.testResults.passed++;
}

function failure(message) {
  log(`❌ ${message}`, 'FAIL', colors.red);
  config.testResults.failed++;
}

function warning(message) {
  log(`⚠️  ${message}`, 'WARN', colors.yellow);
  config.testResults.skipped++;
}

function info(message) {
  log(`ℹ️  ${message}`, 'INFO', colors.blue);
}

// Test 1: Verify Noble curves dependency import
async function testNobleCurvesImport() {
  config.testResults.total++;
  try {
    const { secp256k1 } = require('@noble/curves/secp256k1');
    
    if (typeof secp256k1 === 'object' && secp256k1.schnorr) {
      success('Noble curves import successful');
      return true;
    } else {
      failure('Noble curves import succeeded but schnorr not available');
      return false;
    }
  } catch (error) {
    failure(`Noble curves import failed: ${error.message}`);
    return false;
  }
}

// Test 2: Verify MCP server starts correctly
async function testMcpServerStartup() {
  config.testResults.total++;
  return new Promise((resolve) => {
    const server = spawn('node', ['scripts/bitcoin/mcp-server.js'], {
      stdio: ['pipe', 'pipe', 'pipe']
    });
    
    let serverStarted = false;
    const timeout = setTimeout(() => {
      if (!serverStarted) {
        server.kill();
        failure('MCP server startup timeout');
        resolve(false);
      }
    }, 5000);
    
    server.stderr.on('data', (data) => {
      const output = data.toString();
      if (output.includes('Bitcoin MCP Server started')) {
        serverStarted = true;
        clearTimeout(timeout);
        server.kill();
        success('MCP server startup successful');
        resolve(true);
      }
    });
    
    server.on('error', (error) => {
      clearTimeout(timeout);
      failure(`MCP server startup error: ${error.message}`);
      resolve(false);
    });
  });
}

// Test 3: Verify JSON-RPC communication
async function testJsonRpcCommunication() {
  config.testResults.total++;
  return new Promise((resolve) => {
    const server = spawn('node', ['scripts/bitcoin/mcp-server.js'], {
      stdio: ['pipe', 'pipe', 'pipe']
    });
    
    let responseReceived = false;
    const timeout = setTimeout(() => {
      if (!responseReceived) {
        server.kill();
        failure('JSON-RPC communication timeout');
        resolve(false);
      }
    }, 5000);
    
    server.stdout.on('data', (data) => {
      try {
        const response = JSON.parse(data.toString().trim());
        if (response.jsonrpc === '2.0' && response.id === 1) {
          responseReceived = true;
          clearTimeout(timeout);
          server.kill();
          success('JSON-RPC communication successful');
          resolve(true);
        }
      } catch (error) {
        // Continue waiting for valid response
      }
    });
    
    // Wait for server to start, then send test request
    setTimeout(() => {
      const testRequest = {
        jsonrpc: '2.0',
        id: 1,
        method: 'initialize',
        params: {
          protocolVersion: '2024-11-05',
          capabilities: {},
          clientInfo: { name: 'test', version: '1.0.0' }
        }
      };
      
      server.stdin.write(JSON.stringify(testRequest) + '\n');
    }, 1000);
    
    server.on('error', (error) => {
      clearTimeout(timeout);
      failure(`JSON-RPC communication error: ${error.message}`);
      resolve(false);
    });
  });
}

// Test 4: Verify Bitcoin tools functionality
async function testBitcoinTools() {
  config.testResults.total++;
  return new Promise((resolve) => {
    const server = spawn('node', ['scripts/bitcoin/mcp-server.js'], {
      stdio: ['pipe', 'pipe', 'pipe']
    });
    
    let toolCallSuccessful = false;
    const timeout = setTimeout(() => {
      if (!toolCallSuccessful) {
        server.kill();
        failure('Bitcoin tools test timeout');
        resolve(false);
      }
    }, 10000);
    
    server.stdout.on('data', (data) => {
      try {
        const response = JSON.parse(data.toString().trim());
        if (response.jsonrpc === '2.0' && response.id === 2 && response.result && response.result.content) {
          const result = JSON.parse(response.result.content[0].text);
          if (result.success && result.entropy) {
            toolCallSuccessful = true;
            clearTimeout(timeout);
            server.kill();
            success('Bitcoin tools functionality verified');
            resolve(true);
          }
        }
      } catch (error) {
        // Continue waiting for valid response
      }
    });
    
    // Initialize and then call generateSecureRandom
    setTimeout(() => {
      const initRequest = {
        jsonrpc: '2.0',
        id: 1,
        method: 'initialize',
        params: { protocolVersion: '2024-11-05', capabilities: {}, clientInfo: { name: 'test', version: '1.0.0' }}
      };
      server.stdin.write(JSON.stringify(initRequest) + '\n');
      
      setTimeout(() => {
        const toolRequest = {
          jsonrpc: '2.0',
          id: 2,
          method: 'tools/call',
          params: {
            name: 'generateSecureRandom',
            arguments: { length: 32 }
          }
        };
        server.stdin.write(JSON.stringify(toolRequest) + '\n');
      }, 1000);
    }, 1000);
    
    server.on('error', (error) => {
      clearTimeout(timeout);
      failure(`Bitcoin tools test error: ${error.message}`);
      resolve(false);
    });
  });
}

// Test 5: Verify package.json dependencies
async function testPackageJsonDependencies() {
  config.testResults.total++;
  try {
    const packageJsonPath = path.join(process.cwd(), 'package.json');
    const packageJson = JSON.parse(fs.readFileSync(packageJsonPath, 'utf8'));
    
    const requiredDeps = {
      '@noble/curves': true,
      'commander': true,
      'crypto-js': true,
      'node-fetch': true,
      'zod': true
    };
    
    const missing = [];
    for (const dep of Object.keys(requiredDeps)) {
      if (!packageJson.dependencies || !packageJson.dependencies[dep]) {
        missing.push(dep);
      }
    }
    
    if (missing.length === 0) {
      success('Package.json dependencies verified');
      return true;
    } else {
      failure(`Missing dependencies: ${missing.join(', ')}`);
      return false;
    }
  } catch (error) {
    failure(`Package.json test error: ${error.message}`);
    return false;
  }
}

// Test 6: Verify MCP configuration
async function testMcpConfiguration() {
  config.testResults.total++;
  try {
    const mcpConfigPath = path.join(process.cwd(), '.cursor', 'mcp.json');
    if (!fs.existsSync(mcpConfigPath)) {
      failure('MCP configuration file not found');
      return false;
    }
    
    const configContent = fs.readFileSync(mcpConfigPath, 'utf8');
    const cleanContent = configContent.replace(/\/\*[\s\S]*?\*\/|\/\/.*$/gm, '');
    const config = JSON.parse(cleanContent);
    
    if (config.mcpServers && config.mcpServers['anya-bitcoin-tools']) {
      const server = config.mcpServers['anya-bitcoin-tools'];
      if (server.command === 'node' && server.args && server.args.includes('scripts/bitcoin/mcp-server.js')) {
        success('MCP configuration verified');
        return true;
      }
    }
    
    failure('MCP configuration invalid');
    return false;
  } catch (error) {
    failure(`MCP configuration test error: ${error.message}`);
    return false;
  }
}

// Test 7: Verify crypto utilities
async function testCryptoUtils() {
  config.testResults.total++;
  try {
    const cryptoUtils = require('./scripts/bitcoin/crypto-utils');
    
    // Test secure random generation
    const result = cryptoUtils.secureRandomBytes(32);
    if (result && result.length === 32) {
      success('Crypto utilities verified');
      return true;
    } else {
      failure('Crypto utilities test failed');
      return false;
    }
  } catch (error) {
    failure(`Crypto utilities test error: ${error.message}`);
    return false;
  }
}

// Main test runner
async function runAllTests() {
  console.log(`${colors.bold}${colors.blue}🚀 Starting Comprehensive Repository Validation${colors.reset}\n`);
  
  const tests = [
    { name: 'Noble Curves Import', fn: testNobleCurvesImport },
    { name: 'Package.json Dependencies', fn: testPackageJsonDependencies },
    { name: 'MCP Configuration', fn: testMcpConfiguration },
    { name: 'Crypto Utilities', fn: testCryptoUtils },
    { name: 'MCP Server Startup', fn: testMcpServerStartup },
    { name: 'JSON-RPC Communication', fn: testJsonRpcCommunication },
    { name: 'Bitcoin Tools Functionality', fn: testBitcoinTools }
  ];
  
  for (const test of tests) {
    info(`Running: ${test.name}`);
    try {
      await test.fn();
    } catch (error) {
      failure(`${test.name} threw exception: ${error.message}`);
      config.testResults.failed++;
      config.testResults.total++;
    }
    console.log(''); // Add spacing
  }
  
  // Print summary
  console.log(`${colors.bold}📊 Test Results Summary${colors.reset}`);
  console.log(`${colors.green}✅ Passed: ${config.testResults.passed}${colors.reset}`);
  console.log(`${colors.red}❌ Failed: ${config.testResults.failed}${colors.reset}`);
  console.log(`${colors.yellow}⚠️  Skipped: ${config.testResults.skipped}${colors.reset}`);
  console.log(`📋 Total: ${config.testResults.total}`);
  
  const successRate = (config.testResults.passed / config.testResults.total * 100).toFixed(1);
  console.log(`📈 Success Rate: ${successRate}%`);
  
  if (config.testResults.failed === 0) {
    console.log(`\n${colors.bold}${colors.green}🎉 All tests passed! Repository is ready for production.${colors.reset}`);
    process.exit(0);
  } else {
    console.log(`\n${colors.bold}${colors.red}❌ Some tests failed. Please review and fix issues.${colors.reset}`);
    process.exit(1);
  }
}

if (require.main === module) {
  runAllTests().catch(error => {
    console.error(`Test runner failed: ${error.message}`);
    process.exit(1);
  });
}

module.exports = { runAllTests, testNobleCurvesImport, testMcpServerStartup };
