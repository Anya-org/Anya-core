#!/bin/bash
# batch_commit.sh
#
# Batch commit script for Anya Core ecosystem
# Applies changes with proper labeling reference.
#
# Usage: ./batch_commit.sh -m "Commit message" -t "feat" -s "component" -l "AIR-3,AIS-2,AIT-3" [-r "repo1,repo2"] [-v]

set -e

# Default values
MESSAGE=""
TYPE="feat"
SCOPE=""
LABELS=""
REPOSITORIES=""
VALIDATE=false
DRY_RUN=false
BASE_DIR=$(pwd)
LABEL_CACHE_FILE="$BASE_DIR/.label_cache.json"

# Display help information
show_help() {
    echo "Batch Commit Tool with Comprehensive Labeling"
    echo "============================================="
    echo "Usage: ./batch_commit.sh [options]"
    echo ""
    echo "Options:"
    echo "  -m, --message MESSAGE       Commit message (required)"
    echo "  -t, --type TYPE             Commit type (default: feat)"
    echo "  -s, --scope SCOPE           Commit scope (optional)"
    echo "  -l, --labels LABELS         Comma-separated labels (required)"
    echo "  -r, --repos REPOSITORIES    Comma-separated repository list (default: all)"
    echo "  -v, --validate              Validate labels before committing"
    echo "  -d, --dry-run               Show what would be committed without making changes"
    echo "  -h, --help                  Show this help message"
    echo ""
    echo "Examples:"
    echo "  ./batch_commit.sh -m \"Update AI models\" -t \"feat\" -s \"ml\" -l \"AIR-3,AIS-2,AIT-3,AIM-2\""
    echo "  ./batch_commit.sh -m \"Fix security issues\" -t \"fix\" -s \"security\" -l \"AIR-3,AIS-3\" -r \"anya-core,anya-web5\" -v"
    echo ""
    echo "Available commit types:"
    echo "  feat, fix, docs, style, refactor, perf, test, build, ci, chore, revert"
    echo ""
    echo "See docs/standards/AI_LABELING.md for label requirements by component type"
}

# Parse command-line arguments
while [[ $# -gt 0 ]]; do
    case $1 in
        -m|--message)
            MESSAGE="$2"
            shift 2
            ;;
        -t|--type)
            TYPE="$2"
            shift 2
            ;;
        -s|--scope)
            SCOPE="$2"
            shift 2
            ;;
        -l|--labels)
            LABELS="$2"
            shift 2
            ;;
        -r|--repos)
            REPOSITORIES="$2"
            shift 2
            ;;
        -v|--validate)
            VALIDATE=true
            shift
            ;;
        -d|--dry-run)
            DRY_RUN=true
            shift
            ;;
        -h|--help)
            show_help
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            show_help
            exit 1
            ;;
    esac
done

# Validate required arguments
if [ -z "$MESSAGE" ]; then
    echo "Error: Commit message is required (-m)"
    show_help
    exit 1
fi

if [ -z "$LABELS" ]; then
    echo "Error: Labels are required (-l)"
    show_help
    exit 1
fi

# Validate commit type
VALID_TYPES=("feat" "fix" "docs" "style" "refactor" "perf" "test" "build" "ci" "chore" "revert")
if [[ ! " ${VALID_TYPES[*]} " =~ " ${TYPE} " ]]; then
    echo "Error: Invalid commit type: $TYPE"
    echo "Valid types: ${VALID_TYPES[*]}"
    exit 1
fi

# Format scope if provided
if [ -n "$SCOPE" ]; then
    SCOPE="($SCOPE)"
fi

# Format labels
# Convert comma-separated list to array
IFS=',' read -ra LABEL_ARRAY <<< "$LABELS"
FORMATTED_LABELS=""
for label in "${LABEL_ARRAY[@]}"; do
    # Trim whitespace
    label=$(echo $label | xargs)
    FORMATTED_LABELS+="[$label]"
done

# Get list of repositories
if [ -z "$REPOSITORIES" ]; then
    # Default list of repositories
    REPOSITORIES="anya-core,anya-web5,anya-mobile,anya-bitcoin,dash33"
fi
IFS=',' read -ra REPO_ARRAY <<< "$REPOSITORIES"

# Function to validate labels
validate_labels() {
    local component="$1"
    local labels="$2"
    
    # Load validation rules based on component
    case "$component" in
        bitcoin|btc|lightning|ln)
            required=("AIR" "AIS" "AIT" "BPC")
            recommended=("PFM" "SCL" "RES")
            ;;
        web5|dwn|did)
            required=("AIR" "AIS" "AIT" "W5C" "DID")
            recommended=("PFM" "SCL" "RES")
            ;;
        ml|ai|model)
            required=("AIR" "AIS" "AIT" "AIM" "AIP" "AIE")
            recommended=("PFM" "SCL" "RES")
            ;;
        ui|ux|frontend)
            required=("AIR" "UXA")
            recommended=("PFM" "AIP")
            ;;
        api|service)
            required=("AIR" "AIS" "AIP")
            recommended=("PFM" "SCL" "RES")
            ;;
        core|system)
            required=("AIR" "AIS" "AIT" "PFM" "RES" "SCL")
            recommended=()
            ;;
        dao|governance)
            required=("AIR" "AIS" "AIT" "DAO")
            recommended=("PFM" "RES" "SCL")
            ;;
        *)
            # Default requirements
            required=("AIR" "AIS")
            recommended=("AIT" "PFM")
            ;;
    esac
    
    # Check for required labels
    missing_required=()
    for req in "${required[@]}"; do
        if [[ ! " $labels " =~ " $req" ]]; then
            missing_required+=("$req")
        fi
    done
    
    # Check for recommended labels
    missing_recommended=()
    for rec in "${recommended[@]}"; do
        if [[ ! " $labels " =~ " $rec" ]]; then
            missing_recommended+=("$rec")
        fi
    done
    
    # Output validation results
    if [ ${#missing_required[@]} -gt 0 ]; then
        echo "Error: Missing required labels for $component: ${missing_required[*]}"
        return 1
    fi
    
    if [ ${#missing_recommended[@]} -gt 0 ]; then
        echo "Warning: Missing recommended labels for $component: ${missing_recommended[*]}"
    fi
    
    return 0
}

# Function to create full commit message
create_commit_message() {
    local message="$1"
    local type="$2"
    local scope="$3"
    local labels="$4"
    
    # Create conventional commit format
    echo "$type$scope: $message"
    echo ""
    echo "Labels: $labels"
}

# Main execution
echo "Batch Commit with Comprehensive Labeling"
echo "========================================"
echo "Commit Type: $TYPE"
if [ -n "$SCOPE" ]; then
    echo "Scope: $SCOPE"
fi
echo "Message: $MESSAGE"
echo "Labels: $FORMATTED_LABELS"
echo "Repositories: ${REPOSITORIES/,/, }"
echo ""

# Validate labels if requested
if [ "$VALIDATE" = true ]; then
    echo "Validating labels..."
    # Extract component from scope
    component=$(echo "$SCOPE" | sed -E 's/\(([^)]+)\)/\1/')
    validate_labels "$component" "$LABELS"
    validate_result=$?
    
    if [ $validate_result -ne 0 ]; then
        echo "Label validation failed. Use --help to see label requirements."
        exit 1
    fi
    
    echo "Label validation passed."
    echo ""
fi

# Generate commit message
COMMIT_MESSAGE=$(create_commit_message "$MESSAGE" "$TYPE" "$SCOPE" "$FORMATTED_LABELS")

# Display commit details
echo "Commit Message:"
echo "-----------------------------------"
echo "$COMMIT_MESSAGE"
echo "-----------------------------------"
echo ""

# Process each repository
for repo in "${REPO_ARRAY[@]}"; do
    repo_path="$BASE_DIR/../$repo"
    
    # Skip if repository doesn't exist
    if [ ! -d "$repo_path" ]; then
        echo "Warning: Repository $repo not found at $repo_path"
        continue
    fi
    
    echo "Processing repository: $repo"
    
    # Check if there are any changes to commit
    cd "$repo_path"
    git update-index -q --refresh
    if git diff-index --quiet HEAD --; then
        echo "No changes to commit in $repo"
        continue
    fi
    
    # Perform the commit
    if [ "$DRY_RUN" = true ]; then
        echo "DRY RUN: Would commit changes in $repo with message:"
        echo "$COMMIT_MESSAGE"
    else
        echo "Committing changes in $repo..."
        git add .
        echo "$COMMIT_MESSAGE" | git commit -F -
        echo "Changes committed successfully in $repo"
    fi
    
    echo ""
done

echo "Batch commit process completed."

# Return to original directory
cd "$BASE_DIR" 