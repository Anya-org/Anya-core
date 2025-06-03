#!/bin/bash
# docs-cleanup.sh - Automated documentation cleanup for Anya-core
# [AIR-3][AIS-3][BPC-3][RES-3]

set -e

echo "🧹 Starting Anya-core documentation cleanup..."
echo "📅 $(date)"
echo ""

# Navigate to docs directory
cd /home/bmokoka/Anya-core/docs

# Create backup
echo "📦 Creating backup..."
BACKUP_NAME="../docs-backup-$(date +%Y%m%d-%H%M%S).tar.gz"
tar -czf "$BACKUP_NAME" .
echo "✅ Backup created: $BACKUP_NAME"
echo ""

# Check current state
echo "📊 Current documentation state:"
echo "   Total files: $(find . -name "*.md" | wc -l)"
echo "   Total directories: $(find . -type d | wc -l)"
echo ""

# Remove confirmed duplicates
echo "🗑️  Removing duplicate files..."
DUPLICATES_REMOVED=0

if [ -f "architecture.md" ]; then
    rm -f architecture.md
    echo "   ✅ Removed architecture.md (duplicate of ARCHITECTURE.md)"
    ((DUPLICATES_REMOVED++))
fi

if [ -f "contributing.md" ]; then
    rm -f contributing.md  
    echo "   ✅ Removed contributing.md (duplicate of CONTRIBUTING.md)"
    ((DUPLICATES_REMOVED++))
fi

if [ -f "web5_integration.md" ]; then
    rm -f web5_integration.md
    echo "   ✅ Removed web5_integration.md (duplicate of WEB5_INTEGRATION.md)"
    ((DUPLICATES_REMOVED++))
fi

if [ -f "security.md" ]; then
    rm -f security.md
    echo "   ✅ Removed security.md (superseded by SECURITY_*.md files)"
    ((DUPLICATES_REMOVED++))
fi

# Remove backup files
BAK_FILES=$(find . -name "*.bak" | wc -l)
if [ "$BAK_FILES" -gt 0 ]; then
    find . -name "*.bak" -delete
    echo "   ✅ Removed $BAK_FILES backup files"
    DUPLICATES_REMOVED=$((DUPLICATES_REMOVED + BAK_FILES))
fi

echo "   📈 Total duplicates removed: $DUPLICATES_REMOVED"
echo ""

# Remove broken partial files
echo "🔧 Removing broken/partial files..."
BROKEN_REMOVED=0

# Find files with line number indicators (corrupted)
find . -name "*lines*" -type f > broken_files.tmp 2>/dev/null || true
if [ -s broken_files.tmp ]; then
    while read -r file; do
        rm -f "$file"
        echo "   ✅ Removed broken file: $file"
        ((BROKEN_REMOVED++))
    done < broken_files.tmp
fi
rm -f broken_files.tmp

echo "   📈 Total broken files removed: $BROKEN_REMOVED"
echo ""

# Verify AI labeling compliance
echo "🏷️  Checking AI labeling compliance..."
find . -name "*.md" -exec grep -L "\[AIR-3\]\[AIS-3\]\[BPC-3\]" {} \; > missing-labels.txt 2>/dev/null || true

MISSING_LABELS=$(wc -l < missing-labels.txt)
if [ "$MISSING_LABELS" -gt 0 ]; then
    echo "   ⚠️  $MISSING_LABELS files missing AI labels:"
    head -10 missing-labels.txt | sed 's/^/      /'
    if [ "$MISSING_LABELS" -gt 10 ]; then
        echo "      ... and $((MISSING_LABELS - 10)) more (see missing-labels.txt)"
    fi
else
    echo "   ✅ All files have proper AI labeling"
    rm -f missing-labels.txt
fi
echo ""

# Check for placeholder content
echo "📝 Checking for placeholder content..."
grep -r "Add a brief overview" . --include="*.md" > placeholders.txt 2>/dev/null || true

PLACEHOLDER_COUNT=$(wc -l < placeholders.txt)
if [ "$PLACEHOLDER_COUNT" -gt 0 ]; then
    echo "   ⚠️  $PLACEHOLDER_COUNT files with placeholder content:"
    head -5 placeholders.txt | sed 's/^/      /'
    if [ "$PLACEHOLDER_COUNT" -gt 5 ]; then
        echo "      ... and $((PLACEHOLDER_COUNT - 5)) more (see placeholders.txt)"
    fi
else
    echo "   ✅ No placeholder content found"
    rm -f placeholders.txt
fi
echo ""

# Check frontmatter compliance
echo "📋 Checking frontmatter compliance..."
MISSING_FRONTMATTER=0
for file in $(find . -name "*.md"); do
    if ! head -5 "$file" | grep -q "^---$"; then
        echo "   ⚠️  Missing frontmatter: $file" 
        ((MISSING_FRONTMATTER++))
    fi
done

if [ "$MISSING_FRONTMATTER" -eq 0 ]; then
    echo "   ✅ All files have proper frontmatter"
else
    echo "   ⚠️  $MISSING_FRONTMATTER files missing frontmatter"
fi
echo ""

# Generate summary report
echo "📊 CLEANUP SUMMARY:"
echo "   Files removed: $((DUPLICATES_REMOVED + BROKEN_REMOVED))"
echo "   Files needing AI labels: $MISSING_LABELS"
echo "   Files with placeholders: $PLACEHOLDER_COUNT"
echo "   Files missing frontmatter: $MISSING_FRONTMATTER"
echo ""

# Final state check
echo "📈 FINAL STATE:"
echo "   Total files: $(find . -name "*.md" | wc -l)"
echo "   Total directories: $(find . -type d | wc -l)"
echo ""

# Recommendations
echo "🎯 NEXT ACTIONS NEEDED:"

if [ -f "missing-labels.txt" ]; then
    echo "   1. Add AI labels to files listed in missing-labels.txt"
fi

if [ -f "placeholders.txt" ]; then
    echo "   2. Replace placeholder content in files listed in placeholders.txt"
fi

if [ "$MISSING_FRONTMATTER" -gt 0 ]; then
    echo "   3. Add frontmatter to files missing it"
fi

echo "   4. Review DOCUMENTATION_CLEANUP_PLAN.md for detailed next steps"
echo "   5. Review WORKSPACE_MANAGEMENT.md for ongoing maintenance"
echo ""

echo "✨ Cleanup complete!"
echo "📝 Generated files for review:"
ls -la *.txt 2>/dev/null || echo "   No review files generated - all clean!"
echo ""
echo "🎉 Documentation cleanup finished successfully!"
