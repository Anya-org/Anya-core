#!/usr/bin/env python3
"""
Documentation Link Checker

This script checks for broken links in Markdown documentation.
"""

import os
import re
import sys
from pathlib import Path

# Color codes for terminal output
RED = "\033[0;31m"
GREEN = "\033[0;32m"
NC = "\033[0m"  # No Color

def find_markdown_files(directory):
    """Find all Markdown files in the given directory."""
    return list(Path(directory).rglob("*.md"))

def check_links_in_file(file_path):
    """Check all links in a Markdown file."""
    errors = []
    doc_dir = file_path.parent
    
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # Find all markdown links [text](url)
        for match in re.finditer(r'\[([^\]]+)\]\(([^)]+)\)', content):
            link_text = match.group(1)
            link_url = match.group(2).split()[0]  # Get URL part before any space
            
            # Skip external links, mailto, and anchor-only links
            if (link_url.startswith('http') or 
                link_url.startswith('mailto:') or 
                link_url.startswith('#') or 
                not link_url):
                continue
                
            # Handle relative paths
            if link_url.startswith('/'):
                # Path is relative to docs directory
                target_path = Path("docs") / link_url[1:]
            else:
                # Path is relative to current file
                target_path = file_path.parent / link_url
                
            # Normalize path
            target_path = target_path.resolve()
            
            # Check if the target exists
            if not target_path.exists():
                line_num = content[:match.start()].count('\n') + 1
                errors.append(f"{file_path}:{line_num} - Broken link: {link_url} (text: {link_text})")
                
    except Exception as e:
        errors.append(f"Error processing {file_path}: {str(e)}")
        
    return errors

def main():
    """Main function."""
    docs_dir = Path("docs")
    if not docs_dir.exists():
        print(f"{RED}Error:{NC} 'docs' directory not found")
        return 1
        
    print("Checking documentation links...")
    
    all_errors = []
    markdown_files = find_markdown_files(docs_dir)
    
    for md_file in markdown_files:
        errors = check_links_in_file(md_file)
        all_errors.extend(errors)
    
    # Print all errors
    for error in all_errors:
        print(f"{RED}{error}{NC}")
    
    # Print summary
    if all_errors:
        print(f"\n{RED}Found {len(all_errors)} broken links in documentation{NC}")
        return 1
    else:
        print(f"\n{GREEN}All documentation links are valid!{NC}")
        return 0

if __name__ == "__main__":
    sys.exit(main())
