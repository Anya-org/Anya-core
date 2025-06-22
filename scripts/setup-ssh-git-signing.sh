#!/bin/bash
# Setup SSH signing for git commits
# Last updated: 2025-06-22

set -e

echo "Setting up SSH signing for git commits..."

# Check if SSH key exists, if not create one
if [ ! -f ~/.ssh/id_ed25519 ]; then
  echo "No SSH key found, creating a new one..."
  ssh-keygen -t ed25519 -C "$(git config user.email)" -f ~/.ssh/id_ed25519 -N ""
else
  echo "Found existing SSH key: ~/.ssh/id_ed25519"
fi

# Display the public key
echo "Your public key is:"
cat ~/.ssh/id_ed25519.pub

# Set up allowed signers file
echo "Setting up allowed signers file..."
email=$(git config user.email)
if [ -z "$email" ]; then
  echo "Git user.email not set. Please set it first:"
  echo "git config --global user.email \"your.email@example.com\""
  exit 1
fi

# Create or update allowed_signers file
echo "$email $(cat ~/.ssh/id_ed25519.pub)" > ~/.ssh/allowed_signers
chmod 644 ~/.ssh/allowed_signers

# Configure git to use SSH signing
echo "Configuring git to use SSH signing..."
git config --global gpg.format ssh
git config --global user.signingkey ~/.ssh/id_ed25519.pub
git config --global gpg.ssh.allowedSignersFile ~/.ssh/allowed_signers
git config --global commit.gpgsign true

echo "Validating configuration..."
git config --list | grep -E "gpg.format|user.signingkey|gpg.ssh.allowedSignersFile|commit.gpgsign"

echo "SSH signing setup complete!"
echo "Use 'git commit -S -m \"Your commit message\"' to create signed commits"
echo "Or simply use 'git commit -m \"Your commit message\"' since signing is now mandatory"

# Check GitHub CLI access
if command -v gh &>/dev/null; then
  echo "Checking GitHub CLI status..."
  if gh auth status &>/dev/null; then
    echo "GitHub CLI is authenticated. You can add this key to GitHub:"
    echo "gh ssh-key add ~/.ssh/id_ed25519.pub -t \"$(hostname) commit signing key\""
  else
    echo "GitHub CLI not authenticated. To add this key to GitHub, login first:"
    echo "gh auth login"
    echo "Then add the key with: gh ssh-key add ~/.ssh/id_ed25519.pub -t \"$(hostname) commit signing key\""
  fi
else
  echo "GitHub CLI not available. You'll need to manually add this key to GitHub."
  echo "Copy the public key shown above and add it to https://github.com/settings/keys"
fi
