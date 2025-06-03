#!/usr/bin/env python3
"""
Documentation Structure Validator for Anya Core

This script validates that all files referenced in the documentation exist
and that the documentation structure is consistent.
"""

import os
import re
import sys
from pathlib import Path
from typing import List, Set, Dict, Tuple

# Base directory of the project
BASE_DIR = Path(__file__).parent.parent

# Files to check for broken links
DOC_FILES = [
    "ROOT_INDEX.md",
    "docs/INDEX.md",
    "README.md",
]

def find_markdown_links(content: str) -> List[str]:
    """Extract all markdown links from content."""
    # Match [text](url) pattern
    pattern = r'\[.*?\]\((.*?)(?:\s+".*?")?\)'
    matches = re.findall(pattern, content)
    # Filter out anchor-only links and web URLs
    return [m for m in matches if not m.startswith(('#', 'http://', 'https://'))]

def check_file_links(file_path: Path) -> List[Tuple[str, str]]:
    """Check all links in a file and return broken ones."""
    try:
        content = file_path.read_text(encoding='utf-8')
        links = find_markdown_links(content)
        broken = []
        
        for link in links:
            # Handle relative paths
            if link.startswith('/'):
                target = BASE_DIR / link[1:]
            else:
                target = file_path.parent / link
            
            # Check if target exists
            if not target.exists():
                broken.append((str(link), str(target)))
        
        return broken
    except Exception as e:
        print(f"Error processing {file_path}: {e}", file=sys.stderr)
        return [(f"ERROR: {e}", str(file_path))]

def validate_documentation() -> bool:
    """Validate all documentation files and links."""
    success = True
    
    print("🔍 Validating documentation structure...\n")
    
    # Check all documentation files
    for doc_file in DOC_FILES:
        doc_path = BASE_DIR / doc_file
        if not doc_path.exists():
            print(f"❌ Documentation file not found: {doc_file}")
            success = False
            continue
            
        print(f"📄 Checking {doc_file}...")
        broken_links = check_file_links(doc_path)
        
        if broken_links:
            success = False
            print(f"  ❌ Found {len(broken_links)} broken link(s):")
            for link, target in broken_links:
                print(f"     - {link} (resolved to: {target})")
        else:
            print(f"  ✅ All links are valid")
    
    # Check for required directories
    required_dirs = [
        "docs/api",
        "docs/architecture",
        "docs/security",
        "docs/bitcoin",
        "docs/layer2",
        "docs/identity",
        "docs/ml",
        "docs/standards"
    ]
    
    print("\n🔍 Checking required directories...")
    for dir_path in required_dirs:
        full_path = BASE_DIR / dir_path
        if not full_path.exists():
            print(f"⚠️  Directory not found: {dir_path}")
    
    return success

if __name__ == "__main__":
    if validate_documentation():
        print("\n✅ Documentation structure is valid!")
        sys.exit(0)
    else:
        print("\n❌ Documentation validation found issues.")
        sys.exit(1)
