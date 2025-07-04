#!/usr/bin/env bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# Sign Previous Commits Script
# This script helps retroactively sign git commits with your GPG key

set -e

# Default values
COMMIT_COUNT=10
BRANCH_NAME=""
DRY_RUN=false

# Function to display script usage
show_usage() {
    echo -e "\e[36mSign Previous Commits Script\e[0m"
    echo -e "\e[36mThis script helps retroactively sign git commits with your GPG key\e[0m"
    echo ""
    echo -e "\e[33mUsage:\e[0m"
    echo -e "\e[33m  ./sign-previous-commits.sh [-c COUNT] [-b BRANCH] [-d]\e[0m"
    echo ""
    echo -e "\e[33mParameters:\e[0m"
    echo -e "\e[33m  -c COUNT    Number of commits to examine (default: 10)\e[0m"
    echo -e "\e[33m  -b BRANCH   Branch to rebase (default: current branch)\e[0m"
    echo -e "\e[33m  -d          Dry run - show what would be done without making changes\e[0m"
    echo -e "\e[33m  -h          Show this help message\e[0m"
    echo ""
    echo -e "\e[32mExample:\e[0m"
    echo -e "\e[32m  ./sign-previous-commits.sh -c 5\e[0m"
    echo -e "\e[32m  ./sign-previous-commits.sh -b feature/my-branch -d\e[0m"
}

# Parse command line arguments
while getopts "c:b:dh" opt; do
    case $opt in
    c) COMMIT_COUNT=$OPTARG ;;
    b) BRANCH_NAME=$OPTARG ;;
    d) DRY_RUN=true ;;
    h)
        show_usage
        exit 0
        ;;
    *)
        show_usage
        exit 1
        ;;
    esac
done

# Function to check if Git is installed
check_git_installed() {
    if ! command -v git &>/dev/null; then
        echo -e "\e[31mGit is not installed or not in PATH.\e[0m"
        return 1
    fi
    return 0
}

# Function to check if we're in a git repository
check_git_repository() {
    if ! git rev-parse --is-inside-work-tree &>/dev/null; then
        echo -e "\e[31mCurrent directory is not a Git repository.\e[0m"
        return 1
    fi
    return 0
}

# Function to check if GPG signing is configured
check_git_signing_configured() {
    local signing_key
    local gpg_sign

    signing_key=$(git config --get user.signingkey 2>/dev/null || echo "")
    gpg_sign=$(git config --get commit.gpgsign 2>/dev/null || echo "")

    if [[ -z "$signing_key" || "$gpg_sign" != "true" ]]; then
        echo -e "\e[31mGit GPG signing is not properly configured.\e[0m"
        echo -e "\e[33mPlease run ./configure-git-signing.sh first.\e[0m"
        return 1
    fi
    return 0
}

# Function to get the current branch name
get_current_branch() {
    git rev-parse --abbrev-ref HEAD
}

# Function to get commits that need signing
get_unsigned_commits() {
    local count=$1
    local commits=()
    local git_log

    git_log=$(git log --format="%h %G?" -n "$count")

    while IFS= read -r line; do
        local hash
        local sign_status

        hash=$(echo "$line" | cut -d' ' -f1)
        sign_status=$(echo "$line" | cut -d' ' -f2)

        # N = no signature
        # B = bad signature
        # U = unknown signature
        if [[ "$sign_status" == "N" || "$sign_status" == "B" || "$sign_status" == "U" ]]; then
            commits+=("$hash")
        fi
    done <<<"$git_log"

    echo "${commits[@]}"
}

# Main script execution
if ! check_git_installed; then
    exit 1
fi

if ! check_git_repository; then
    exit 1
fi

if ! check_git_signing_configured; then
    exit 1
fi

# Determine the branch to work with
if [[ -z "$BRANCH_NAME" ]]; then
    BRANCH_NAME=$(get_current_branch)
fi

# Get unsigned commits
unsigned_commits=($(get_unsigned_commits "$COMMIT_COUNT"))
unsigned_count=${#unsigned_commits[@]}

if [[ $unsigned_count -eq 0 ]]; then
    echo -e "\e[32mNo unsigned commits found in the last $COMMIT_COUNT commits.\e[0m"
    exit 0
fi

# Display unsigned commits
echo -e "\e[33mFound $unsigned_count unsigned commits in the last $COMMIT_COUNT commits:\e[0m"
for commit_hash in "${unsigned_commits[@]}"; do
    commit_info=$(git log --format="%h %G? %an %s" -n 1 "$commit_hash")
    echo -e "\e[37m$commit_info\e[0m"
done

# Confirm action
if [[ "$DRY_RUN" != "true" ]]; then
    echo ""
    echo -e "\e[31mWARNING: This will rewrite Git history by adding signatures to these commits.\e[0m"
    echo -e "\e[31mIf you've already pushed these commits, you'll need to force push after signing.\e[0m"
    echo -e "\e[31mThis can cause problems for other contributors if they've based work on these commits.\e[0m"
    echo ""
    read -p "Do you want to proceed? (y/N): " confirmation

    if [[ "$confirmation" != "y" && "$confirmation" != "Y" ]]; then
        echo -e "\e[33mOperation cancelled.\e[0m"
        exit 0
    fi

    # Get the oldest commit hash to start rebasing from
    oldest_commit="${unsigned_commits[$((unsigned_count - 1))]}"

    echo -e "\e[36mStarting interactive rebase to sign commits...\e[0m"
    echo -e "\e[36mIf Git opens an editor, save and close it to continue.\e[0m"

    # Execute the rebase
    if git rebase -i "${oldest_commit}~1" --exec "git commit --amend --no-edit -S"; then
        echo -e "\e[32mSuccessfully signed commits!\e[0m"
        echo -e "\e[33mIf these commits were already pushed, you'll need to force push:\e[0m"
        echo -e "\e[33m  git push --force origin $BRANCH_NAME\e[0m"
    else
        echo -e "\e[31mRebase failed or was aborted.\e[0m"
        echo -e "\e[33mYou may need to run 'git rebase --abort' to cleanup.\e[0m"
    fi
else
    # Dry run mode
    echo ""
    echo -e "\e[36mDRY RUN: The following actions would be taken:\e[0m"
    oldest_commit="${unsigned_commits[$((unsigned_count - 1))]}"
    echo -e "\e[37m - Interactive rebase would be started from commit ${oldest_commit}~1\e[0m"
    echo -e "\e[37m - Each commit would be amended with your GPG signature\e[0m"
    echo -e "\e[37m - You would need to force push after completion\e[0m"
    echo ""
    echo -e "\e[33mTo execute these actions, run without the -d parameter.\e[0m"
fi
