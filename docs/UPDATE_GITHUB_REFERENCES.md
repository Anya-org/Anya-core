<!-- markdownlint-disable MD013 line-length -->

# GitHub URL Update Plan

## Overview

This document outlines the plan to update all GitHub repository references from `https://github.com/anya-org/anya-core` to `https://github.com/anya-org/anya-core` across the entire codebase.

## Implementation Strategy

### 1. Identify All References

First, identify all references to the old URL using grep:

```bash
grep -r "anya-org/anya-core" --include="*.{md,html,yaml,toml,json,sh,ps1,yml,d,rs,py,js,ts}" .
```

### 2. Update Scripts

We have created two scripts to automate the URL update process:

#### PowerShell Script (`scripts/update_github_url.ps1`)

```powershell
## GitHub URL Update Script for Windows
## This script updates all references to the old GitHub URL with the new URL

param(
    [string]$OldUrl = "github.com/anya-org/anya-core",
    [string]$NewUrl = "github.com/anya-org/anya-core",
    [string]$RootDir = ".",
    [switch]$DryRun
)

## File extensions to check
$Extensions = @(
    "*.md", "*.html", "*.yaml", "*.yml", "*.toml", "*.json", 
    "*.sh", "*.ps1", "*.rs", "*.py", "*.js", "*.ts", "*.lock",
    "*.d", "*.html", "*.txt", "*.css"
)

## Function to update URLs in files
function Update-URLs {
    param (
        [string]$FilePath
    )
    
    $content = Get-Content -Path $FilePath -Raw
    $originalContent = $content
    
    # Replace URLs (handling various formats)
    $content = $content -replace "https://github.com/anya-org/anya-core", "https://github.com/anya-org/anya-core"
    $content = $content -replace "https://github.com/anya-org/anya-core-main", "https://github.com/anya-org/anya-core-main"
    $content = $content -replace "github.com/anya-org/anya-core", "github.com/anya-org/anya-core"
    $content = $content -replace "github.com/anya-org/anya-core-main", "github.com/anya-org/anya-core-main"
    $content = $content -replace "anya-org/anya-core", "anya-org/anya-core"
    $content = $content -replace "anya-org/anya-core-main", "anya-org/anya-core-main"
    
    # Check if content was modified
    if ($content -ne $originalContent) {
        if (-not $DryRun) {
            Set-Content -Path $FilePath -Value $content
            Write-Host "Updated: $FilePath" -ForegroundColor Green
        } else {
            Write-Host "Would update: $FilePath" -ForegroundColor Yellow
        }
        return $true
    }
    
    return $false
}

## Main execution
Write-Host "Starting GitHub URL update process..."
$updateCount = 0

## Process each file extension
foreach ($ext in $Extensions) {
    $files = Get-ChildItem -Path $RootDir -Recurse -File -Filter $ext
    
    foreach ($file in $files) {
        if (Update-URLs -FilePath $file.FullName) {
            $updateCount++
        }
    }
}

## Update .git submodules
$gitmodulesPath = Join-Path $RootDir ".gitmodules"
if (Test-Path $gitmodulesPath) {
    if (Update-URLs -FilePath $gitmodulesPath) {
        $updateCount++
    }
}

## Summary
Write-Host "`nUpdate complete."
if ($DryRun) {
    Write-Host "DRY RUN: Would have updated $updateCount files." -ForegroundColor Yellow
} else {
    Write-Host "Updated $updateCount files." -ForegroundColor Green
}

## Next steps
Write-Host "`nNext steps:"
Write-Host "1. Run 'grep -r \"anya-org/anya-core\" .' to find any remaining references" -ForegroundColor Yellow
Write-Host "2. Commit changes: 'git commit -am \"Update GitHub URLs from botshelomokoka to anya-org\"'" -ForegroundColor Yellow
Write-Host "3. Run tests to ensure everything still works" -ForegroundColor Yellow
```

#### Bash Script (`scripts/update_github_url.sh`)

```bash
#!/bin/bash
## GitHub URL Update Script for Linux/macOS
## This script updates all references to the old GitHub URL with the new URL

OLD_URL="github.com/anya-org/anya-core"
NEW_URL="github.com/anya-org/anya-core"
ROOT_DIR="."
DRY_RUN=false

## Parse command line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --old-url) OLD_URL="$2"; shift ;;
        --new-url) NEW_URL="$2"; shift ;;
        --root-dir) ROOT_DIR="$2"; shift ;;
        --dry-run) DRY_RUN=true ;;
        *) echo "Unknown parameter: $1"; exit 1 ;;
    esac
    shift
done

## File extensions to check
EXTENSIONS=(
    "*.md" "*.html" "*.yaml" "*.yml" "*.toml" "*.json" 
    "*.sh" "*.ps1" "*.rs" "*.py" "*.js" "*.ts" "*.lock"
    "*.d" "*.html" "*.txt" "*.css"
)

## Function to update URLs in a file
update_urls() {
    local file="$1"
    local content
    local original_content
    
    content=$(cat "$file")
    original_content="$content"
    
    # Replace URLs (handling various formats)
    content=$(echo "$content" | sed "s|https://github.com/anya-org/anya-core|https://github.com/anya-org/anya-core|g")
    content=$(echo "$content" | sed "s|https://github.com/anya-org/anya-core-main|https://github.com/anya-org/anya-core-main|g")
    content=$(echo "$content" | sed "s|github.com/anya-org/anya-core|github.com/anya-org/anya-core|g")
    content=$(echo "$content" | sed "s|github.com/anya-org/anya-core-main|github.com/anya-org/anya-core-main|g")
    content=$(echo "$content" | sed "s|anya-org/anya-core|anya-org/anya-core|g")
    content=$(echo "$content" | sed "s|anya-org/anya-core-main|anya-org/anya-core-main|g")
    
    # Check if content was modified
    if [ "$content" != "$original_content" ]; then
        if [ "$DRY_RUN" = false ]; then
            echo "$content" > "$file"
            echo -e "\e[32mUpdated: $file\e[0m"
        else
            echo -e "\e[33mWould update: $file\e[0m"
        fi
        return 0
    fi
    
    return 1
}

## Main execution
echo "Starting GitHub URL update process..."
update_count=0

## Process each file extension
for ext in "${EXTENSIONS[@]}"; do
    while IFS= read -r file; do
        if update_urls "$file"; then
            ((update_count++))
        fi
    done < <(find "$ROOT_DIR" -type f -name "$ext" 2>/dev/null)
done

## Update .git submodules
if [ -f "$ROOT_DIR/.gitmodules" ]; then
    if update_urls "$ROOT_DIR/.gitmodules"; then
        ((update_count++))
    fi
fi

## Summary
echo -e "\nUpdate complete."
if [ "$DRY_RUN" = true ]; then
    echo -e "\e[33mDRY RUN: Would have updated $update_count files.\e[0m"
else
    echo -e "\e[32mUpdated $update_count files.\e[0m"
fi

## Next steps
echo -e "\nNext steps:"
echo -e "\e[33m1. Run 'grep -r \"anya-org/anya-core\" .' to find any remaining references\e[0m"
echo -e "\e[33m2. Commit changes: 'git commit -am \"Update GitHub URLs from botshelomokoka to anya-org\"'\e[0m"
echo -e "\e[33m3. Run tests to ensure everything still works\e[0m"
```

### 3. Manual Update Plan

For critical files that need careful attention:

- [ ] API.md documentation references
- [ ] README.md and other primary documentation
- [ ] Package metadata files (package.json, Cargo.toml, pubspec.yaml)
- [ ] CI/CD configuration files in .github directory
- [ ] Git submodule references in .gitmodules
- [ ] Documentation files in /docs directory

### 4. Implementation Steps

1. **Create a new branch**:

   ```bash
   git checkout -b update-github-urls
   ```

2. **Run the URL update script**:
   - For Windows:

     ```powershell
     ./scripts/update_github_url.ps1 -DryRun
     ```

   - For Linux/macOS:

     ```bash
     ./scripts/update_github_url.sh --dry-run
     ```

3. **Review the changes before applying**:
   - Run without the dry-run flag to apply the changes:

     ```powershell
     ./scripts/update_github_url.ps1
     ```

     or

     ```bash
     ./scripts/update_github_url.sh
     ```

4. **Verify remaining references**:

   ```bash
   grep -r "anya-org/anya-core" --include="*.{md,html,yaml,toml,json,sh,ps1,yml,d,rs,py,js,ts}" .
   ```

5. **Make any necessary manual changes** for complex cases.

6. **Run tests to validate** that all functionality still works.

7. **Commit and create a pull request**:

   ```bash
   git add .
   git commit -m "Update GitHub URLs from botshelomokoka to anya-org"
   git push origin update-github-urls
   ```

### 5. Post-Update Verification

- [ ] Verify all links work in documentation
- [ ] Check CI/CD pipelines run successfully
- [ ] Ensure git submodules can be cloned and updated
- [ ] Test building and running the application
- [ ] Check all badge URLs correctly point to the new repository

## Affected Files

Types of files that need to be updated:

1. Documentation files (*.md,*.rst, *.html)
2. Configuration files (*.yml,*.yaml, *.toml,*.json)
3. Script files (*.sh,*.ps1)
4. Source code comments and metadata
5. Package manager files (package.json, Cargo.toml, pubspec.yaml)
6. Git configuration files (.gitmodules)

## Timeline

1. Preparation and scripting: 1 day
2. Execution of update scripts: 1 day
3. Manual validation and fixes: 1-2 days
4. Testing and verification: 1 day
5. Pull request review and approval: 1-2 days

## Risks and Mitigations

| Risk | Mitigation |
|------|------------|
| Breaking CI/CD pipelines | Run CI in fork before merging |
| Missing some references | Use multiple grep patterns and manual verification |
| Breaking submodule references | Test submodule operations after update |
| Code references in comments | Review code for references in string literals or comments |

## Conclusion

This comprehensive plan ensures a smooth transition from the old GitHub URL to the new one across the entire codebase.
