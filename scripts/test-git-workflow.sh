#!/bin/bash
# Git Workflow Testing Script
# This script walks through a complete git workflow cycle to validate procedures
# Usage: ./scripts/test-git-workflow.sh

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
NC='\033[0m' # No Color

echo -e "${BLUE}Git Workflow Testing Procedure${NC}"
echo -e "${PURPLE}=========================${NC}"
echo

# Save current state for cleanup
ORIGINAL_BRANCH=$(git rev-parse --abbrev-ref HEAD)
echo -e "${YELLOW}Starting from branch:${NC} $ORIGINAL_BRANCH"
echo

# Run cargo test to ensure we're starting with a clean state
echo -e "${BLUE}Step 1: Verify starting state${NC}"
echo -e "${YELLOW}Running cargo test to ensure starting with a clean state...${NC}"
cargo test --lib
if [ $? -ne 0 ]; then
    echo -e "${RED}Initial tests failed. Fix issues before continuing with workflow test.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Starting state verified${NC}"
echo

# Step 2: Create a feature branch
echo -e "${BLUE}Step 2: Create a feature branch${NC}"
FEATURE_BRANCH="test-feature-$(date +%s)"
echo -e "${YELLOW}Creating branch:${NC} $FEATURE_BRANCH"
git checkout -b $FEATURE_BRANCH
echo -e "${GREEN}✓ Feature branch created${NC}"
echo

# Step 3: Make a change
echo -e "${BLUE}Step 3: Make a test change${NC}"
TEST_FILE="src/test_workflow_file.rs"
echo -e "${YELLOW}Creating test file:${NC} $TEST_FILE"
cat >$TEST_FILE <<EOF
//! Test file for git workflow
//! This file was automatically created by the workflow test script

/// A test function
pub fn test_workflow_function() -> &'static str {
    "This is a test function for git workflow"
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_workflow() {
        assert_eq!(test_workflow_function(), "This is a test function for git workflow");
    }
}
EOF
echo -e "${GREEN}✓ Test file created${NC}"
echo

# Step 4: Add the file to git
echo -e "${BLUE}Step 4: Add changes to git${NC}"
echo -e "${YELLOW}Adding file to git...${NC}"
git add $TEST_FILE
git status --short
echo -e "${GREEN}✓ Changes staged${NC}"
echo

# Step 5: Commit with conventional format
echo -e "${BLUE}Step 5: Commit with conventional format${NC}"
echo -e "${YELLOW}Creating commit...${NC}"
git commit -m "test: add workflow test function"
echo -e "${GREEN}✓ Changes committed with conventional format${NC}"
echo

# Step 6: Run tests again
echo -e "${BLUE}Step 6: Verify tests still pass${NC}"
echo -e "${YELLOW}Running cargo test...${NC}"
cargo test --lib
if [ $? -ne 0 ]; then
    echo -e "${RED}Tests failed after changes. Fix issues before continuing.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Tests pass after changes${NC}"
echo

# Step 7: Simulate code review by making another commit
echo -e "${BLUE}Step 7: Simulate code review feedback${NC}"
echo -e "${YELLOW}Making additional changes based on 'review feedback'...${NC}"
cat >>$TEST_FILE <<EOF

/// Another test function added based on review feedback
pub fn another_test_function() -> i32 {
    42
}

#[cfg(test)]
mod additional_tests {
    use super::*;
    
    #[test]
    fn test_another_function() {
        assert_eq!(another_test_function(), 42);
    }
}
EOF
git add $TEST_FILE
git commit -m "refactor: address review feedback for test function"
echo -e "${GREEN}✓ Review feedback addressed${NC}"
echo

# Step 8: Run tests again
echo -e "${BLUE}Step 8: Verify tests still pass after review changes${NC}"
echo -e "${YELLOW}Running cargo test...${NC}"
cargo test --lib
if [ $? -ne 0 ]; then
    echo -e "${RED}Tests failed after review changes. Fix issues before continuing.${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Tests pass after review changes${NC}"
echo

# Step 9: Simulate merge to main
echo -e "${BLUE}Step 9: Simulate merge to main${NC}"
echo -e "${YELLOW}Checking out main...${NC}"
git checkout main
echo -e "${YELLOW}Simulating merge of feature branch...${NC}"
git merge --no-ff $FEATURE_BRANCH -m "Merge test feature branch"
echo -e "${GREEN}✓ Feature branch merged to main${NC}"
echo

# Step 10: Verify main branch
echo -e "${BLUE}Step 10: Verify main branch integrity${NC}"
echo -e "${YELLOW}Running tests on main branch...${NC}"
cargo test --lib
if [ $? -ne 0 ]; then
    echo -e "${RED}Tests failed on main branch after merge. Something went wrong!${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Main branch integrity verified${NC}"
echo

# Step 11: Clean up
echo -e "${BLUE}Step 11: Clean up${NC}"
echo -e "${YELLOW}Removing test file...${NC}"
git rm $TEST_FILE
git commit -m "chore: remove test workflow file"
echo -e "${YELLOW}Removing feature branch...${NC}"
git branch -d $FEATURE_BRANCH
echo -e "${GREEN}✓ Clean up complete${NC}"
echo

# Step 12: Return to original branch
echo -e "${BLUE}Step 12: Returning to original branch${NC}"
echo -e "${YELLOW}Checking out ${ORIGINAL_BRANCH}...${NC}"
git checkout $ORIGINAL_BRANCH
echo -e "${GREEN}✓ Returned to ${ORIGINAL_BRANCH}${NC}"
echo

echo -e "${GREEN}✅ Git workflow test completed successfully${NC}"
echo
echo -e "${YELLOW}This test simulated:${NC}"
echo -e "  - Creating a feature branch"
echo -e "  - Making changes and committing with conventional format"
echo -e "  - Running tests to verify changes"
echo -e "  - Addressing code review feedback"
echo -e "  - Merging changes to main"
echo -e "  - Cleaning up"
echo
echo -e "${PURPLE}Note:${NC} This test used a local workflow. In a real scenario:"
echo -e "  - You would push the feature branch to the remote repository"
echo -e "  - Create a pull request"
echo -e "  - Have it reviewed by team members"
echo -e "  - Use the GitHub interface for the merge"
echo

exit 0
