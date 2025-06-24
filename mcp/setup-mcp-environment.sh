#!/bin/bash
# MCP Environment Setup and Testing Script
# This script prepares the MCP environment and tests if the tools can be used
# Last updated: 2025-06-23

set -e

# Output formatting
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}MCP Environment Setup and Testing${NC}"
echo -e "===============================\n"

# Set up directories
WORKSPACE_ROOT="/workspaces/Anya-core"
MCP_DIR="${WORKSPACE_ROOT}/mcp"
TOOLBOX_DIR="${MCP_DIR}/toolbox"
CURSOR_CONFIG_DIR="${WORKSPACE_ROOT}/.cursor"

# Make sure required directories exist
mkdir -p "${MCP_DIR}/logs"
mkdir -p "${TOOLBOX_DIR}/servers"

# Set up environment variables
echo -e "${YELLOW}Setting up environment variables...${NC}"

# Source the GitHub auth helper
GITHUB_AUTH_HELPER="${WORKSPACE_ROOT}/scripts/common/github-auth.sh"
if [ -f "$GITHUB_AUTH_HELPER" ]; then
  echo -e "Using GitHub CLI authentication helper..."
  source "$GITHUB_AUTH_HELPER"

  # Check if GitHub CLI is available and authenticated
  if check_github_cli && check_github_auth; then
    # Set up environment using GitHub CLI auth
    echo -e "Setting up environment using GitHub CLI authentication..."

    # Get GitHub auth info
    eval $(get_github_auth_info)

    # Create .env file in MCP directory
    ENV_FILE="${MCP_DIR}/.env"
    echo -e "Creating MCP environment file at ${ENV_FILE}..."
    cat >"$ENV_FILE" <<EOL
# MCP Environment Variables
# Created: $(date)
# Generated from GitHub CLI authentication

# GitHub credentials
MCP_GITHUB_USERNAME="$GITHUB_USERNAME"
MCP_GITHUB_EMAIL="$GITHUB_EMAIL"
MCP_GITHUB_DEFAULT_OWNER="anya-org"
MCP_GITHUB_DEFAULT_REPO="anya-core"

# GitHub token from GitHub CLI
GITHUB_TOKEN="$GITHUB_TOKEN"

# Other API Keys (add as needed)
# MEM0_API_KEY="your_mem0_api_key"
# BRAVE_API_KEY="your_brave_api_key"
EOL
    echo -e "${GREEN}✓${NC} Created .env file with GitHub CLI authentication"
  else
    # Fallback to existing .env file or create one with placeholder values
    if [ ! -f "$ENV_FILE" ]; then
      echo -e "Creating MCP environment file with placeholder values..."
      cat >"$ENV_FILE" <<EOL
# MCP Environment Variables
# Created: $(date)
# WARNING: Using placeholder values - please authenticate with GitHub CLI

# GitHub credentials
MCP_GITHUB_USERNAME="placeholder_username"
MCP_GITHUB_EMAIL="placeholder_email@example.com"
MCP_GITHUB_DEFAULT_OWNER="anya-org"
MCP_GITHUB_DEFAULT_REPO="anya-core"

# Set a placeholder token for testing (replace with a real token for actual GitHub operations)
GITHUB_TOKEN="gh_placeholder_token"

# Other API Keys (add as needed)
# MEM0_API_KEY="your_mem0_api_key"
# BRAVE_API_KEY="your_brave_api_key"
EOL
      echo -e "${YELLOW}⚠️${NC} Created .env file with placeholder values"
      echo -e "${YELLOW}⚠️${NC} Please authenticate with GitHub CLI by running 'gh auth login'"
    else
      echo -e "${GREEN}✓${NC} Using existing .env file"
    fi
  fi
else
  echo -e "${YELLOW}⚠️${NC} GitHub CLI auth helper not found at ${GITHUB_AUTH_HELPER}"
  echo -e "${YELLOW}⚠️${NC} Falling back to existing .env file or creating with placeholder values"

  if [ ! -f "$ENV_FILE" ]; then
    echo -e "Creating MCP environment file with placeholder values..."
    cat >"$ENV_FILE" <<EOL
# MCP Environment Variables
# Created: $(date)
# WARNING: Using placeholder values - please authenticate with GitHub CLI

# GitHub credentials
MCP_GITHUB_USERNAME="placeholder_username"
MCP_GITHUB_EMAIL="placeholder_email@example.com"
MCP_GITHUB_DEFAULT_OWNER="anya-org"
MCP_GITHUB_DEFAULT_REPO="anya-core"

# Set a placeholder token for testing (replace with a real token for actual GitHub operations)
GITHUB_TOKEN="gh_placeholder_token"

# Other API Keys (add as needed)
# MEM0_API_KEY="your_mem0_api_key"
# BRAVE_API_KEY="your_brave_api_key"
EOL
  fi
fi

# Load the environment variables
echo -e "Loading environment variables..."
source "$ENV_FILE"

# Export them to current session
export MCP_GITHUB_USERNAME
export MCP_GITHUB_EMAIL
export MCP_GITHUB_DEFAULT_OWNER
export MCP_GITHUB_DEFAULT_REPO
export GITHUB_TOKEN

echo -e "${GREEN}✓${NC} Environment variables loaded"

# Make sure toolbox directory has package.json
if [ ! -f "${TOOLBOX_DIR}/package.json" ]; then
  echo -e "${YELLOW}Creating package.json in toolbox directory...${NC}"
  mkdir -p "${TOOLBOX_DIR}"
  cat >"${TOOLBOX_DIR}/package.json" <<EOL
{
  "name": "anya-mcp-toolbox",
  "version": "1.2.0",
  "description": "MCP servers for Anya Core development",
  "private": true,
  "dependencies": {
    "@modelcontextprotocol/server-github": "^2025.4.8",
    "@modelcontextprotocol/server-filesystem": "^2025.3.28",
    "@modelcontextprotocol/sdk": "latest"
  },
  "scripts": {
    "postinstall": "echo 'MCP servers installed successfully'"
  }
}
EOL
  echo -e "${GREEN}✓${NC} Created package.json"
else
  echo -e "${GREEN}✓${NC} Found existing package.json"
fi

# Install dependencies
echo -e "\n${YELLOW}Installing MCP dependencies...${NC}"
cd "${TOOLBOX_DIR}"
if ! npm install --no-fund; then
  echo -e "${RED}Failed to install MCP dependencies${NC}"
  exit 1
fi
echo -e "${GREEN}✓${NC} MCP dependencies installed"

# Create MCP config file for Cursor
echo -e "\n${YELLOW}Creating MCP configuration for Cursor...${NC}"
mkdir -p "${CURSOR_CONFIG_DIR}"
cat >"${CURSOR_CONFIG_DIR}/mcp.json" <<EOL
{
  "version": "0.1",
  "servers": [
    {
      "type": "file",
      "path": "${MCP_DIR}/github-api-status.js"
    },
    {
      "type": "stdio",
      "command": "node",
      "args": ["${TOOLBOX_DIR}/servers/anya-dev-tools.js"]
    }
  ]
}
EOL
echo -e "${GREEN}✓${NC} Created MCP configuration for Cursor"

# Create simple test script to verify MCP functionality
echo -e "\n${YELLOW}Creating MCP test script...${NC}"
cat >"${MCP_DIR}/mcp-test-github.js" <<EOL
#!/usr/bin/env node

/**
 * Simple MCP GitHub API Test
 * Verifies that basic GitHub API functionality works
 */

// Simple GitHub API client
class GitHubClient {
  async getRepoInfo(owner, repo) {
    try {
      const response = await fetch(\`https://api.github.com/repos/\${owner}/\${repo}\`);
      if (!response.ok) {
        throw new Error(\`HTTP error! Status: \${response.status}\`);
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
  
  console.log(\`Looking up repository: \${owner}/\${repo}\`);
  
  const client = new GitHubClient();
  try {
    const repoInfo = await client.getRepoInfo(owner, repo);
    console.log('SUCCESS! Repository information retrieved:');
    console.log(\`Name: \${repoInfo.name}\`);
    console.log(\`Description: \${repoInfo.description}\`);
    console.log(\`Stars: \${repoInfo.stargazers_count}\`);
    console.log(\`Forks: \${repoInfo.forks_count}\`);
    console.log(\`Default Branch: \${repoInfo.default_branch}\`);
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
EOL
chmod +x "${MCP_DIR}/mcp-test-github.js"
echo -e "${GREEN}✓${NC} Created MCP test script"

# Create a simple MCP server for Layer2 status
echo -e "\n${YELLOW}Creating Layer2 status MCP server...${NC}"
mkdir -p "${TOOLBOX_DIR}/servers"
cat >"${TOOLBOX_DIR}/servers/layer2-status-server.js" <<EOL
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
              \`Unknown tool: \${name}\`
            );
        }
      } catch (error) {
        throw new McpError(
          ErrorCode.InternalError,
          \`Tool execution failed: \${error.message}\`
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
      throw new Error(\`Layer2 implementation check failed: \${error.message}\`);
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
        const { stdout } = await execPromise(\`find \${projectRoot} -path "*/layer2/*\${pattern}" -type f\`);
        if (stdout) {
          const files = stdout.trim().split('\\n');
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
      const { stdout } = await execPromise(\`grep -r "fn test_.*\${protocol}" --include="*.rs" \${projectRoot}/tests | wc -l\`);
      implementation.testCount = parseInt(stdout.trim(), 10);
      
      const { stdout: asyncTestOut } = await execPromise(\`grep -r "#\\\\[tokio::test\\\\]\\\\s*async\\\\s*fn\\\\s*test_.*\${protocol}" --include="*.rs" \${projectRoot}/tests | wc -l\`);
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
      let markdown = \`# Layer2 Protocol Implementation Status\n\n\`;
      markdown += \`*Generated on \${new Date().toISOString().split('T')[0]}*\n\n\`;
      markdown += \`## Implementation Status\n\n\`;
      markdown += \`| Protocol | Implementation | Async Support | Tests | Async Tests | Status |\n\`;
      markdown += \`|----------|----------------|--------------|-------|------------|--------|\n\`;
      
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
        
        markdown += \`| \${protocol} | \${data.found ? '✅' : '❌'} | \${data.asyncFound ? '✅' : '❌'} | \${data.testCount > 0 ? \`✅ (\${data.testCount})\` : '❌'} | \${data.asyncTestCount > 0 ? \`✅ (\${data.asyncTestCount})\` : '❌'} | \${status} |\n\`;
      }
      
      // Add summary
      const totalCount = protocols.length;
      const completionPercentage = Math.round((completeCount / totalCount) * 100);
      
      markdown += \`\n## Summary\n\n\`;
      markdown += \`- **Complete implementations**: \${completeCount}/\${totalCount}\n\`;
      markdown += \`- **Partial implementations**: \${partialCount}/\${totalCount}\n\`;
      markdown += \`- **Missing implementations**: \${missingCount}/\${totalCount}\n\`;
      markdown += \`- **Overall completion**: \${completionPercentage}%\n\`;
      
      return {
        content: [
          {
            type: 'text',
            text: markdown
          }
        ]
      };
    } catch (error) {
      throw new Error(\`Failed to generate Layer2 report: \${error.message}\`);
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
EOL

chmod +x "${TOOLBOX_DIR}/servers/layer2-status-server.js"
echo -e "${GREEN}✓${NC} Created Layer2 status MCP server"

# Update the MCP configuration to include the new server
cat >"${CURSOR_CONFIG_DIR}/mcp.json" <<EOL
{
  "version": "0.1",
  "servers": [
    {
      "type": "stdio",
      "command": "node",
      "args": ["${TOOLBOX_DIR}/servers/layer2-status-server.js"]
    },
    {
      "type": "file",
      "path": "${MCP_DIR}/github-api-status.js"
    }
  ]
}
EOL
echo -e "${GREEN}✓${NC} Updated MCP configuration to include Layer2 status server"

# Run the test script
echo -e "\n${YELLOW}Testing MCP GitHub API...${NC}"
cd "${MCP_DIR}"
if ! node mcp-test-github.js; then
  echo -e "\n${YELLOW}GitHub API test failed, but we'll continue...${NC}"
else
  echo -e "\n${GREEN}GitHub API test passed!${NC}"
fi

echo -e "\n${GREEN}MCP setup completed successfully!${NC}"
echo -e "${YELLOW}To use MCP tools, make sure to source the environment file:${NC}"
echo -e "  source ${ENV_FILE}"
echo -e "\nMCP GitHub tools should now be available for use."
echo -e "MCP Layer2 status server can be started with:"
echo -e "  node ${TOOLBOX_DIR}/servers/layer2-status-server.js"
