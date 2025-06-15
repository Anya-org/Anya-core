#!/bin/bash
# Branch Protection Enforcement Script
# This script helps enforce branch protection rules locally

set -e

BRANCH_NAME=$(git branch --show-current)
PROTECTED_BRANCHES=("main" "master" "production" "release")

echo "🛡️ Branch Protection Enforcement Check"
echo "Current branch: $BRANCH_NAME"

# Check if we're on a protected branch
is_protected=false
for protected in "${PROTECTED_BRANCHES[@]}"; do
    if [[ "$BRANCH_NAME" == "$protected" ]]; then
        is_protected=true
        break
    fi
done

if [[ "$is_protected" == "true" ]]; then
    echo "⚠️ WARNING: You are on a protected branch ($BRANCH_NAME)"
    echo ""
    echo "❌ Direct commits to protected branches are not allowed!"
    echo ""
    echo "Please follow these steps:"
    echo "1. Create a feature branch: git checkout -b feature/your-feature-name"
    echo "2. Make your changes and commit them"
    echo "3. Push your branch: git push origin feature/your-feature-name"
    echo "4. Create a Pull Request on GitHub"
    echo ""
    echo "🔍 Available commands:"
    echo "  - Create feature branch: git checkout -b feature/$(date +%Y%m%d)-$(whoami)"
    echo "  - Create bugfix branch: git checkout -b bugfix/$(date +%Y%m%d)-$(whoami)"
    echo "  - Create hotfix branch: git checkout -b hotfix/$(date +%Y%m%d)-$(whoami)"
    echo ""
    exit 1
fi

echo "✅ Branch protection check passed"

# Check for uncommitted changes
if ! git diff-index --quiet HEAD --; then
    echo "📝 You have uncommitted changes"
    echo ""
    echo "Modified files:"
    git status --porcelain
    echo ""
    echo "Please commit your changes before pushing:"
    echo "  git add -A"
    echo "  git commit -S -m 'type(scope): description'"
    echo "  git push origin $BRANCH_NAME"
    echo ""
    echo "💡 Remember to sign your commits with -S flag for security!"
fi

# Check if branch is ahead of main
git fetch origin main --quiet
AHEAD=$(git rev-list --count origin/main..HEAD)
BEHIND=$(git rev-list --count HEAD..origin/main)

if [[ $AHEAD -gt 0 ]]; then
    echo "📤 Your branch is $AHEAD commits ahead of main"
fi

if [[ $BEHIND -gt 0 ]]; then
    echo "📥 Your branch is $BEHIND commits behind main"
    echo "💡 Consider rebasing: git rebase origin/main"
fi

echo "✅ Branch protection enforcement complete"
