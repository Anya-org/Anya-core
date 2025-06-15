#!/bin/bash
# Install git hooks for Anya Core
# Run this script to set up local development hooks

echo "🔗 Installing Git Hooks for Anya Core..."

# Create hooks directory if it doesn't exist
mkdir -p .git/hooks

# Copy pre-commit hook
if [[ -f "hooks/pre-commit" ]]; then
    cp hooks/pre-commit .git/hooks/pre-commit
    chmod +x .git/hooks/pre-commit
    echo "✅ Installed pre-commit hook"
else
    echo "❌ pre-commit hook file not found"
fi

# Create commit-msg hook
cat > .git/hooks/commit-msg << 'EOF'
#!/bin/bash
# Commit message validation for Anya Core

commit_file="$1"
commit_msg=$(cat "$commit_file")

# Check conventional commit format
commit_regex='^(feat|fix|docs|style|refactor|test|chore|perf|ci|build|revert)(\(.+\))?: .+'
emoji_regex='^(🔧|📊|✨|🐛|📝|💄|♻️|🧪|🔨|⚡|👷|🏗️|⏪)'

if ! echo "$commit_msg" | grep -qE "$commit_regex" && ! echo "$commit_msg" | grep -qE "$emoji_regex"; then
    echo "❌ Invalid commit message format!"
    echo ""
    echo "Use conventional commit format:"
    echo "  type(scope): description"
    echo ""
    echo "Types: feat, fix, docs, style, refactor, test, chore, perf, ci, build, revert"
    echo "Or use emojis: 🔧 ✨ 🐛 📝 💄 ♻️ 🧪 🔨 ⚡ 👷 🏗️ ⏪"
    echo ""
    echo "Examples:"
    echo "  feat(hsm): add new key generation algorithm"
    echo "  fix(bitcoin): resolve transaction signing issue"
    echo "  🔧 update dependencies and fix warnings"
    echo ""
    exit 1
fi

# Check commit message length
if [[ ${#commit_msg} -gt 72 ]]; then
    echo "⚠️ Commit message is longer than 72 characters"
    echo "Consider a shorter, more concise message."
fi

echo "✅ Commit message format is valid"
EOF

chmod +x .git/hooks/commit-msg
echo "✅ Installed commit-msg hook"

# Create pre-push hook
cat > .git/hooks/pre-push << 'EOF'
#!/bin/bash
# Pre-push validation for Anya Core

echo "🚀 Running pre-push validation..."

# Check if commits are signed
protected_branch="main"
current_branch=$(git branch --show-current)

# Only check signatures for main branch pushes
if [[ "$current_branch" == "$protected_branch" ]]; then
    echo "⚠️ Pushing to protected branch: $protected_branch"
    
    # Get commits that will be pushed
    commits=$(git rev-list @{u}..HEAD 2>/dev/null || git rev-list HEAD)
    
    for commit in $commits; do
        if ! git verify-commit "$commit" 2>/dev/null; then
            echo "❌ Commit $commit is not signed!"
            echo "Please sign your commits:"
            echo "  git rebase --exec 'git commit --amend --no-edit -S' @{u}"
            exit 1
        fi
    done
    
    echo "✅ All commits are properly signed"
fi

# Run quick tests for critical changes
echo "🧪 Running quick validation tests..."
if ! timeout 60s cargo check --quiet; then
    echo "❌ Code doesn't compile!"
    echo "Fix compilation errors before pushing."
    exit 1
fi

echo "✅ Pre-push validation passed!"
EOF

chmod +x .git/hooks/pre-push
echo "✅ Installed pre-push hook"

echo ""
echo "🎉 Git hooks installation complete!"
echo ""
echo "Installed hooks:"
echo "  - pre-commit: Checks formatting, linting, and secrets"
echo "  - commit-msg: Validates commit message format"
echo "  - pre-push: Ensures signed commits and compilation"
echo ""
echo "💡 To test the hooks:"
echo "  1. Make a change: echo '// test' >> src/lib.rs"
echo "  2. Stage it: git add src/lib.rs"
echo "  3. Try to commit: git commit -m 'test: testing hooks'"
echo ""
echo "🔐 Don't forget to set up GPG signing:"
echo "  ./scripts/setup-gpg-signing.sh"
