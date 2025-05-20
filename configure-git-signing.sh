#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# Configure Git Signing
# This script configures Git to use GPG for commit signing

# User details from custom instructions
GIT_USER_NAME="bo_thebig"
GIT_USER_EMAIL="botshelomokokoka@gmail.com"

echo -e "\e[32mConfiguring Git user details and commit signing...\e[0m"

# Check if git is installed
if ! command -v git &> /dev/null; then
    echo -e "\e[31mGit is not installed or not in PATH. Please install Git and try again.\e[0m"
    exit 1
fi

echo -e "\e[32mGit is installed: $(git --version)\e[0m"

# Check if GPG is installed
if ! command -v gpg &> /dev/null; then
    echo -e "\e[31mGPG is not installed or not in PATH. Please install GPG and try again.\e[0m"
    exit 1
fi

echo -e "\e[32mGPG is installed: $(gpg --version | head -n 1)\e[0m"

# Configure Git user information
echo -e "\e[33mSetting Git user name to: $GIT_USER_NAME\e[0m"
git config --global user.name "$GIT_USER_NAME"

echo -e "\e[33mSetting Git user email to: $GIT_USER_EMAIL\e[0m"
git config --global user.email "$GIT_USER_EMAIL"

# List available GPG keys
echo -e "\e[33mAvailable GPG keys:\e[0m"
gpg --list-secret-keys --keyid-format LONG

# Prompt for key selection
echo -e "\e[33mEnter the GPG key ID to use for signing (the 16-character ID after 'sec rsa4096/'):\e[0m"
read KEY_ID

# Configure Git to use the selected GPG key
if [ -n "$KEY_ID" ]; then
    echo -e "\e[33mSetting Git signing key to: $KEY_ID\e[0m"
    git config --global user.signingkey "$KEY_ID"
    
    # Enable commit signing by default
    echo -e "\e[33mEnabling commit signing by default\e[0m"
    git config --global commit.gpgsign true
    
    # Test the configuration
    echo -e "\e[33mTesting GPG configuration...\e[0m"
    TEST_RESULT=$(git config --global --get commit.gpgsign)
    if [ "$TEST_RESULT" = "true" ]; then
        echo -e "\e[32mGit is now configured to sign commits automatically!\e[0m"
    else
        echo -e "\e[31mConfiguration may have failed. Please check your settings.\e[0m"
    fi
    
    # Information about adding the key to GitHub/GitLab
    echo -e "\n\e[36mTo add this GPG key to GitHub/GitLab:\e[0m"
    echo -e "\e[36m1. Run: gpg --armor --export $KEY_ID\e[0m"
    echo -e "\e[36m2. Copy the output (including BEGIN and END lines)\e[0m"
    echo -e "\e[36m3. Paste it into your GitHub/GitLab settings under SSH and GPG keys\e[0m"
else
    echo -e "\e[31mNo key ID provided. Git signing configuration aborted.\e[0m"
fi

# Instructions for signing commits
echo -e "\n\e[36mTo sign commits:\e[0m"
echo -e "\e[36m- Commits will be automatically signed (commit.gpgsign=true)\e[0m"
echo -e "\e[36m- For manual signing: git commit -S -m 'your message'\e[0m"
echo -e "\e[36m- To verify a signed commit: git verify-commit <commit-hash>\e[0m"

echo -e "\n\e[32mConfiguration complete!\e[0m" 