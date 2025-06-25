#!/bin/bash
# Script to clean up merged branches while preserving main repository
# This script will never remove protected branches

# Define protected branches as per repository rules
PROTECTED_BRANCHES="main develop feature/bitcoin-improvements feature/enhanced-dev-container"
echo "Starting branch cleanup process..."
echo "Protected branches that will NOT be removed: $PROTECTED_BRANCHES"

# Make sure we're on the main branch to safely clean up
git checkout main

echo -e "\n--- Step 1: Removing fully merged local branches ---"
echo "These branches have been fully merged into main and are safe to remove:"
# Build a grep pattern to exclude all protected branches
EXCLUDE_PATTERN=$(echo $PROTECTED_BRANCHES | sed 's/ /\\|/g')
MERGED_BRANCHES=$(git branch --merged main | grep -v '^\*' | grep -v "$EXCLUDE_PATTERN")
if [ -z "$MERGED_BRANCHES" ]; then
    echo "No fully merged branches found."
else
    echo "$MERGED_BRANCHES"
    # Ask for confirmation before deleting
    read -p "Do you want to delete these branches? (y/n): " confirm
    if [ "$confirm" = "y" ]; then
        echo "$MERGED_BRANCHES" | xargs git branch -d
        echo "Merged branches removed."
    else
        echo "Branch deletion cancelled."
    fi
fi

echo -e "\n--- Step 2: Pruning remote tracking branches ---"
echo "Removing references to remote branches that no longer exist:"
git fetch --prune
echo "Pruning complete."

echo -e "\n--- Step 3: Finding old branches ---"
echo "Branches not modified in the last 90 days:"
# List branches with last commit date
for branch in $(git branch | grep -v "main" | grep -v "develop" | sed 's/^[ *]*//'); do
    last_commit_date=$(git log -1 --format="%cr" $branch)
    echo "$branch - Last commit: $last_commit_date"
done

echo -e "\n--- Step 4: Branch status overview ---"
echo "Local branches:"
git branch

echo -e "\nRemote branches:"
git branch -r

echo -e "\n--- Step 5: Branch protection status ---"
echo "To ensure branch protection rules are enforced, review the GitHub repository settings."
echo "The main and develop branches should have the following protections:"
echo "  - Require pull request reviews before merging"
echo "  - Require status checks to pass before merging"
echo "  - Require signed commits"
echo "  - Do not allow bypassing the above settings"

echo -e "\nDone! Branch cleanup completed successfully."
echo "IMPORTANT: The main repository and critical branches have been preserved."
