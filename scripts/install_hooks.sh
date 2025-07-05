#!/bin/bash

# ========================================================================
# ANYA CORE PRE-COMMIT HOOK INSTALLER
# ========================================================================
# Purpose: Install mandatory quality gate as pre-commit hook for all developers
# Usage: ./scripts/install_hooks.sh
# ========================================================================

set -e

echo "🔧 Installing Anya Core Quality Gate Pre-Commit Hook"
echo "===================================================="

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "❌ Error: Not in a git repository"
    exit 1
fi

# Create .git/hooks directory if it doesn't exist
mkdir -p .git/hooks

# Create the pre-commit hook
cat >.git/hooks/pre-commit <<'EOF'
#!/bin/bash

# ========================================================================
# ANYA CORE MANDATORY PRE-COMMIT HOOK
# ========================================================================
# This hook enforces all repository rules and quality standards
# DO NOT MODIFY OR BYPASS - Required for all developers
# ========================================================================

echo "🔍 Running Anya Core Quality Gate..."

# Run the quality gate script
if ! ./scripts/quality_gate.sh --pre-commit; then
    echo ""
    echo "❌ COMMIT REJECTED BY QUALITY GATE"
    echo "Please fix the issues above and try again."
    echo ""
    echo "For help:"
    echo "  • Review COMMIT_RULES.md for commit message format"
    echo "  • Run './scripts/quality_gate.sh --full' for detailed analysis"
    echo "  • Check IMPLEMENTATION_STATUS_VERIFIED_REALITY.md for current status"
    exit 1
fi

echo "✅ Quality gate passed - proceeding with commit"
EOF

# Make the hook executable
chmod +x .git/hooks/pre-commit

echo "✅ Pre-commit hook installed successfully"
echo ""
echo "📋 What this hook enforces:"
echo "  • Conventional Commits format with required labels"
echo "  • Zero unimplemented!() macros allowed"
echo "  • Compilation must succeed"
echo "  • Maximum 10 compilation warnings"
echo "  • No hardcoded secrets or aspirational claims"
echo "  • Evidence-based documentation updates"
echo ""
echo "🚀 Ready! All commits will now be validated automatically."
echo ""
echo "To test the hook:"
echo "  git add . && git commit -m 'test: validate quality gate hook'"
echo ""
echo "To run quality gate manually:"
echo "  ./scripts/quality_gate.sh --full"
