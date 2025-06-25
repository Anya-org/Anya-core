#!/usr/bin/env python3
"""
Simple Link Campaign - Documentation link fixer
Created: June 17, 2025
Purpose: Find and fix broken links in markdown documentation
"""

import os
import re
import sys
import json
from typing import Dict, List, Set, Optional, Tuple, Any

# Configuration
ROOT_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
DOCS_DIR = os.path.join(ROOT_DIR, "docs")
REPORT_PATH = os.path.join(ROOT_DIR, "link_campaign_report.md")
LINK_MAPPING_FILE = os.path.join(ROOT_DIR, "scripts", "link_mappings.json")

# Colors for console output
GREEN = "\033[92m"
YELLOW = "\033[93m"
RED = "\033[91m"
RESET = "\033[0m"

# Load custom link mappings
custom_mappings: Dict[str, str] = {}
try:
    if os.path.exists(LINK_MAPPING_FILE):
        with open(LINK_MAPPING_FILE, 'r') as f:
            custom_mappings = json.load(f)
        print(f"Loaded {len(custom_mappings)} custom mappings")
except Exception as e:
    print(f"Warning: Could not load custom mappings: {e}")

# Global file tracking
all_files: Set[str] = set()


def find_markdown_files() -> None:
    """Find all markdown files in the repository"""
    global all_files

    print(f"{GREEN}Scanning for markdown files...{RESET}")

    for root, dirs, files in os.walk(ROOT_DIR):
        # Skip hidden directories and common build/cache directories
        dirs[:] = [d for d in dirs if not d.startswith('.') and d not in [
            'node_modules', '__pycache__', 'target', 'build', 'dist'
        ]]

        for file in files:
            if file.endswith('.md'):
                full_path = os.path.join(root, file)
                # Convert to relative path from root
                rel_path = os.path.relpath(full_path, ROOT_DIR)
                all_files.add(rel_path)

    print(f"{GREEN}Found {len(all_files)} markdown files{RESET}")


def find_best_match(broken_link: str, source_file: str) -> Optional[str]:
    """Find the best matching file for a broken link"""

    # Check custom mappings first
    if broken_link in custom_mappings:
        return custom_mappings[broken_link]

    # Extract filename from the broken link
    filename = os.path.basename(broken_link)
    matches: List[str] = []

    # Look for exact filename matches
    for file_path in all_files:
        if os.path.basename(file_path) == filename:
            matches.append(file_path)

    # If no exact matches, try case-insensitive
    if not matches:
        for file_path in all_files:
            if os.path.basename(file_path).lower() == filename.lower():
                matches.append(file_path)

    if matches:
        # If multiple matches, prefer ones in similar directory structure
        source_dir = os.path.dirname(source_file)

        # Sort by similarity to source directory
        def similarity_score(match_path: str) -> int:
            match_dir = os.path.dirname(match_path)
            common_parts = 0
            source_parts = source_dir.split(os.sep)
            match_parts = match_dir.split(os.sep)

            for i in range(min(len(source_parts), len(match_parts))):
                if source_parts[i] == match_parts[i]:
                    common_parts += 1
                else:
                    break
            return common_parts

        matches.sort(key=similarity_score, reverse=True)

        # Calculate relative path from source file to the best match
        source_dir = os.path.dirname(source_file)
        best_match = matches[0]

        # Calculate relative path
        try:
            rel_path = os.path.relpath(best_match, source_dir)
            return rel_path
        except ValueError:
            # Fallback to absolute path if relative calculation fails
            return best_match

    return None


def check_links_in_file(file_path: str) -> Tuple[List[Dict[str, Any]], List[Dict[str, Any]]]:
    """Check all links in a markdown file"""
    broken_links: List[Dict[str, Any]] = []
    valid_links: List[Dict[str, Any]] = []

    try:
        with open(os.path.join(ROOT_DIR, file_path), 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"{RED}Error reading {file_path}: {e}{RESET}")
        return broken_links, valid_links

    # Find all markdown links
    link_pattern = re.compile(r'\[([^\]]+)\]\(([^)]+)\)')
    links = link_pattern.findall(content)

    for link_text, link_path in links:
        # Skip external links
        if link_path.startswith(('http://', 'https://', 'mailto:', '#')):
            continue

        # Skip anchors within the same file
        if link_path.startswith('#'):
            continue

        # Resolve relative path
        source_dir = os.path.dirname(file_path)
        full_path = os.path.normpath(os.path.join(source_dir, link_path))

        # Check if file exists
        if os.path.exists(os.path.join(ROOT_DIR, full_path)):
            valid_links.append({
                'source': file_path,
                'text': link_text,
                'link': link_path,
                'resolved': full_path
            })
        else:
            broken_links.append({
                'source': file_path,
                'text': link_text,
                'link': link_path,
                'resolved': full_path
            })

    return broken_links, valid_links


def fix_links_in_file(file_path: str, fixes: Dict[str, str]) -> int:
    """Apply link fixes to a file"""
    try:
        with open(os.path.join(ROOT_DIR, file_path), 'r', encoding='utf-8') as f:
            content = f.read()
    except Exception as e:
        print(f"{RED}Error reading {file_path}: {e}{RESET}")
        return 0

    original_content = content
    fixes_applied = 0

    for old_link, new_link in fixes.items():
        # Create the markdown link pattern for replacement
        old_pattern = re.escape(f"]({old_link})")
        new_replacement = f"]({new_link})"

        # Replace all occurrences
        new_content = re.sub(old_pattern, new_replacement, content)
        if new_content != content:
            content = new_content
            fixes_applied += 1

    # Write back if changes were made
    if content != original_content:
        try:
            with open(os.path.join(ROOT_DIR, file_path), 'w', encoding='utf-8') as f:
                f.write(content)
        except Exception as e:
            print(f"{RED}Error writing {file_path}: {e}{RESET}")
            return 0

    return fixes_applied


def generate_report(broken_links: List[Dict[str, Any]], fixed_links: List[Dict[str, Any]],
                    manual_review: List[Dict[str, Any]], dry_run: bool = True) -> None:
    """Generate a detailed report of the link campaign"""

    with open(REPORT_PATH, 'w') as report:
        report.write("# Link Campaign Report\n\n")
        report.write(
            f"*Generated: {__import__('datetime').datetime.now().strftime('%Y-%m-%d %H:%M:%S')}*\n\n")

        # Summary
        report.write("## Summary\n\n")
        report.write(f"- **Total broken links found**: {len(broken_links)}\n")
        report.write(f"- **Links automatically fixed**: {len(fixed_links)}\n")
        report.write(
            f"- **Links requiring manual review**: {len(manual_review)}\n")
        report.write(
            f"- **Mode**: {'Dry run' if dry_run else 'Applied fixes'}\n\n")

        # Fixed links
        if fixed_links:
            report.write("## Automatically Fixed Links\n\n")
            report.write("| Source File | Link Text | Old Path | New Path |\n")
            report.write("|-------------|-----------|----------|----------|\n")

            for link in fixed_links:
                report.write(
                    f"| {link['source']} | {link['text']} | {link['link']} | {link['suggested_fix']} |\n")

            report.write("\n")

        # Manual review needed
        if manual_review:
            report.write("## Links Requiring Manual Review\n\n")
            report.write(
                "| Source File | Link Text | Broken Path | Suggested Fix |\n")
            report.write(
                "|-------------|-----------|-------------|---------------|\n")

            for link in manual_review:
                fix = link['suggested_fix'] if link['suggested_fix'] else "No suggestion"
                report.write(
                    f"| {link['source']} | {link['text']} | {link['link']} | {fix} |\n")

            report.write("\n")

        # Guidance
        report.write("## Next Steps\n\n")
        report.write("1. Review this report for any incorrectly fixed links\n")
        report.write(
            "2. Manually update links in the 'Manual Review' section\n")
        report.write(
            "3. Run the link checker again to verify all issues are resolved\n")
        report.write("4. Update custom mappings for problematic links\n")


def main() -> int:
    import argparse

    parser = argparse.ArgumentParser(description='Simple Link Campaign Tool')
    parser.add_argument('--fix', action='store_true',
                        help='Apply fixes (default is dry run)')
    parser.add_argument('--report', action='store_true',
                        help='Only generate report, don\'t check links')
    args = parser.parse_args()

    print(f"{GREEN}Simple Link Campaign - Starting{RESET}")
    print("=" * 50)

    # Find all markdown files
    find_markdown_files()

    # Check all links
    all_broken_links: List[Dict[str, Any]] = []
    all_valid_links: List[Dict[str, Any]] = []

    print(f"{GREEN}Checking links in all markdown files...{RESET}")

    for file_path in sorted(all_files):
        broken_links, valid_links = check_links_in_file(file_path)
        all_broken_links.extend(broken_links)
        all_valid_links.extend(valid_links)

        if broken_links:
            print(f"{YELLOW}  {file_path}: {len(broken_links)} broken link(s){RESET}")

    print(f"\n{GREEN}Link Analysis Complete{RESET}")
    print(
        f"- Total links found: {len(all_broken_links) + len(all_valid_links)}")
    print(f"- Broken links: {len(all_broken_links)}")
    print(f"- Valid links: {len(all_valid_links)}")

    # Try to fix broken links
    fixed_links: List[Dict[str, Any]] = []
    manual_review: List[Dict[str, Any]] = []

    print(f"\n{GREEN}Analyzing broken links for potential fixes...{RESET}")

    for broken_link in all_broken_links:
        suggested_fix = find_best_match(
            broken_link['link'], broken_link['source'])

        if suggested_fix:
            broken_link['suggested_fix'] = suggested_fix
            fixed_links.append(broken_link)

            if args.fix:
                # Apply the fix
                fixes = {broken_link['link']: suggested_fix}
                fixes_applied = fix_links_in_file(broken_link['source'], fixes)
                if fixes_applied > 0:
                    print(
                        f"{GREEN}  Fixed: {broken_link['link']} -> {suggested_fix}{RESET}")
        else:
            broken_link['suggested_fix'] = None
            manual_review.append(broken_link)

    # Generate report
    generate_report(all_broken_links, fixed_links, manual_review, not args.fix)

    print(f"\n{GREEN}=" * 50)
    print(f"Link Campaign Complete{RESET}")
    print(f"- Broken links found: {len(all_broken_links)}")
    print(f"- Automatic fixes available: {len(fixed_links)}")
    print(f"- Manual review needed: {len(manual_review)}")

    if args.report:
        print(f"{GREEN}View the detailed report at: {REPORT_PATH}{RESET}")
    elif args.fix:
        print(f"{GREEN}Links fixed! View the detailed report at: {REPORT_PATH}{RESET}")
    else:
        print(f"{YELLOW}Dry run complete. Use --fix to apply changes.{RESET}")
        print(f"{YELLOW}View the detailed report at: {REPORT_PATH}{RESET}")

    return 0 if len(all_broken_links) == len(fixed_links) else 1


if __name__ == "__main__":
    sys.exit(main())
