#!/bin/bash
# Script to clean up merged branches both locally and remotely

# Make sure we're on the main branch
git checkout main

# Fetch latest changes and prune remote branches
echo "Fetching latest changes and pruning stale remote-tracking branches..."
git fetch --all --prune

# List branches that have been merged into main
echo -e "\nLocal branches merged into main:"
git branch --merged main

# Ask confirmation before deleting local merged branches
read -p "Do you want to delete all local merged branches except main? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Delete all merged branches except main
    git branch --merged main | grep -v "main" | xargs -r git branch -d
    echo "Merged local branches deleted."
else
    echo "Skipping deletion of local merged branches."
fi

# Handle unmerged branches
echo -e "\nUnmerged local branches:"
git branch --no-merged main

# Ask confirmation before force deleting local unmerged branches
read -p "Do you want to force delete all local unmerged branches except current? (CAUTION: y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Delete all unmerged branches except current
    git branch --no-merged main | grep -v "^\*" | xargs -r git branch -D
    echo "Unmerged local branches force deleted."
else
    echo "Skipping deletion of unmerged branches."
fi

# List remote branches that are merged into main
echo -e "\nRemote branches merged into origin/main:"
git branch -r --merged origin/main | grep -v "origin/main" | grep -v "origin/HEAD"

# Ask confirmation before deleting remote merged branches
read -p "Do you want to delete all remote merged branches except main? (y/n) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    # Delete all merged remote branches except main
    git branch -r --merged origin/main | grep -v "origin/main" | grep -v "origin/HEAD" | sed 's/origin\///' | xargs -r -I{} git push origin --delete {}
    echo "Merged remote branches deleted."
else
    echo "Skipping deletion of remote merged branches."
fi

# Clean up references that are no longer needed
echo -e "\nCleaning up git references..."
git gc --prune=now

echo -e "\nBranch cleanup complete!"
