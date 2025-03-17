#!/usr/bin/env python3
"""
sync_labelling.py

Synchronizes docs/standards/AI_LABELING.md across all repositories in the Anya ecosystem.
Ensures consistent labeling standards across the entire project.

Usage: python sync_labelling.py [--source REPO] [--target REPOS] [--check-only] [--dry-run]
"""

import argparse
import os
import sys
import shutil
import subprocess
import json
import hashlib
from datetime import datetime
from pathlib import Path

# Default source repository (considered the "source of truth")
DEFAULT_SOURCE = "anya-core"

# Default list of all repositories
DEFAULT_REPOS = [
    "anya-core",
    "anya-web5",
    "anya-mobile",
    "anya-bitcoin",
    "dash33",
]

# Labels file name
LABELLING_FILE = "docs/standards/AI_LABELING.md"
COMMIT_RULES_FILE = "COMMIT_RULES.md"

# Directory for label history tracking
LABEL_HISTORY_DIR = ".label_history"

def parse_args():
    """Parse command line arguments."""
    parser = argparse.ArgumentParser(
        description="Synchronize docs/standards/AI_LABELING.md and related files across repositories"
    )
    parser.add_argument(
        "--source", 
        default=DEFAULT_SOURCE,
        help=f"Source repository for label standards (default: {DEFAULT_SOURCE})"
    )
    parser.add_argument(
        "--target", 
        default=",".join(DEFAULT_REPOS),
        help=f"Target repositories (comma-separated, default: {','.join(DEFAULT_REPOS)})"
    )
    parser.add_argument(
        "--check-only", 
        action="store_true",
        help="Only check for differences without making changes"
    )
    parser.add_argument(
        "--dry-run", 
        action="store_true",
        help="Show what would be done without making actual changes"
    )
    parser.add_argument(
        "--no-commit", 
        action="store_true",
        help="Do not commit changes after synchronization"
    )
    parser.add_argument(
        "--batch-commit",
        action="store_true",
        help="Use batch_commit.sh for committing changes"
    )
    return parser.parse_args()

def get_file_hash(file_path):
    """Get MD5 hash of file contents."""
    if not os.path.exists(file_path):
        return None
    
    with open(file_path, 'rb') as f:
        return hashlib.md5(f.read()).hexdigest()

def save_history(source_path, history_dir):
    """Save a copy of the current labelling file to history directory."""
    # Create history directory if it doesn't exist
    os.makedirs(history_dir, exist_ok=True)
    
    # Create timestamped filename
    timestamp = datetime.now().strftime("%Y%m%d_%H%M%S")
    history_file = os.path.join(history_dir, f"{LABELLING_FILE}_{timestamp}")
    
    # Save a copy
    shutil.copy2(source_path, history_file)
    print(f"Saved historical copy: {history_file}")
    
    # Keep only the last 10 historical files
    all_history_files = sorted([
        f for f in os.listdir(history_dir) 
        if f.startswith(LABELLING_FILE) and f != LABELLING_FILE
    ])
    
    if len(all_history_files) > 10:
        for old_file in all_history_files[:-10]:
            os.remove(os.path.join(history_dir, old_file))
            print(f"Removed old history file: {old_file}")

def sync_file(source_repo, target_repo, filename, dry_run=False, check_only=False):
    """Synchronize a specific file between repositories."""
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    source_path = os.path.join(base_dir, "..", source_repo, filename)
    target_path = os.path.join(base_dir, "..", target_repo, filename)
    
    # Convert paths to absolute
    source_path = os.path.abspath(source_path)
    target_path = os.path.abspath(target_path)
    
    # Check if source file exists
    if not os.path.exists(source_path):
        print(f"ERROR: Source file {source_path} does not exist")
        return False, False
    
    # Calculate file hashes
    source_hash = get_file_hash(source_path)
    target_hash = get_file_hash(target_path)
    
    # Skip if the files are identical
    if source_hash == target_hash and target_hash is not None:
        print(f"✓ {filename} in {target_repo} is already up to date")
        return False, True
    
    # Check if target file exists but with different content
    if target_hash is not None and source_hash != target_hash:
        print(f"! {filename} in {target_repo} differs from source")
        if check_only:
            return True, False
    
    # Copy the file if not in check-only mode
    if not check_only:
        if dry_run:
            print(f"WOULD COPY: {source_path} -> {target_path}")
        else:
            # Create target directory if needed
            os.makedirs(os.path.dirname(target_path), exist_ok=True)
            
            # Save historical copy
            history_dir = os.path.join(os.path.dirname(target_path), LABEL_HISTORY_DIR)
            if os.path.exists(target_path):
                save_history(target_path, history_dir)
            
            # Copy the file
            shutil.copy2(source_path, target_path)
            print(f"✓ Updated {filename} in {target_repo}")
            
        return True, True
    
    return True, False

def commit_changes(repo, changed_files, args):
    """Commit changes to the repository using either batch_commit.sh or direct git."""
    base_dir = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    repo_path = os.path.join(base_dir, "..", repo)
    repo_path = os.path.abspath(repo_path)
    
    # No changes to commit
    if not changed_files:
        return True
    
    # Don't commit if --no-commit was specified
    if args.no_commit:
        print(f"NOT COMMITTING: Changes in {repo} (--no-commit specified)")
        return True
    
    # Use batch_commit.sh if specified
    if args.batch_commit:
        batch_script = os.path.join(base_dir, "scripts", "batch_commit.sh")
        if not os.path.exists(batch_script):
            print(f"ERROR: batch_commit.sh not found at {batch_script}")
            return False
        
        if args.dry_run:
            print(f"WOULD BATCH COMMIT: Changes in {repo}")
            return True
        
        # Prepare the command
        cmd = [
            "bash", batch_script,
            "-m", "Synchronize AI labelling system across repositories",
            "-t", "docs",
            "-s", "labelling",
            "-l", "AIR-3,AIS-3,AIE-3",
            "-r", repo
        ]
        
        try:
            subprocess.run(cmd, check=True, cwd=base_dir)
            print(f"✓ Committed changes in {repo} using batch_commit.sh")
            return True
        except subprocess.CalledProcessError as e:
            print(f"ERROR: Failed to commit changes in {repo}: {e}")
            return False
    
    # Direct git commit
    if args.dry_run:
        print(f"WOULD COMMIT: Changes in {repo}")
        return True
    
    try:
        # Check if git is installed
        subprocess.run(["git", "--version"], check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        
        # Add changes
        subprocess.run(["git", "add"] + changed_files, check=True, cwd=repo_path)
        
        # Create commit message
        commit_msg = (
            "docs(labelling): synchronize AI labelling system\n\n"
            "Labels: [AIR-3][AIS-3][AIE-3]\n\n"
            "Ensure consistent labelling standards across all repositories."
        )
        
        # Commit changes
        subprocess.run(["git", "commit", "-m", commit_msg], check=True, cwd=repo_path)
        print(f"✓ Committed changes in {repo}")
        return True
    except subprocess.CalledProcessError as e:
        print(f"ERROR: Failed to commit changes in {repo}: {e}")
        return False

def main():
    """Main function."""
    args = parse_args()
    
    # Get list of target repositories
    target_repos = args.target.split(",")
    
    # Validation
    if args.source not in target_repos:
        print(f"WARNING: Source repository {args.source} not in target list. Adding it.")
        target_repos.append(args.source)
    
    print(f"Synchronizing labelling files from {args.source} to {len(target_repos)} repositories")
    print(f"Mode: {'Check only' if args.check_only else 'Synchronize'} {'(Dry run)' if args.dry_run else ''}")
    
    # Files to synchronize
    files_to_sync = [LABELLING_FILE, COMMIT_RULES_FILE]
    
    # Track repositories with changes
    changes_by_repo = {repo: [] for repo in target_repos}
    repos_with_diffs = []
    repos_synced = []
    
    # Process each target repository
    for repo in target_repos:
        if repo == args.source:
            continue  # Skip the source repository
        
        print(f"\nProcessing {repo}...")
        repo_has_changes = False
        
        # Synchronize each file
        for filename in files_to_sync:
            has_diff, synced = sync_file(
                args.source, repo, filename, 
                dry_run=args.dry_run, 
                check_only=args.check_only
            )
            
            if has_diff:
                changes_by_repo[repo].append(filename)
                repo_has_changes = True
            
            if synced and not args.check_only:
                repos_synced.append(repo)
        
        if repo_has_changes:
            repos_with_diffs.append(repo)
    
    # Commit changes if needed
    if not args.check_only:
        for repo, changed_files in changes_by_repo.items():
            if changed_files and repo != args.source:
                commit_changes(repo, changed_files, args)
    
    # Summary
    print("\nSummary:")
    print(f"- Repositories with differences: {len(repos_with_diffs)}")
    if repos_with_diffs:
        print(f"  - {', '.join(repos_with_diffs)}")
        
    if not args.check_only:
        print(f"- Repositories synchronized: {len(repos_synced)}")
        if repos_synced:
            print(f"  - {', '.join(set(repos_synced))}")
    
    # Exit with status code
    if args.check_only and repos_with_diffs:
        sys.exit(1)
    
    sys.exit(0)

if __name__ == "__main__":
    main() 