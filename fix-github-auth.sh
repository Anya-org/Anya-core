#!/bin/bash
# Fix GitHub authentication for Codespaces using GitHub CLI

echo "Setting up GitHub authentication..."

# Check if gh is already authenticated
if gh auth status &>/dev/null; then
  echo "GitHub CLI is already authenticated."
  gh auth status
else
  echo "Not authenticated with GitHub CLI."

  # In a Codespace, we can use the built-in GitHub authentication
  if [ -n "$CODESPACES" ] || [ -n "$GITHUB_CODESPACES" ]; then
    echo "Codespace environment detected. Using GitHub CLI auth..."

    # Set codespace permissions to access the repo
    REPO_NAME=$(git remote get-url origin | sed -e 's/.*github.com[:\/]\(.*\).git/\1/')
    echo "Setting permissions for repo: $REPO_NAME"
    gh codespace permissions set --repo $REPO_NAME --permissions write

    # Log in using GitHub CLI (will use codespace identity)
    gh auth login --with-token </dev/null

    if [ $? -eq 0 ]; then
      echo "Authenticated with GitHub through Codespace identity."
    else
      echo "Failed to authenticate with GitHub CLI using Codespace identity."
      echo "Please run 'gh auth login' manually."
      exit 1
    fi
  else
    # Regular authentication
    echo "Please authenticate with GitHub CLI by running:"
    echo "  gh auth login"
    gh auth login
  fi
fi

# Verify authentication
gh auth status
if [ $? -eq 0 ]; then
  echo "GitHub authentication successfully configured!"
  # Get the username for display
  USERNAME=$(gh api user | grep login | cut -d'"' -f4)
  echo "Authenticated as: $USERNAME"
  echo "You can now commit with signing."

  # Set up Git config for commit signing if needed
  if [ -z "$(git config --global --get user.email)" ]; then
    # Set Git config based on GitHub info
    USER_EMAIL=$(gh api user/emails | grep email | head -n 1 | cut -d'"' -f4)
    if [ -n "$USER_EMAIL" ]; then
      git config --global user.email "$USER_EMAIL"
      git config --global user.name "$USERNAME"
      echo "Set up Git user config based on GitHub account."
    fi
  fi

  echo "GitHub CLI-based authentication workflow is complete."
else
  echo "GitHub authentication failed. Please run 'gh auth login' manually."
  exit 1
fi
