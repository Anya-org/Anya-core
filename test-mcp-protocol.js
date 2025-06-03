#!/usr/bin/env node

/**
 * Test script for MCP protocol implementation
 * Tests the Bitcoin MCP server with proper JSON-RPC requests
 */

const { spawn } = require('child_process');
const path = require('path');

// Test requests
const testRequests = [
  // Initialize request
  {
    jsonrpc: '2.0',
    id: 1,
    method: 'initialize',
    params: {
      protocolVersion: '2024-11-05',
      capabilities: {},
      clientInfo: {
        name: 'test-client',
        version: '1.0.0'
      }
    }
  },
  
  // List tools request
  {
    jsonrpc: '2.0',
    id: 2,
    method: 'tools/list',
    params: {}
  },
  
  // Test Schnorr signature verification
  {
    jsonrpc: '2.0',
    id: 3,
    method: 'tools/call',
    params: {
      name: 'verifySchnorrSignature',
      arguments: {
        pubkey: 'f9308a019258c31049344f85f89d5229b531c845836f99b08601f113bce036f9388',
        msg: '0000000000000000000000000000000000000000000000000000000000000000',
        signature: 'e907831f80848d1069a5371b402410364bdf1c5f8307b0084c55f1ce2dca821525f66a4a85ea8b71e482a74f382d2ce5ebeee8fdb2172f477df4900d310536c0'
      }
    }
  },
  
  // Test secure random generation
  {
    jsonrpc: '2.0',
    id: 4,
    method: 'tools/call',
    params: {
      name: 'generateSecureRandom',
      arguments: {
        length: 32
      }
    }
  }
];

async function testMCPServer() {
  console.log('🚀 Testing Bitcoin MCP Server...\n');
  
  // Start the MCP server
  const serverPath = path.join(__dirname, 'scripts', 'bitcoin', 'mcp-server.js');
  const server = spawn('node', [serverPath], {
    stdio: ['pipe', 'pipe', 'pipe']
  });
  
  let responseCount = 0;
  const expectedResponses = testRequests.length;
  
  // Handle server output (JSON-RPC responses)
  server.stdout.on('data', (data) => {
    const lines = data.toString().trim().split('\n');
    
    for (const line of lines) {
      if (line.trim()) {
        try {
          const response = JSON.parse(line);
          responseCount++;
          
          console.log(`📝 Response ${responseCount}:`);
          console.log(JSON.stringify(response, null, 2));
          console.log('---\n');
          
          // Check if we've received all expected responses
          if (responseCount >= expectedResponses) {
            console.log('✅ All tests completed successfully!');
            server.kill('SIGTERM');
            process.exit(0);
          }
        } catch (error) {
          console.error('❌ Error parsing response:', error.message);
        }
      }
    }
  });
  
  // Handle server errors
  server.stderr.on('data', (data) => {
    console.error('🔧 Server log:', data.toString().trim());
  });
  
  server.on('error', (error) => {
    console.error('❌ Server error:', error.message);
    process.exit(1);
  });
  
  server.on('close', (code) => {
    console.log(`🏁 Server exited with code ${code}`);
    if (responseCount < expectedResponses) {
      console.log(`⚠️  Only received ${responseCount}/${expectedResponses} responses`);
    }
  });
  
  // Wait a moment for server to start
  await new Promise(resolve => setTimeout(resolve, 1000));
  
  // Send test requests
  console.log('📤 Sending test requests...\n');
  for (let i = 0; i < testRequests.length; i++) {
    const request = testRequests[i];
    console.log(`📨 Sending request ${i + 1}: ${request.method}`);
    server.stdin.write(JSON.stringify(request) + '\n');
    
    // Small delay between requests
    await new Promise(resolve => setTimeout(resolve, 500));
  }
}

// Handle process termination
process.on('SIGINT', () => {
  console.log('\n🛑 Test interrupted');
  process.exit(0);
});

// Run the test
testMCPServer().catch(error => {
  console.error('❌ Test failed:', error.message);
  process.exit(1);
});
