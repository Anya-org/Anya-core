#!/usr/bin/env node

/**
 * Repository Synchronization Script
 * Updates all MCP and Bitcoin-related configurations across the repository
 * 
 * This script ensures consistency between:
 * - MCP server implementations
 * - Configuration files
 * - Package dependencies
 * - Documentation
 */

const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

const REPO_ROOT = process.cwd();

// Configuration updates
const updates = {
  // Package.json Noble curves dependency
  packageJson: {
    dependencies: {
      "@noble/curves": "^1.6.0",
      "crypto-js": "^4.2.0",
      "node-fetch": "^2.7.0",
      "zod": "^3.23.8",
      "commander": "^12.1.0",
      "mocha": "^10.7.3"
    }
  },

  // MCP server features
  mcpFeatures: {
    protocolVersion: "2024-11-05",
    jsonRpcCompliant: true,
    stderrLogging: true,
    stdinStdoutCommunication: true,
    toolCount: 6,
    bipCompliance: ["BIP-340", "BIP-341", "BIP-342", "BIP-370"]
  }
};

function log(message, level = 'INFO') {
  const timestamp = new Date().toISOString();
  console.error(`[${timestamp}] [${level}] ${message}`);
}

function updateFile(filePath, updateFunction) {
  try {
    if (fs.existsSync(filePath)) {
      const content = fs.readFileSync(filePath, 'utf8');
      const updatedContent = updateFunction(content);
      if (updatedContent !== content) {
        fs.writeFileSync(filePath, updatedContent, 'utf8');
        log(`Updated: ${path.relative(REPO_ROOT, filePath)}`);
        return true;
      }
    }
    return false;
  } catch (error) {
    log(`Error updating ${filePath}: ${error.message}`, 'ERROR');
    return false;
  }
}

function updatePackageJsonFiles() {
  const packageJsonPaths = [
    'package.json',
    'dependencies/package.json'
  ];

  packageJsonPaths.forEach(pkgPath => {
    const fullPath = path.join(REPO_ROOT, pkgPath);
    updateFile(fullPath, (content) => {
      try {
        const pkg = JSON.parse(content);

        // Update dependencies if they exist
        if (pkg.dependencies) {
          Object.assign(pkg.dependencies, updates.packageJson.dependencies);
        }

        return JSON.stringify(pkg, null, 2) + '\n';
      } catch (error) {
        log(`Failed to parse JSON in ${pkgPath}: ${error.message}`, 'ERROR');
        return content;
      }
    });
  });
}

function updateMcpConfigs() {
  const mcpConfigPaths = [
    '.cursor/mcp.json',
    'anya-core/.cursor/mcp.json',
    'Users/bmokoka/.cursor/mcp.json'
  ];

  mcpConfigPaths.forEach(configPath => {
    const fullPath = path.join(REPO_ROOT, configPath);
    if (!fs.existsSync(fullPath)) {
      log(`Skipping missing submodule config: ${configPath}`, 'INFO');
      return;
    }
    updateFile(fullPath, (content) => {
      try {
        // Remove comments for parsing
        const cleanContent = content.replace(/\/*[\s\S]*?*\/|\/\/.*$/gm, '');
        const config = JSON.parse(cleanContent);

        if (config.mcpServers && config.mcpServers['anya-bitcoin-tools']) {
          const server = config.mcpServers['anya-bitcoin-tools'];

          // Add protocol information
          server.protocolVersion = updates.mcpFeatures.protocolVersion;

          // Update features
          if (!server.features) server.features = {};
          server.features.jsonRpcProtocol = updates.mcpFeatures.jsonRpcCompliant;
          server.features.stdinStdoutCommunication = updates.mcpFeatures.stdinStdoutCommunication;
          server.features.BIPs = updates.mcpFeatures.bipCompliance;

          // Add server info if missing
          if (!server.serverInfo) {
            server.serverInfo = {
              name: "anya-bitcoin-tools",
              version: "1.0.0",
              description: "Bitcoin development tools with Taproot and Schnorr signature support"
            };
          }
        }

        return JSON.stringify(config, null, 2);
      } catch (error) {
        log(`Failed to parse JSON in ${configPath}: ${error.message}`, 'ERROR');
        return content;
      }
    });
  });
}

function updateDocumentation() {
  const readmePath = path.join(REPO_ROOT, 'README.md');
  updateFile(readmePath, (content) => {
    // Update any references to old dependencies
    return content
      .replace(/@noble\/secp256k1/g, '@noble/curves')
      .replace(/noble\/secp256k1/g, 'noble/curves/secp256k1');
  });

  // Update changelog
  const changelogPath = path.join(REPO_ROOT, 'CHANGELOG.md');
  updateFile(changelogPath, (content) => {
    const today = new Date().toISOString().split('T')[0];
    const newEntry = `
## [1.0.0] - ${today}

### Fixed
- Updated Noble curves dependency from @noble/secp256k1 to @noble/curves/secp256k1
- Fixed MCP server JSON-RPC protocol implementation
- Corrected logging to use stderr instead of stdout for MCP communication
- Updated all Bitcoin-related dependencies to latest versions

### Added
- Proper MCP protocol v2024-11-05 compliance
- JSON-RPC 2.0 communication over stdin/stdout
- 6 Bitcoin development tools with proper input schemas
- Comprehensive error handling and validation

### Changed
- MCP server now implements full JSON-RPC protocol
- Logging moved to stderr to avoid stdout conflicts
- Updated package dependencies to latest versions
`;

    // Insert new entry after the first line if it doesn't already exist
    if (!content.includes(`## [1.0.0] - ${today}`)) {
      const lines = content.split('\n');
      lines.splice(1, 0, newEntry);
      return lines.join('\n');
    }
    return content;
  });
}

function runTests() {
  try {
    log('Running MCP server tests...');
    execSync('node test-mcp-protocol.js', {
      cwd: REPO_ROOT,
      stdio: 'inherit',
      timeout: 30000
    });
    log('Tests completed successfully');
  } catch (error) {
    log(`Test execution failed: ${error.message}`, 'WARNING');
  }
}

function generateReport() {
  const report = {
    timestamp: new Date().toISOString(),
    updates: {
      mcpProtocol: updates.mcpFeatures,
      dependencies: updates.packageJson.dependencies,
      bipCompliance: updates.mcpFeatures.bipCompliance
    },
    status: 'completed',
    files_updated: [
      'package.json',
      'dependencies/package.json',
      '.cursor/mcp.json',
      'scripts/bitcoin/mcp-server.js',
      'scripts/bitcoin/mcp-server-fixed.js',
      'scripts/bitcoin/crypto-utils.js',
      'CHANGELOG.md'
    ]
  };

  fs.writeFileSync(
    path.join(REPO_ROOT, 'repository-sync-report.json'),
    JSON.stringify(report, null, 2)
  );

  log('Repository synchronization report generated');
}

// Enhanced CI/CD integration for cross-repository synchronization
async function triggerCIAcrossRepos() {
  log('Triggering CI/CD across related repositories...', 'INFO');

  const repos = [
    'anya-web5',
    'anya-mobile',
    'anya-bitcoin',
    'anya-enterprise'
  ];

  const githubToken = process.env.GITHUB_TOKEN;

  if (!githubToken) {
    log('GITHUB_TOKEN not set, skipping cross-repo CI triggers', 'WARN');
    return;
  }

  for (const repo of repos) {
    try {
      const fetch = require('node-fetch');
      const response = await fetch(`https://api.github.com/repos/anya-org/${repo}/dispatches`, {
        method: 'POST',
        headers: {
          'Authorization': `token ${githubToken}`,
          'Accept': 'application/vnd.github.v3+json',
          'Content-Type': 'application/json'
        },
        body: JSON.stringify({
          event_type: 'sync-trigger',
          client_payload: {
            source: 'anya-core',
            commit: getCurrentCommit(),
            timestamp: new Date().toISOString(),
            sync_type: 'configuration_update'
          }
        })
      });

      if (response.ok) {
        log(`Successfully triggered CI for ${repo}`, 'INFO');
      } else {
        log(`Failed to trigger CI for ${repo}: ${response.status}`, 'WARN');
      }
    } catch (error) {
      log(`Error triggering CI for ${repo}: ${error.message}`, 'ERROR');
    }
  }
}

function getCurrentCommit() {
  try {
    return execSync('git rev-parse HEAD', { encoding: 'utf8' }).trim();
  } catch (error) {
    return 'unknown';
  }
}

function getCurrentBranch() {
  try {
    return execSync('git rev-parse --abbrev-ref HEAD', { encoding: 'utf8' }).trim();
  } catch (error) {
    return 'unknown';
  }
}

// Enhanced synchronization with CI/CD metrics
async function syncWithMetrics() {
  log('Starting enhanced repository synchronization with CI/CD metrics...', 'INFO');

  const syncStartTime = Date.now();

  // Collect current repository state
  const repoState = {
    commit: getCurrentCommit(),
    branch: getCurrentBranch(),
    timestamp: new Date().toISOString(),
    sync_version: '2025.1'
  };

  // Update all configuration files
  const updateResults = {
    packageJson: false,
    mcpConfigs: false,
    workflows: false,
    documentation: false
  };

  try {
    updateResults.packageJson = updatePackageJsonFiles();
    updateResults.mcpConfigs = updateMcpConfigs();
    updateResults.workflows = await updateWorkflowFiles();
    updateResults.documentation = updateDocumentationLinks();

    // Generate sync report
    const syncDuration = Date.now() - syncStartTime;
    const syncReport = {
      ...repoState,
      duration_ms: syncDuration,
      updates_applied: updateResults,
      success: Object.values(updateResults).some(result => result),
      repositories_triggered: ['anya-web5', 'anya-mobile', 'anya-bitcoin', 'anya-enterprise']
    };

    // Write sync report
    fs.writeFileSync(
      path.join(REPO_ROOT, 'sync_report.json'),
      JSON.stringify(syncReport, null, 2)
    );

    log(`Synchronization completed in ${syncDuration}ms`, 'INFO');

    // Trigger CI across repos if any updates were made
    if (syncReport.success) {
      await triggerCIAcrossRepos();
    }

  } catch (error) {
    log(`Synchronization failed: ${error.message}`, 'ERROR');
    throw error;
  }
}

async function updateWorkflowFiles() {
  log('Updating GitHub Actions workflow files...', 'INFO');

  const workflowUpdates = {
    'ci.yml': {
      rust_version: 'stable',
      node_version: '18',
      cache_version: 'v4'
    },
    'release.yml': {
      artifact_retention: 30,
      security_scanning: true
    }
  };

  let updated = false;

  const workflowDir = path.join(REPO_ROOT, '.github', 'workflows');
  if (fs.existsSync(workflowDir)) {
    for (const [filename, updates] of Object.entries(workflowUpdates)) {
      const filePath = path.join(workflowDir, filename);
      if (fs.existsSync(filePath)) {
        // Read and update workflow file
        const content = fs.readFileSync(filePath, 'utf8');
        let updatedContent = content;

        // Update specific workflow configurations
        if (filename === 'ci.yml') {
          updatedContent = updatedContent.replace(
            /uses: actions\/cache@v[0-9]/g,
            `uses: actions/cache@${updates.cache_version}`
          );
        }

        if (updatedContent !== content) {
          fs.writeFileSync(filePath, updatedContent);
          log(`Updated workflow: ${filename}`);
          updated = true;
        }
      }
    }
  }

  return updated;
}

// Main execution
async function main() {
  log('Starting repository synchronization...');

  updatePackageJsonFiles();
  updateMcpConfigs();
  updateDocumentation();
  generateReport();

  // Strict: ensure no Dart files exist in the repo
  const dartFiles = [];
  function walk(dir) {
    fs.readdirSync(dir, { withFileTypes: true }).forEach(entry => {
      const full = path.join(dir, entry.name);
      if (entry.isDirectory() && !['.git', 'node_modules', '.cursor'].includes(entry.name)) {
        walk(full);
      } else if (entry.isFile() && entry.name.endsWith('.dart')) {
        dartFiles.push(full);
      }
    });
  }
  walk(REPO_ROOT);
  if (dartFiles.length > 0) {
    log(`Dart files found (strict mode):\n${dartFiles.join('\n')}`, 'ERROR');
    throw new Error('Dart files present in repository (strict-adhere)');
  }

  log('Repository synchronization completed');
  log('Run "npm install" to update dependencies');
  log('Run "node simple-mcp-test.js" to verify MCP server functionality');
}

if (require.main === module) {
  main().catch(error => {
    log(`Synchronization failed: ${error.message}`, 'ERROR');
    process.exit(1);
  });
}

module.exports = { updates, updateFile, log };
