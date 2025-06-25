#!/usr/bin/env node
/**
 * Layer2 Status MCP Server
 * Reports on Layer2 implementation status
 * Last updated: 2025-06-22
 */

const { Server } = require('@modelcontextprotocol/sdk/server/index.js');
const { StdioServerTransport } = require('@modelcontextprotocol/sdk/server/stdio.js');
const {
  CallToolRequestSchema,
  ErrorCode,
  ListToolsRequestSchema,
  McpError,
} = require('@modelcontextprotocol/sdk/types.js');

const fs = require('fs').promises;
const path = require('path');
const { exec } = require('child_process');
const util = require('util');
const execPromise = util.promisify(exec);

class Layer2StatusServer {
  constructor() {
    this.server = new Server(
      {
        name: 'layer2-status',
        version: '1.0.0',
      },
      {
        capabilities: {
          tools: {},
        },
      }
    );

    this.setupToolHandlers();
  }

  setupToolHandlers() {
    this.server.setRequestHandler(ListToolsRequestSchema, async () => {
      return {
        tools: [
          {
            name: 'check_layer2_implementation',
            description: 'Check the implementation status of Layer2 protocols',
            inputSchema: {
              type: 'object',
              properties: {
                protocol: {
                  type: 'string',
                  description: 'The Layer2 protocol to check',
                  enum: ['bob', 'liquid', 'rsk', 'stacks', 'taproot_assets', 'lightning', 'state_channels', 'all']
                }
              }
            }
          },
          {
            name: 'generate_layer2_report',
            description: 'Generate a comprehensive report on Layer2 implementation',
            inputSchema: {
              type: 'object',
              properties: {
                format: {
                  type: 'string',
                  description: 'Output format',
                  enum: ['markdown', 'json'],
                  default: 'markdown'
                }
              }
            }
          }
        ]
      };
    });

    this.server.setRequestHandler(CallToolRequestSchema, async (request) => {
      try {
        const { name, arguments: args } = request.params;
        
        switch (name) {
          case 'check_layer2_implementation':
            return await this.checkLayer2Implementation(args);
          case 'generate_layer2_report':
            return await this.generateLayer2Report(args);
          default:
            throw new McpError(
              ErrorCode.MethodNotFound,
              `Unknown tool: ${name}`
            );
        }
      } catch (error) {
        throw new McpError(
          ErrorCode.InternalError,
          `Tool execution failed: ${error.message}`
        );
      }
    });
  }

  async checkLayer2Implementation(args = {}) {
    const { protocol = 'all' } = args;
    const projectRoot = '/workspaces/Anya-core';
    
    try {
      const result = {
        timestamp: new Date().toISOString(),
        protocol: protocol,
        status: {}
      };
      
      // Define protocols to check
      const protocols = protocol === 'all' ? 
        ['bob', 'liquid', 'rsk', 'stacks', 'taproot_assets', 'lightning', 'state_channels'] :
        [protocol];
      
      for (const p of protocols) {
        result.status[p] = await this.checkSingleProtocol(p, projectRoot);
      }
      
      return {
        content: [
          {
            type: 'text',
            text: JSON.stringify(result, null, 2)
          }
        ]
      };
    } catch (error) {
      throw new Error(`Layer2 implementation check failed: ${error.message}`);
    }
  }
  
  async checkSingleProtocol(protocol, projectRoot) {
    // Map protocol name to file path patterns
    const filePatterns = {
      'bob': ['bob.rs', 'bob/mod.rs'],
      'liquid': ['liquid.rs', 'liquid/mod.rs'],
      'rsk': ['rsk.rs', 'rsk/mod.rs'],
      'stacks': ['stacks.rs', 'stacks/mod.rs'],
      'taproot_assets': ['taproot_assets.rs', 'taproot_assets/mod.rs'],
      'lightning': ['lightning.rs', 'lightning/mod.rs'],
      'state_channels': ['state_channels.rs', 'state_channels/mod.rs']
    };
    
    // Get file patterns for this protocol
    const patterns = filePatterns[protocol] || [];
    
    // Check for implementations
    let implementation = {
      found: false,
      asyncFound: false,
      syncFound: false,
      testCount: 0,
      asyncTestCount: 0,
      files: []
    };
    
    // Find relevant files
    for (const pattern of patterns) {
      try {
        const { stdout } = await execPromise(`find ${projectRoot} -path "*/layer2/*${pattern}" -type f`);
        if (stdout) {
          const files = stdout.trim().split('\n');
          for (const file of files) {
            if (file) {
              const fileContent = await fs.readFile(file, 'utf8');
              const isAsync = fileContent.includes('#[async_trait]') || 
                             fileContent.includes('pub async fn');
              
              implementation.found = true;
              implementation.files.push(file);
              
              if (isAsync) {
                implementation.asyncFound = true;
              } else {
                implementation.syncFound = true;
              }
            }
          }
        }
      } catch (error) {
        // Ignore errors here
      }
    }
    
    // Check for tests
    try {
      const { stdout } = await execPromise(`grep -r "fn test_.*${protocol}" --include="*.rs" ${projectRoot}/tests | wc -l`);
      implementation.testCount = parseInt(stdout.trim(), 10);
      
      const { stdout: asyncTestOut } = await execPromise(`grep -r "#\\[tokio::test\\]\\s*async\\s*fn\\s*test_.*${protocol}" --include="*.rs" ${projectRoot}/tests | wc -l`);
      implementation.asyncTestCount = parseInt(asyncTestOut.trim(), 10);
    } catch (error) {
      // Ignore errors here
    }
    
    return implementation;
  }

  async generateLayer2Report(args = {}) {
    const { format = 'markdown' } = args;
    
    try {
      // Get status of all protocols
      const allStatus = await this.checkLayer2Implementation({ protocol: 'all' });
      const statusData = JSON.parse(allStatus.content[0].text);
      
      if (format === 'json') {
        return allStatus;
      }
      
      // Generate markdown report
      let markdown = `# Layer2 Protocol Implementation Status\n\n`;
      markdown += `*Generated on ${new Date().toISOString().split('T')[0]}*\n\n`;
      markdown += `## Implementation Status\n\n`;
      markdown += `| Protocol | Implementation | Async Support | Tests | Async Tests | Status |\n`;
      markdown += `|----------|----------------|--------------|-------|------------|--------|\n`;
      
      const protocols = Object.keys(statusData.status);
      let completeCount = 0;
      let partialCount = 0;
      let missingCount = 0;
      
      for (const protocol of protocols) {
        const data = statusData.status[protocol];
        const status = data.asyncFound && data.asyncTestCount > 0 ? '✅ Complete' : 
                     data.found ? '🟡 Partial' : '❌ Missing';
                     
        if (status === '✅ Complete') completeCount++;
        else if (status === '🟡 Partial') partialCount++;
        else missingCount++;
        
        markdown += `| ${protocol} | ${data.found ? '✅' : '❌'} | ${data.asyncFound ? '✅' : '❌'} | ${data.testCount > 0 ? `✅ (${data.testCount})` : '❌'} | ${data.asyncTestCount > 0 ? `✅ (${data.asyncTestCount})` : '❌'} | ${status} |\n`;
      }
      
      // Add summary
      const totalCount = protocols.length;
      const completionPercentage = Math.round((completeCount / totalCount) * 100);
      
      markdown += `\n## Summary\n\n`;
      markdown += `- **Complete implementations**: ${completeCount}/${totalCount}\n`;
      markdown += `- **Partial implementations**: ${partialCount}/${totalCount}\n`;
      markdown += `- **Missing implementations**: ${missingCount}/${totalCount}\n`;
      markdown += `- **Overall completion**: ${completionPercentage}%\n`;
      
      return {
        content: [
          {
            type: 'text',
            text: markdown
          }
        ]
      };
    } catch (error) {
      throw new Error(`Failed to generate Layer2 report: ${error.message}`);
    }
  }

  async run() {
    try {
      const transport = new StdioServerTransport();
      await this.server.run(transport);
    } catch (error) {
      console.error('Server error:', error);
      process.exit(1);
    }
  }
}

const server = new Layer2StatusServer();
server.run().catch(console.error);
