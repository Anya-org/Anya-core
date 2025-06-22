#!/bin/bash
# Fix GitHub authentication for Codespaces

echo "Setting up GitHub authentication..."

# Check if GITHUB_TOKEN is set in environment
if [ -z "${GITHUB_TOKEN}" ]; then
  echo "No GITHUB_TOKEN found in environment."
  echo "You need to set a valid GitHub token."
  echo "Please generate a new token with repo and workflow scopes at:"
  echo "https://github.com/settings/tokens"
  
  # Generate a temporary token for this session
  echo
  echo "Creating a temporary GitHub token for this session..."
  echo "This will expire when your codespace session ends."
  
  # Set codespace permissions to access the repo
  gh codespace permissions set --repo $(git remote get-url origin | sed -e 's/.*github.com[:\/]\(.*\).git/\1/') --permissions write
  
  # Get the token from gh codespace
  GH_TOKEN=$(gh codespace ssh -c "cat /workspaces/.codespaces/shared/_.env | grep GITHUB_TOKEN | cut -d= -f2" | tr -d '\r\n')
  
  if [ -n "$GH_TOKEN" ]; then
    export GITHUB_TOKEN="$GH_TOKEN"
    echo "export GITHUB_TOKEN=\"$GH_TOKEN\"" >> ~/.bashrc
    echo "GitHub token set successfully from codespace environment."
  else
    echo "Failed to retrieve a valid GitHub token automatically."
    echo "Please provide one manually by running:"
    echo "  export GITHUB_TOKEN=\"your_token_here\""
    exit 1
  fi
fi

# Set up GitHub CLI authentication with the token
echo "${GITHUB_TOKEN}" | gh auth login --with-token
if [ $? -ne 0 ]; then
  echo "Failed to authenticate with GitHub using the provided token."
  echo "The token may be invalid or expired."
  echo "Please generate a new token and try again."
  exit 1
fi

# Verify authentication
gh auth status
if [ $? -eq 0 ]; then
  echo "GitHub authentication successfully configured!"
  echo "You can now commit with signing."
else
  echo "GitHub authentication failed. Please check your token."
  exit 1
fi
