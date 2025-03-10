#!/usr/bin/env node
/**
 * Anya Bitcoin MCP Server Test Script
 * [AIR-3][AIS-3][AIT-3][AIM-2][AIP-2][BPC-3]
 * 
 * This script tests the Model Context Protocol (MCP) server for Bitcoin development tools.
 * It simulates client requests and validates the responses.
 */

const fs = require('fs');
const path = require('path');
const { spawn } = require('child_process');
const crypto = require('crypto');

// MCP Server path
const MCP_SERVER_PATH = path.join(__dirname, 'mcp-server.js');

// Test log file
const TEST_LOG_FILE = path.join(__dirname, 'mcp-server-test.log');

// Utility: Log to file
function log(message) {
  const timestamp = new Date().toISOString();
  fs.appendFileSync(TEST_LOG_FILE, `${timestamp} - ${message}\n`);
  console.log(`${timestamp} - ${message}`);
}

// Utility: Generate request ID
function generateRequestId() {
  return crypto.randomBytes(8).toString('hex');
}

// Start server and return process
function startServer() {
  log('Starting MCP server...');
  const server = spawn('node', [MCP_SERVER_PATH], {
    stdio: ['pipe', 'pipe', process.stderr]
  });
  
  // Handle unexpected errors
  server.on('error', (error) => {
    log(`Server error: ${error.message}`);
  });
  
  // Handle exit
  server.on('exit', (code) => {
    log(`Server exited with code ${code}`);
  });
  
  return server;
}

// Send request to server and get response
async function sendRequest(server, tool, parameters) {
  return new Promise((resolve, reject) => {
    const requestId = generateRequestId();
    const request = {
      id: requestId,
      tool,
      parameters
    };
    
    log(`Sending request: ${JSON.stringify(request)}`);
    
    // Send request to server
    server.stdin.write(JSON.stringify(request) + '\n');
    
    // Set up data handler
    const dataHandler = (data) => {
      const responseText = data.toString().trim();
      log(`Received response: ${responseText}`);
      
      try {
        const response = JSON.parse(responseText);
        
        // Check if this is the response for our request
        if (response.id === requestId) {
          // Remove listener to avoid processing more data
          server.stdout.removeListener('data', dataHandler);
          resolve(response);
        }
      } catch (error) {
        log(`Error parsing response: ${error.message}`);
        // Continue listening for valid responses
      }
    };
    
    // Listen for responses
    server.stdout.on('data', dataHandler);
    
    // Set timeout
    setTimeout(() => {
      server.stdout.removeListener('data', dataHandler);
      reject(new Error(`Request timed out after 5000ms`));
    }, 5000);
  });
}

// Test cases
const TEST_CASES = [
  {
    name: 'Bitcoin Protocol Validator - Valid Taproot',
    tool: 'validate_bitcoin_protocol',
    parameters: {
      input: 'tr(KEY,{SILENT_LEAF})'
    },
    validate: (response) => {
      return response.status === 'success' && 
             response.result.compliant === true &&
             response.result.standardsChecked.includes('BIP-341');
    }
  },
  {
    name: 'Bitcoin Protocol Validator - Invalid Input',
    tool: 'validate_bitcoin_protocol',
    parameters: {
      input: 'just some random text'
    },
    validate: (response) => {
      return response.status === 'success' && 
             response.result.compliant === false;
    }
  },
  {
    name: 'Taproot Asset Creator',
    tool: 'create_taproot_asset',
    parameters: {
      name: 'TestAsset',
      supply: 1000000,
      precision: 8
    },
    validate: (response) => {
      return response.status === 'success' && 
             response.result.success === true &&
             response.result.asset.asset.name === 'TestAsset' &&
             response.result.asset.asset.supply === 1000000;
    }
  },
  {
    name: 'Bitcoin Security Audit - With Warnings',
    tool: 'audit_bitcoin_security',
    parameters: {
      code: `
        function verifySignature(signature, message) {
          // No input validation
          const result = someSignatureFunction(signature, message);
          return result;
        }
      `
    },
    validate: (response) => {
      return response.status === 'success' && 
             Array.isArray(response.result.details) &&
             response.result.checksPassed < response.result.totalChecks;
    }
  },
  {
    name: 'Bitcoin Security Audit - With Standards',
    tool: 'audit_bitcoin_security',
    parameters: {
      code: `
        function verifyTaprootSignature(signature, message) {
          if (!signature || !message) throw new Error("Invalid inputs");
          const schnorrSignature = convertToSchnorr(signature);
          return verify(schnorrSignature, message, "taproot");
        }
      `,
      standards: ['BIP-341', 'BIP-342']
    },
    validate: (response) => {
      return response.status === 'success' && 
             response.result.totalChecks > 5 && // Basic checks + standards checks
             response.result.score !== undefined;
    }
  },
  {
    name: 'PSBT Generator',
    tool: 'generate_psbt',
    parameters: {
      inputs: [
        { txid: 'abcdef1234567890abcdef1234567890', vout: 0, amount: 1.5 }
      ],
      outputs: [
        { address: 'bc1q...', amount: 1.499 }
      ]
    },
    validate: (response) => {
      return response.status === 'success' && 
             response.result.success === true &&
             response.result.psbt.inputs.length === 1 &&
             response.result.psbt.outputs.length === 1 &&
             response.result.psbt.metadata.compliance['BIP-174'] === true;
    }
  },
  {
    name: 'DLC Verifier - Valid',
    tool: 'verify_dlc',
    parameters: {
      contract: 'oracle|SILENT_LEAF|payout|conditions',
      oraclePublicKey: '02abcdef1234567890abcdef1234567890'
    },
    validate: (response) => {
      return response.status === 'success' && 
             response.result.valid === true &&
             response.result.privacyPreserving === true;
    }
  },
  {
    name: 'DLC Verifier - Invalid',
    tool: 'verify_dlc',
    parameters: {
      contract: 'no_oracle|conditions',
      oraclePublicKey: 'invalid_key'
    },
    validate: (response) => {
      return response.status === 'success' && 
             response.result.valid === false &&
             Array.isArray(response.result.recommendations) &&
             response.result.recommendations.length > 0;
    }
  }
];

// Run tests
async function runTests() {
  // Clear log file
  fs.writeFileSync(TEST_LOG_FILE, '');
  
  log('Starting MCP server tests...');
  
  const server = startServer();
  
  // Wait for server to initialize
  await new Promise(resolve => setTimeout(resolve, 1000));
  
  let passedTests = 0;
  let failedTests = 0;
  
  for (const testCase of TEST_CASES) {
    log(`\n====== Running test: ${testCase.name} ======`);
    
    try {
      const response = await sendRequest(server, testCase.tool, testCase.parameters);
      
      const passed = testCase.validate(response);
      
      if (passed) {
        log(`✅ Test PASSED: ${testCase.name}`);
        passedTests++;
      } else {
        log(`❌ Test FAILED: ${testCase.name}`);
        log(`Expected validation to pass, but it failed`);
        log(`Response: ${JSON.stringify(response, null, 2)}`);
        failedTests++;
      }
    } catch (error) {
      log(`❌ Test ERROR: ${testCase.name}`);
      log(`Error: ${error.message}`);
      failedTests++;
    }
  }
  
  log('\n====== Test Summary ======');
  log(`Total tests: ${TEST_CASES.length}`);
  log(`Passed: ${passedTests}`);
  log(`Failed: ${failedTests}`);
  
  // Shutdown server
  server.stdin.end();
  
  // Return exit code based on test results
  process.exit(failedTests > 0 ? 1 : 0);
}

// Run tests
runTests().catch(error => {
  log(`Fatal error: ${error.message}`);
  process.exit(1);
}); 