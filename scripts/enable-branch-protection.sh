#!/bin/bash
# Branch Protection Enable Script
# This script re-enables branch protection after administrative tasks
# Usage: ./scripts/enable-branch-protection.sh

set -e

echo "🛡️ Re-Enabling Branch Protection Rules"
echo "-------------------------------------"

# Get current repository information
if [ -z "$GITHUB_TOKEN" ]; then
    echo "❌ GITHUB_TOKEN environment variable is required"
    echo "Please set your Personal Access Token with 'repo' permissions:"
    echo "export GITHUB_TOKEN=your_token_here"
    exit 1
fi

# Extract repo info from git remote
REMOTE_URL=$(git remote get-url origin)
if [[ $REMOTE_URL =~ github\.com[:/]([^/]+)/([^/]+)(\.git)?$ ]]; then
    OWNER=${BASH_REMATCH[1]}
    REPO=${BASH_REMATCH[2]%.git}
else
    echo "❌ Could not determine repository owner and name from remote URL"
    exit 1
fi

echo "📁 Repository: $OWNER/$REPO"

# Calculate time branch protection was disabled
if [ -f /tmp/branch_protection_disabled_at ]; then
    DISABLED_AT=$(cat /tmp/branch_protection_disabled_at)
    NOW=$(date +%s)
    DURATION=$((NOW - DISABLED_AT))

    HOURS=$((DURATION / 3600))
    MINUTES=$(((DURATION % 3600) / 60))
    SECONDS=$((DURATION % 60))

    echo "⏱️ Branch protection was disabled for: ${HOURS}h ${MINUTES}m ${SECONDS}s"
    rm /tmp/branch_protection_disabled_at
fi

# Re-enable branch protection for main branch with recommended settings
echo "🔒 Re-enabling protection for 'main' branch..."
curl -s -X PUT \
    -H "Authorization: token $GITHUB_TOKEN" \
    -H "Accept: application/vnd.github.v3+json" \
    "https://api.github.com/repos/$OWNER/$REPO/branches/main/protection" \
    -d '{
    "required_status_checks": {
      "strict": true,
      "contexts": ["build", "test", "lint"]
    },
    "enforce_admins": true,
    "required_pull_request_reviews": {
      "dismiss_stale_reviews": true,
      "require_code_owner_reviews": true,
      "required_approving_review_count": 1
    },
    "restrictions": null,
    "required_linear_history": true,
    "allow_force_pushes": false,
    "allow_deletions": false,
    "required_conversation_resolution": true,
    "required_signatures": true
  }'

echo "✅ Branch protection for 'main' re-enabled with secure settings"
echo ""
echo "🛡️ Protection rules now in place:"
echo "  - Required status checks (build, test, lint)"
echo "  - Required pull request reviews (at least 1)"
echo "  - Required code owner reviews"
echo "  - Required signed commits"
echo "  - Protected from force push and deletion"
echo "  - Linear history required"
echo "  - Required conversation resolution"
echo ""

exit 0
