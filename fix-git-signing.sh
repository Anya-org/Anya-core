#!/bin/bash
# Fix Git GPG signing in Codespaces

echo "Configuring Git commit signing..."

# Set email and name if not already set
if [ -z "$(git config --global user.email)" ]; then
  git config --global user.email "botshelomokoka+anya-core@gmail.com"
  echo "Set user.email to botshelomokoka+anya-core@gmail.com"
fi

if [ -z "$(git config --global user.name)" ]; then
  git config --global user.name "bo_thebig"
  echo "Set user.name to bo_thebig"
fi

# Ensure GitHub CLI is installed
if ! command -v gh &> /dev/null; then
  echo "GitHub CLI not found, installing..."
  curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
  sudo chmod go+r /usr/share/keyrings/githubcli-archive-keyring.gpg
  echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
  sudo apt update
  sudo apt install gh -y
else
  echo "GitHub CLI already installed"
fi

# Check if we're in a GitHub Codespace
if [ -n "${GITHUB_CODESPACES}" ] || [ -d "/.codespaces" ]; then
  echo "Detected GitHub Codespaces environment"
  
  # Make sure gpg program is set to gh-gpgsign
  if [ -f "/.codespaces/bin/gh-gpgsign" ]; then
    git config --global gpg.program "/.codespaces/bin/gh-gpgsign"
    echo "Set gpg.program to /.codespaces/bin/gh-gpgsign"
  else
    echo "Warning: /.codespaces/bin/gh-gpgsign not found"
    exit 1
  fi
  
  # Enable commit signing
  git config --global commit.gpgsign true
  echo "Enabled commit.gpgsign"
  
  echo "GPG signing configuration completed for GitHub Codespaces"
  echo "You can now commit with signing!"
  
  # Verify settings
  echo "Verifying Git signing configuration:"
  git config --list | grep -E 'sign|gpg'
  
else
  echo "Not in GitHub Codespaces. Please run the appropriate GPG setup for your environment."
  exit 1
fi
