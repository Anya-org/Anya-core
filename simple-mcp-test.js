#!/usr/bin/env node

/**
 * Simple MCP protocol test
 */

const { spawn } = require('child_process');
const path = require('path');

async function testMCP() {
  console.log('Testing MCP Server...');
  
  const serverPath = path.join(__dirname, 'scripts', 'bitcoin', 'mcp-server.js');
  const server = spawn('node', [serverPath]);
  
  let output = '';
  
  server.stdout.on('data', (data) => {
    output += data.toString();
    console.log('Response:', data.toString().trim());
  });
  
  server.stderr.on('data', (data) => {
    console.log('Log:', data.toString().trim());
  });
  
  // Wait for server to start
  setTimeout(() => {
    console.log('Sending initialize request...');
    const initRequest = {
      jsonrpc: '2.0',
      id: 1,
      method: 'initialize',
      params: {
        protocolVersion: '2024-11-05',
        capabilities: {},
        clientInfo: { name: 'test', version: '1.0.0' }
      }
    };
    
    server.stdin.write(JSON.stringify(initRequest) + '\n');
    
    setTimeout(() => {
      console.log('Sending tools/list request...');
      const listRequest = {
        jsonrpc: '2.0',
        id: 2,
        method: 'tools/list',
        params: {}
      };
      
      server.stdin.write(JSON.stringify(listRequest) + '\n');
      
      setTimeout(() => {
        server.kill();
      }, 2000);
    }, 1000);
  }, 1000);
  
  server.on('close', () => {
    console.log('Server closed');
  });
}

testMCP();
