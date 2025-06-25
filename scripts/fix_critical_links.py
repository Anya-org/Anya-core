#!/usr/bin/env python3
"""
Critical Link Fixer - Targeted documentation link fixer
Created: June 17, 2025
Purpose: Fix critical broken links in installation and setup documentation
"""

import json
import os
import re
import sys
from typing import Dict

# Paths
ROOT_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
CRITICAL_FILES = [
    os.path.join(ROOT_DIR, "docs/installation/README.md"),
    os.path.join(ROOT_DIR, "ROOT_INDEX.md"),
    os.path.join(ROOT_DIR, "docs/SYSTEM_MAP.md"),
    os.path.join(ROOT_DIR, "DOCUMENTATION_CLEANUP_PLAN.md"),
    os.path.join(ROOT_DIR, "DOCUMENTATION_CLEANUP_SUMMARY.md"),
    os.path.join(ROOT_DIR, "DOCUMENTATION_TRUTH_REVIEW.md"),
    os.path.join(ROOT_DIR, "DOCUMENTATION_LINK_CAMPAIGN_CONSOLIDATED.md"),
    os.path.join(ROOT_DIR, "MISSING_DOCUMENTATION_REFERENCE.md"),
    os.path.join(ROOT_DIR, "INSTALLATION.md"),
    os.path.join(ROOT_DIR, "CONTRIBUTING.md"),
    os.path.join(ROOT_DIR, "README.md"),
    os.path.join(ROOT_DIR, "docs/README.md")
]

# Manual mappings - Read from link_mappings.json
critical_link_fixes: Dict[str, str] = {}

try:
    with open(os.path.join(os.path.dirname(__file__), "link_mappings.json"), "r") as f:
        critical_link_fixes = json.load(f)
    print(
        f"Loaded {len(critical_link_fixes)} link mappings from link_mappings.json")
except Exception as e:
    print(f"Error loading link_mappings.json: {e}")
    critical_link_fixes = {
        "troubleshooting.md": "../TROUBLESHOOTING.md",
        "./troubleshooting.md": "troubleshooting.md",
        "./related1.md": "../INSTALLATION.md",
        "./related2.md": "../INSTALLATION_REVIEW.md"
    }
    print(f"Falling back to {len(critical_link_fixes)} default mappings")

# Regular expression pattern for markdown links
LINK_PATTERN = re.compile(r'\[([^\]]+)\]\(([^)]+)\)')


def fix_file(file_path: str) -> int:
    """Fix links in a specific file"""
    if not os.path.exists(file_path):
        print(f"File not found: {file_path}")
        return 0

    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()

    original_content = content
    fixed_count = 0

    # Find all markdown links
    links = LINK_PATTERN.findall(content)
    for link_text, link_path in links:
        # Skip external links
        if link_path.startswith(('http://', 'https://')):
            continue

        # Check if this is a known broken link we need to fix
        if link_path in critical_link_fixes:
            new_link = critical_link_fixes[link_path]
            old_link = f"[{link_text}]({link_path})"
            new_link_text = f"[{link_text}]({new_link})"
            content = content.replace(old_link, new_link_text)
            fixed_count += 1
            print(f"  Fixed: {old_link} -> {new_link_text}")

    # Write back if changes were made
    if content != original_content:
        with open(file_path, 'w', encoding='utf-8') as f:
            f.write(content)
        print(f"✓ Fixed {fixed_count} links in {os.path.basename(file_path)}")
    else:
        print(f"✓ No critical links to fix in {os.path.basename(file_path)}")

    return fixed_count


def main() -> int:
    total_fixed = 0

    print("Critical Link Fixer - Starting")
    print("--------------------------------")

    # Fix each critical file
    for file_path in CRITICAL_FILES:
        print(f"\nChecking {os.path.basename(file_path)}...")
        fixed = fix_file(file_path)
        total_fixed += fixed

    print("\n--------------------------------")
    print(f"Total links fixed: {total_fixed}")
    print("Critical Link Fixer - Complete")

    return 0  # Always return success, don't fail if no links need fixing


if __name__ == "__main__":
    sys.exit(main())
