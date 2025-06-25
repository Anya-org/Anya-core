#!/bin/bash
# Script to make signed commits even when token authentication fails

# Check if commit message is provided
if [ $# -lt 1 ]; then
  echo "Usage: $0 <commit-message> [additional-git-commit-args]"
  exit 1
fi

COMMIT_MSG="$1"
shift
ADDITIONAL_ARGS="$@"

# Temporary disable gpg signing if it's failing
if ! git config --get gpg.program >/dev/null || [ ! -x "$(git config --get gpg.program)" ]; then
  echo "Warning: GPG program not properly configured or not executable"
  # Use SSH key based signing instead
  echo "Switching to SSH key based signing"
  git config --global gpg.format ssh
  git config --global user.signingkey "$(cat ~/.ssh/id_ed25519.pub 2>/dev/null || echo '')"
fi

# Try to commit with signing
echo "Attempting to commit with message: \"$COMMIT_MSG\""
if git commit -S -m "$COMMIT_MSG" $ADDITIONAL_ARGS; then
  echo "Commit successful with signature!"
else
  echo "Signed commit failed. Trying with SSH or alternative signing..."
  
  # Try SSH signing
  if git commit -S -m "$COMMIT_MSG" $ADDITIONAL_ARGS; then
    echo "Commit successful with SSH signature!"
  else
    echo "All signing methods failed. Commit was not made."
    echo
    echo "To proceed, you may need to:"
    echo "1. Set up SSH keys: ssh-keygen -t ed25519"
    echo "2. Configure commit signing without GPG:"
    echo "   git config --global commit.gpgsign false"
    echo "   git commit -m \"$COMMIT_MSG\" $ADDITIONAL_ARGS"
    echo "3. Re-enable signing later: git config --global commit.gpgsign true"
    exit 1
  fi
fi
