#!/bin/bash
set -e

echo "Setting up VSCode for anya-core development..."

# Create .vscode directory if it doesn't exist
mkdir -p .vscode

# Create settings.json
cat > .vscode/settings.json << 'SETTINGS'
{
  "rust-analyzer.cargo.features": ["hsm", "adapters", "audit"],
  "rust-analyzer.check.command": "clippy",
  "editor.formatOnSave": true,
  "cursor.ai.llm.modelName": "Claude-3-Opus",
  "cursor.ai.codeGeneration.enabled": true,
  "cursor.ai.codeCompletion.enabled": true
}
SETTINGS

# Create launch.json for debugging
cat > .vscode/launch.json << 'LAUNCH'
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug anya-core",
      "cargo": {
        "args": [
          "build",
          "--bin=anya-core",
          "--package=anya-core"
        ],
        "filter": {
          "name": "anya-core",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    },
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug unified_installer",
      "cargo": {
        "args": [
          "build",
          "--bin=unified_installer",
          "--package=anya-core"
        ],
        "filter": {
          "name": "unified_installer",
          "kind": "bin"
        }
      },
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
LAUNCH

echo "VSCode configuration completed!"
