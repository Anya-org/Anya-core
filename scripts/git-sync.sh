#!/bin/bash
# Git Sync and Branch Management Script
# This script provides utilities to sync branches and manage git workflows
# Usage: ./scripts/git-sync.sh [command]

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

# Help function
show_help() {
    echo -e "${BLUE}Anya Core Git Sync & Branch Management${NC}"
    echo -e "${PURPLE}=================================${NC}"
    echo
    echo -e "${YELLOW}Commands:${NC}"
    echo -e "  ${GREEN}sync${NC}              Sync current branch with main"
    echo -e "  ${GREEN}fresh${NC}             Create a fresh branch from updated main"
    echo -e "  ${GREEN}clean${NC}             Remove merged branches locally"
    echo -e "  ${GREEN}list${NC}              List all branches with their status"
    echo -e "  ${GREEN}conflicts${NC}         Check for potential merge conflicts"
    echo -e "  ${GREEN}status${NC}            Show overall repository status"
    echo -e "  ${GREEN}verify${NC}            Run pre-commit checks on current branch"
    echo -e "  ${GREEN}help${NC}              Show this help message"
    echo
    echo -e "${YELLOW}Examples:${NC}"
    echo -e "  ./scripts/git-sync.sh sync"
    echo -e "  ./scripts/git-sync.sh fresh feature/new-capability"
    echo
}

# Sync current branch with main
sync_branch() {
    echo -e "${BLUE}Syncing branch with main...${NC}"

    # Remember current branch
    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

    # Fetch latest changes
    echo -e "${YELLOW}Fetching latest changes...${NC}"
    git fetch origin

    if [ "$CURRENT_BRANCH" == "main" ]; then
        echo -e "${YELLOW}On main branch, pulling latest changes...${NC}"
        git pull origin main
    else
        echo -e "${YELLOW}Rebasing branch on top of main...${NC}"
        git rebase origin/main

        if [ $? -ne 0 ]; then
            echo -e "${RED}Rebase conflicts detected! Resolve manually and continue.${NC}"
            echo -e "${YELLOW}After resolving conflicts:${NC}"
            echo -e "  git add ."
            echo -e "  git rebase --continue"
            echo -e "${YELLOW}To abort the rebase:${NC}"
            echo -e "  git rebase --abort"
            exit 1
        fi
    fi

    echo -e "${GREEN}✓ Branch successfully synced with main${NC}"
}

# Create a fresh branch from updated main
create_fresh_branch() {
    if [ -z "$1" ]; then
        echo -e "${RED}Error: Branch name required${NC}"
        echo -e "${YELLOW}Usage: ./scripts/git-sync.sh fresh <branch-name>${NC}"
        exit 1
    fi

    NEW_BRANCH=$1

    echo -e "${BLUE}Creating fresh branch '$NEW_BRANCH' from updated main...${NC}"

    # Check if the branch already exists
    git show-ref --verify --quiet refs/heads/$NEW_BRANCH
    if [ $? -eq 0 ]; then
        echo -e "${RED}Error: Branch '$NEW_BRANCH' already exists${NC}"
        exit 1
    fi

    # Get latest main
    echo -e "${YELLOW}Updating main branch...${NC}"
    git checkout main
    git pull origin main

    # Create new branch
    echo -e "${YELLOW}Creating new branch '$NEW_BRANCH'...${NC}"
    git checkout -b $NEW_BRANCH

    echo -e "${GREEN}✓ Branch '$NEW_BRANCH' created successfully${NC}"
    echo -e "${YELLOW}You are now on branch '$NEW_BRANCH'${NC}"
}

# Clean up merged branches
clean_branches() {
    echo -e "${BLUE}Cleaning up merged branches...${NC}"

    # Switch to main to avoid deleting current branch
    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
    if [ "$CURRENT_BRANCH" != "main" ]; then
        git checkout main
    fi

    # Update main
    git pull origin main

    # Get list of merged branches
    MERGED_BRANCHES=$(git branch --merged | grep -v '^\*\|main\|dev\|release')

    if [ -z "$MERGED_BRANCHES" ]; then
        echo -e "${GREEN}✓ No merged branches to clean up${NC}"

        # Return to original branch
        if [ "$CURRENT_BRANCH" != "main" ]; then
            git checkout $CURRENT_BRANCH
        fi

        return
    fi

    echo -e "${YELLOW}The following merged branches will be deleted:${NC}"
    echo "$MERGED_BRANCHES"
    echo

    read -p "Continue with deletion? (y/N) " CONFIRM
    if [[ $CONFIRM =~ ^[Yy]$ ]]; then
        echo -e "${YELLOW}Deleting merged branches...${NC}"
        git branch --merged | grep -v '^\*\|main\|dev\|release' | xargs git branch -d
        echo -e "${GREEN}✓ Merged branches deleted${NC}"
    else
        echo -e "${YELLOW}Deletion cancelled${NC}"
    fi

    # Return to original branch
    if [ "$CURRENT_BRANCH" != "main" ]; then
        git checkout $CURRENT_BRANCH
    fi
}

# List all branches with status
list_branches() {
    echo -e "${BLUE}Listing branches with status...${NC}"

    # Fetch to get latest information
    git fetch --all >/dev/null 2>&1

    # Current branch
    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

    # List local branches with information
    echo -e "${YELLOW}Local branches:${NC}"
    for branch in $(git branch | sed 's/^..//' | sort); do
        # Check if branch exists on remote
        git show-ref --verify --quiet refs/remotes/origin/$branch
        if [ $? -eq 0 ]; then
            # Get ahead/behind status
            AHEAD_BEHIND=$(git rev-list --left-right --count origin/$branch...refs/heads/$branch)
            BEHIND=$(echo $AHEAD_BEHIND | cut -f1 -d' ')
            AHEAD=$(echo $AHEAD_BEHIND | cut -f2 -d' ')

            STATUS=""
            if [ "$BEHIND" -ne 0 ]; then
                STATUS="${RED}↓$BEHIND${NC}"
            fi
            if [ "$AHEAD" -ne 0 ]; then
                if [ -n "$STATUS" ]; then
                    STATUS="$STATUS ${GREEN}↑$AHEAD${NC}"
                else
                    STATUS="${GREEN}↑$AHEAD${NC}"
                fi
            fi
            if [ -z "$STATUS" ]; then
                STATUS="${GREEN}✓${NC}"
            fi

            # Check if branch is merged to main
            git merge-base --is-ancestor $branch main >/dev/null 2>&1
            MERGED=$?
            if [ $MERGED -eq 0 ]; then
                MERGED_STATUS="${PURPLE}[merged]${NC}"
            else
                MERGED_STATUS=""
            fi

            # Show last commit date
            LAST_COMMIT=$(git log -1 --format="%cr" $branch)

            if [ "$branch" == "$CURRENT_BRANCH" ]; then
                echo -e "${GREEN}* $branch${NC} $STATUS $MERGED_STATUS - $LAST_COMMIT"
            else
                echo -e "  $branch $STATUS $MERGED_STATUS - $LAST_COMMIT"
            fi
        else
            # Local only branch
            LAST_COMMIT=$(git log -1 --format="%cr" $branch)

            if [ "$branch" == "$CURRENT_BRANCH" ]; then
                echo -e "${GREEN}* $branch${NC} ${YELLOW}[local only]${NC} - $LAST_COMMIT"
            else
                echo -e "  $branch ${YELLOW}[local only]${NC} - $LAST_COMMIT"
            fi
        fi
    done
}

# Check for potential merge conflicts
check_conflicts() {
    echo -e "${BLUE}Checking for potential merge conflicts with main...${NC}"

    # Remember current branch
    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)

    # Update main
    git fetch origin main:main

    # Create temporary branch for testing
    TEMP_BRANCH="temp-merge-test-$(date +%s)"
    git checkout -b $TEMP_BRANCH >/dev/null 2>&1

    # Try to merge main into temporary branch
    echo -e "${YELLOW}Testing merge with main...${NC}"
    if git merge main --no-commit --no-ff >/dev/null 2>&1; then
        echo -e "${GREEN}✓ No conflicts detected!${NC}"
        git merge --abort >/dev/null 2>&1
    else
        echo -e "${RED}⚠ Potential conflicts detected!${NC}"
        echo -e "${YELLOW}Conflicting files:${NC}"
        git diff --name-only --diff-filter=U | sed 's/^/  /'
        git merge --abort >/dev/null 2>&1
    fi

    # Clean up
    git checkout $CURRENT_BRANCH >/dev/null 2>&1
    git branch -D $TEMP_BRANCH >/dev/null 2>&1
}

# Show repository status
show_status() {
    echo -e "${BLUE}Repository Status Summary${NC}"
    echo -e "${PURPLE}======================${NC}"

    # Current branch
    CURRENT_BRANCH=$(git rev-parse --abbrev-ref HEAD)
    echo -e "${YELLOW}Current branch:${NC} $CURRENT_BRANCH"

    # Last commit
    echo -e "${YELLOW}Last commit:${NC} $(git log -1 --pretty=format:'%h \"%s\" (%ar)')"

    # Branch status (ahead/behind)
    if git show-ref --verify --quiet refs/remotes/origin/$CURRENT_BRANCH; then
        AHEAD_BEHIND=$(git rev-list --left-right --count origin/$CURRENT_BRANCH...HEAD)
        BEHIND=$(echo $AHEAD_BEHIND | cut -f1 -d' ')
        AHEAD=$(echo $AHEAD_BEHIND | cut -f2 -d' ')

        if [ "$BEHIND" -ne 0 ]; then
            echo -e "${YELLOW}Status:${NC} ${RED}$BEHIND commits behind${NC} origin/$CURRENT_BRANCH"
        fi
        if [ "$AHEAD" -ne 0 ]; then
            echo -e "${YELLOW}Status:${NC} ${GREEN}$AHEAD commits ahead${NC} of origin/$CURRENT_BRANCH"
        fi
        if [ "$BEHIND" -eq 0 ] && [ "$AHEAD" -eq 0 ]; then
            echo -e "${YELLOW}Status:${NC} ${GREEN}up to date${NC} with origin/$CURRENT_BRANCH"
        fi
    else
        echo -e "${YELLOW}Status:${NC} ${PURPLE}local branch only${NC} (not on remote)"
    fi

    # Modified files
    MODIFIED=$(git status --porcelain | wc -l)
    if [ "$MODIFIED" -ne 0 ]; then
        echo -e "${YELLOW}Working tree:${NC} ${RED}$MODIFIED files modified${NC}"
    else
        echo -e "${YELLOW}Working tree:${NC} ${GREEN}clean${NC}"
    fi

    # Branch relation to main
    MAIN_AHEAD_BEHIND=$(git rev-list --left-right --count origin/main...HEAD 2>/dev/null)
    if [ $? -eq 0 ]; then
        MAIN_BEHIND=$(echo $MAIN_AHEAD_BEHIND | cut -f1 -d' ')
        MAIN_AHEAD=$(echo $MAIN_AHEAD_BEHIND | cut -f2 -d' ')

        if [ "$CURRENT_BRANCH" != "main" ]; then
            echo -e "${YELLOW}Relative to main:${NC} ${BLUE}$MAIN_AHEAD commits ahead${NC}, ${BLUE}$MAIN_BEHIND commits behind${NC}"
        fi
    fi

    # Show stashes if any
    STASH_COUNT=$(git stash list | wc -l)
    if [ "$STASH_COUNT" -ne 0 ]; then
        echo -e "${YELLOW}Stashes:${NC} $STASH_COUNT"
    fi

    echo
    echo -e "${YELLOW}Quick actions:${NC}"
    echo -e "  ${GREEN}Sync with main:${NC} ./scripts/git-sync.sh sync"
    echo -e "  ${GREEN}Check conflicts:${NC} ./scripts/git-sync.sh conflicts"
    echo -e "  ${GREEN}Verify before commit:${NC} ./scripts/git-sync.sh verify"
}

# Run pre-commit checks
verify_branch() {
    echo -e "${BLUE}Running pre-commit checks...${NC}"

    # Check for uncommitted changes
    if ! git diff-index --quiet HEAD --; then
        echo -e "${YELLOW}Uncommitted changes detected, stashing them temporarily...${NC}"
        git stash push -m "temp_stash_for_verification"
        STASHED=true
    else
        STASHED=false
    fi

    # Run Rust checks
    echo -e "${YELLOW}Running cargo check...${NC}"
    if cargo check; then
        echo -e "${GREEN}✓ Cargo check passed${NC}"
    else
        echo -e "${RED}✗ Cargo check failed${NC}"
        if [ "$STASHED" = true ]; then
            echo -e "${YELLOW}Restoring stashed changes...${NC}"
            git stash pop
        fi
        exit 1
    fi

    echo -e "${YELLOW}Running cargo clippy...${NC}"
    if cargo clippy -- -D warnings; then
        echo -e "${GREEN}✓ Clippy passed${NC}"
    else
        echo -e "${RED}✗ Clippy found issues${NC}"
        if [ "$STASHED" = true ]; then
            echo -e "${YELLOW}Restoring stashed changes...${NC}"
            git stash pop
        fi
        exit 1
    fi

    echo -e "${YELLOW}Running tests...${NC}"
    if cargo test; then
        echo -e "${GREEN}✓ Tests passed${NC}"
    else
        echo -e "${RED}✗ Tests failed${NC}"
        if [ "$STASHED" = true ]; then
            echo -e "${YELLOW}Restoring stashed changes...${NC}"
            git stash pop
        fi
        exit 1
    fi

    # Restore stashed changes if any
    if [ "$STASHED" = true ]; then
        echo -e "${YELLOW}Restoring stashed changes...${NC}"
        git stash pop
    fi

    echo -e "${GREEN}✓ All pre-commit checks passed!${NC}"
}

# Main script execution
case "$1" in
sync)
    sync_branch
    ;;
fresh)
    create_fresh_branch "$2"
    ;;
clean)
    clean_branches
    ;;
list)
    list_branches
    ;;
conflicts)
    check_conflicts
    ;;
status)
    show_status
    ;;
verify)
    verify_branch
    ;;
help | --help | -h)
    show_help
    ;;
*)
    echo -e "${RED}Unknown command: $1${NC}"
    show_help
    exit 1
    ;;
esac

exit 0
