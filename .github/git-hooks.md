# Git Hooks for Anya Core
# These hooks enforce local development standards

## Pre-commit Hook
This hook runs before each commit to ensure code quality:

```bash
#!/bin/bash
# Save as .git/hooks/pre-commit and make executable

echo "🔍 Running pre-commit checks..."

# Check branch protection
if [[ "$(git branch --show-current)" == "main" ]]; then
    echo "❌ Direct commits to main branch are not allowed!"
    echo "Please create a feature branch and submit a PR."
    exit 1
fi

# Run cargo fmt check
echo "📝 Checking code formatting..."
if ! cargo fmt -- --check; then
    echo "❌ Code formatting issues found!"
    echo "Run 'cargo fmt' to fix them."
    exit 1
fi

# Run clippy
echo "🔍 Running clippy..."
if ! cargo clippy --all-targets --all-features -- -D warnings; then
    echo "❌ Clippy found issues!"
    exit 1
fi

# Check for secrets
echo "🔐 Scanning for secrets..."
if grep -r -E "(api_key|secret|password|token|private_key)" --include="*.rs" src/ | grep -v "test" | grep -v "example"; then
    echo "❌ Potential secrets found! Please remove them."
    exit 1
fi

echo "✅ Pre-commit checks passed!"
```

## Commit-msg Hook
This hook validates commit message format:

```bash
#!/bin/bash
# Save as .git/hooks/commit-msg and make executable

commit_regex='^(feat|fix|docs|style|refactor|test|chore|perf|ci|build|revert)(\(.+\))?: .+'
emoji_regex='^(🔧|📊|✨|🐛|📝|💄|♻️|🧪|🔨|⚡|👷|🏗️|⏪)'

if ! grep -qE "$commit_regex" "$1" && ! grep -qE "$emoji_regex" "$1"; then
    echo "❌ Invalid commit message format!"
    echo "Use: type(scope): description"
    echo "Types: feat, fix, docs, style, refactor, test, chore, perf, ci, build, revert"
    echo "Or emojis: 🔧 ✨ 🐛 📝 💄 ♻️ 🧪 🔨 ⚡ 👷 🏗️ ⏪"
    exit 1
fi
```

## Pre-push Hook
This hook runs before pushing to remote:

```bash
#!/bin/bash
# Save as .git/hooks/pre-push and make executable

echo "🚀 Running pre-push checks..."

# Ensure all commits are signed
commits=$(git rev-list @{u}..HEAD 2>/dev/null || git rev-list HEAD)
for commit in $commits; do
    if ! git verify-commit $commit 2>/dev/null; then
        echo "❌ Commit $commit is not signed!"
        echo "Please sign commits with 'git commit -S'"
        exit 1
    fi
done

# Run tests
echo "🧪 Running tests..."
if ! cargo test; then
    echo "❌ Tests failed!"
    exit 1
fi

echo "✅ Pre-push checks passed!"
```

## Installation
Run this script to install all hooks:

```bash
#!/bin/bash
# Install git hooks
cp hooks/pre-commit .git/hooks/
cp hooks/commit-msg .git/hooks/
cp hooks/pre-push .git/hooks/
chmod +x .git/hooks/*
echo "✅ Git hooks installed!"
```
