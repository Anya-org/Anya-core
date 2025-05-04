#!/bin/bash
# GitHub URL Update Script (Bash version)
# This script updates all references to the old GitHub repository URL

# Default values
OLD_URL="github.com/anya-org/anya-core"
NEW_URL="github.com/anya-org/anya-core"
DRY_RUN=false
ROOT_DIR="."

# Parse command line arguments
while [[ $# -gt 0 ]]; do
  case $1 in
    --old-url)
      OLD_URL="$2"
      shift 2
      ;;
    --new-url)
      NEW_URL="$2"
      shift 2
      ;;
    --dry-run)
      DRY_RUN=true
      shift
      ;;
    --root-dir)
      ROOT_DIR="$2"
      shift 2
      ;;
    *)
      echo "Unknown option: $1"
      exit 1
      ;;
  esac
done

# Log file
LOG_FILE="$ROOT_DIR/github_url_update_log.txt"

# Function to update files
update_files() {
  local pattern="$1"
  local file_pattern="$2"
  
  echo "Updating files matching: $file_pattern"
  
  # Find all matching files
  find "$ROOT_DIR" -type f -name "$file_pattern" | while read -r file; do
    # Check if file contains the pattern
    if grep -q "$pattern" "$file"; then
      echo "Updating: $file"
      
      if [ "$DRY_RUN" = false ]; then
        # Replace pattern in file
        sed -i "s|$pattern|$NEW_URL|g" "$file"
        
        # Log updated file
        echo "Updated: $file" >> "$LOG_FILE"
      else
        echo "  (Dry run - no changes made)"
      fi
    fi
  done
}

# Function to check specific files
check_specific_files() {
  local specific_files=(
    "pubspec.yaml"
    "package.json"
    "Cargo.toml"
    "docs/_config.yml"
    "book.toml"
    ".gitmodules"
    "docs/CONTRIBUTING.md"
    ".github/workflows/sync-enterprise.yml"
  )
  
  for file in "${specific_files[@]}"; do
    file_path="$ROOT_DIR/$file"
    
    if [ -f "$file_path" ]; then
      if grep -q "$OLD_URL" "$file_path"; then
        echo -e "\e[33mCritical file needs update: $file\e[0m"
      else
        echo -e "\e[32mCritical file OK: $file\e[0m"
      fi
    else
      echo -e "\e[31mCritical file not found: $file\e[0m"
    fi
  done
}

# Main execution
echo -e "\e[36mStarting GitHub URL update process...\e[0m"

# Create log file
if [ "$DRY_RUN" = false ]; then
  echo "GitHub URL Update Log - $(date)" > "$LOG_FILE"
  echo "" >> "$LOG_FILE"
fi

# Update various file types
echo -e "\e[36mUpdating Markdown files...\e[0m"
update_files "$OLD_URL" "*.md"

echo -e "\e[36mUpdating HTML files...\e[0m"
update_files "$OLD_URL" "*.html"

echo -e "\e[36mUpdating YAML files...\e[0m"
update_files "$OLD_URL" "*.yaml"
update_files "$OLD_URL" "*.yml"

echo -e "\e[36mUpdating TOML files...\e[0m"
update_files "$OLD_URL" "*.toml"

echo -e "\e[36mUpdating JSON files...\e[0m"
update_files "$OLD_URL" "*.json"

echo -e "\e[36mUpdating Shell scripts...\e[0m"
update_files "$OLD_URL" "*.sh"

echo -e "\e[36mUpdating PowerShell scripts...\e[0m"
update_files "$OLD_URL" "*.ps1"

# Verify critical files
echo -e "\n\e[36mVerifying critical files...\e[0m"
check_specific_files

# Reminder for manual verification
echo -e "\n\e[33mRemember to manually verify the following:\e[0m"
echo -e "\e[33m1. Run 'grep -r \"anya-org/anya-core\" .' to find any remaining references\e[0m"
echo -e "\e[33m2. Check if CI/CD pipelines still function correctly\e[0m"
echo -e "\e[33m3. Test local builds with updated configurations\e[0m"
echo -e "\e[33m4. Verify all documentation links resolve correctly\e[0m"

echo -e "\n\e[32mGitHub URL update process complete.\e[0m"

if [ "$DRY_RUN" = true ]; then
  echo -e "\e[36mThis was a dry run - no actual changes were made.\e[0m"
else
  echo -e "\e[36mSee github_url_update_log.txt for a list of all updated files.\e[0m"
fi 
