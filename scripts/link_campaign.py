#!/usr/bin/env python3
"""
Link Campaign - Advanced documentation link fixer
Created: June 17, 2025
Purpose: Find and fix broken links in ma        # Return the best match if found
        if matches:
            best_match = matches[0][0]
            try:
                source_dir = os.path.dirname(os.path.join(ROOT_DIR, source_file))
                target_path = os.path.join(ROOT_DIR, best_match)
                
                # Handle empty source directory
                if not source_dir:
                    source_dir = ROOT_DIR
                    
                rel_path = os.path.relpath(target_path, source_dir)
                
                # Normalize path with forward slashes
                rel_path = rel_path.replace('\\', '/')
                
                # Add anchor if present
                if anchor:
                    rel_path = f"{rel_path}#{anchor}"
                    
                return rel_path
            except Exception as e:
                print(f"Error calculating relative path: {e}")
                # Return absolute path as fallback
                return "/" + best_matchmentation

Features:
- Finds broken links across all documentation
- Suggests fixes based on filename matching
- Fixes links automatically when a confident match is found
- Generates report of changes and remaining issues
- Handles different link formats (relative, absolute, with anchors)
"""

import os
import re
import sys
import json
from collections import defaultdict
from pathlib import Path

# Configuration
ROOT_DIR = os.path.abspath(os.path.join(os.path.dirname(__file__), ".."))
DOCS_DIR = os.path.join(ROOT_DIR, "docs")
REPORT_PATH = os.path.join(ROOT_DIR, "link_campaign_report.md")
LINK_MAPPING_FILE = os.path.join(ROOT_DIR, "scripts", "link_mappings.json")

# Regex patterns
MARKDOWN_LINK_PATTERN = re.compile(r'\[([^\]]+)\]\(([^)]+)\)')
ANCHOR_PATTERN = re.compile(r'(.*)#(.*)')

# Console colors
RED = '\033[91m'
GREEN = '\033[92m'
YELLOW = '\033[93m'
BLUE = '\033[94m'
RESET = '\033[0m'

# Link database - structure to hold all documents and links


class LinkDatabase:
    def __init__(self):
        self.all_files = set()  # All available markdown files
        self.file_content = {}  # Cache of file content
        self.broken_links = []  # List of broken links found
        self.fixed_links = []   # List of links that were fixed
        self.manual_review = []  # Links that need manual review
        self.custom_mappings = {}  # User-defined link mappings

    def load_custom_mappings(self):
        """Load any custom link mappings from the mapping file"""
        if os.path.exists(LINK_MAPPING_FILE):
            try:
                with open(LINK_MAPPING_FILE, 'r') as f:
                    self.custom_mappings = json.load(f)
                print(
                    f"{BLUE}Loaded {len(self.custom_mappings)} custom link mappings{RESET}")
            except json.JSONDecodeError:
                print(
                    f"{YELLOW}Warning: Could not parse custom link mappings file{RESET}")

    def save_custom_mappings(self):
        """Save custom mappings back to the file"""
        with open(LINK_MAPPING_FILE, 'w') as f:
            json.dump(self.custom_mappings, f, indent=2, sort_keys=True)

    def scan_all_files(self):
        """Build a database of all markdown files in the repository"""
        print(f"{BLUE}Building file database...{RESET}")
        for root, dirs, files in os.walk(ROOT_DIR):
            # Skip .git and other hidden directories
            dirs[:] = [d for d in dirs if not d.startswith('.')]

            for file in files:
                if file.lower().endswith('.md'):
                    full_path = os.path.join(root, file)
                    rel_path = os.path.relpath(full_path, ROOT_DIR)
                    self.all_files.add(rel_path)

        print(f"{GREEN}Found {len(self.all_files)} markdown files{RESET}")

    def find_best_match(self, broken_link, source_file):
        """Find the best match for a broken link based on filename"""
        if broken_link in self.custom_mappings:
            return self.custom_mappings[broken_link]

        # Extract the filename from the broken link
        link_parts = broken_link.split('/')
        target_file = link_parts[-1]
        anchor = None

        # Handle anchors
        anchor_match = ANCHOR_PATTERN.match(target_file)
        if anchor_match:
            target_file = anchor_match.group(1)
            anchor = anchor_match.group(2)

        # Look for exact filename matches
        matches = []
        for file in self.all_files:
            if file.lower().endswith('/' + target_file.lower()):
                # Calculate distance from source to potential target
                source_dir = os.path.dirname(source_file)
                target_dir = os.path.dirname(file)
                # Handle empty paths for relpath
                if not source_dir:
                    source_dir = "."
                if not target_dir:
                    target_dir = "."
                try:
                    rel_path = os.path.relpath(target_dir, source_dir)
                    distance = len(rel_path.split('/'))
                except ValueError:
                    # If paths are on different drives, set a high distance
                    distance = 1000
                matches.append((file, distance))

        # If no match found by filename, try case-insensitive match
        if not matches:
            for file in self.all_files:
                if os.path.basename(file).lower() == target_file.lower():
                    source_dir = os.path.dirname(source_file)
                    target_dir = os.path.dirname(file)
                    # Handle empty paths for relpath
                    if not source_dir:
                        source_dir = "."
                    if not target_dir:
                        target_dir = "."
                    try:
                        rel_path = os.path.relpath(target_dir, source_dir)
                        distance = len(rel_path.split('/'))
                    except ValueError:
                        # If paths are on different drives, set a high distance
                        distance = 1000
                    matches.append((file, distance))

        # Sort matches by distance (ascending)
        matches.sort(key=lambda x: x[1])

        # Return the best match if found
        if matches:
            best_match = matches[0][0]
            rel_path = os.path.relpath(os.path.join(ROOT_DIR, best_match),
                                       os.path.dirname(os.path.join(ROOT_DIR, source_file)))
            # Normalize path with forward slashes
            rel_path = rel_path.replace('\\', '/')
            # Add anchor if present
            if anchor:
                rel_path = f"{rel_path}#{anchor}"
            return rel_path

        return None

    def check_links(self, dry_run=True):
        """Check all links in all markdown files"""
        print(f"{BLUE}Checking links in all files...{RESET}")
        for source_file in self.all_files:
            full_path = os.path.join(ROOT_DIR, source_file)
            if not os.path.exists(full_path):
                continue

            with open(full_path, 'r', encoding='utf-8') as f:
                content = f.read()
                self.file_content[source_file] = content

            # Find all markdown links
            links = MARKDOWN_LINK_PATTERN.findall(content)
            for text, link in links:
                # Skip external links and absolute links to pages
                if link.startswith(('http://', 'https://', '#', '/')):
                    continue

                # Normalize the path
                # Remove anchors for path checking
                link_path = link.split('#')[0]
                if not link_path:  # Skip anchor-only links
                    continue

                # Resolve the link path relative to the source file
                link_full_path = os.path.normpath(
                    os.path.join(os.path.dirname(
                        os.path.join(ROOT_DIR, source_file)), link_path)
                )
                link_rel_path = os.path.relpath(link_full_path, ROOT_DIR)

                # Check if the target exists
                if not os.path.exists(link_full_path):
                    # This link is broken
                    best_match = self.find_best_match(link, source_file)
                    self.broken_links.append({
                        'source': source_file,
                        'text': text,
                        'link': link,
                        'suggested_fix': best_match
                    })

        print(f"{YELLOW}Found {len(self.broken_links)} broken links{RESET}")

        # Fix links if not a dry run
        if not dry_run:
            self.fix_links()

    def fix_links(self):
        """Fix broken links where a confident match exists"""
        print(f"{BLUE}Fixing broken links...{RESET}")

        # Group broken links by source file for efficient processing
        links_by_file = defaultdict(list)
        for link_info in self.broken_links:
            links_by_file[link_info['source']].append(link_info)

        # Process each file
        for source_file, links in links_by_file.items():
            content = self.file_content[source_file]
            updated_content = content
            file_changes = []

            # Sort links by length (descending) to avoid replacement conflicts
            links.sort(key=lambda x: len(x['link']), reverse=True)

            for link_info in links:
                if link_info['suggested_fix']:
                    old_link_pattern = re.escape(
                        f'[{link_info["text"]}]({link_info["link"]})')
                    new_link = f'[{link_info["text"]}]({link_info["suggested_fix"]})'

                    # Check if this exact pattern exists in the content
                    if re.search(old_link_pattern, updated_content):
                        updated_content = re.sub(
                            old_link_pattern,
                            # Escape backslashes in replacement
                            new_link.replace('\\', '\\\\'),
                            updated_content
                        )
                        file_changes.append({
                            'old': f'[{link_info["text"]}]({link_info["link"]})',
                            'new': new_link
                        })
                        self.fixed_links.append(link_info)
                    else:
                        # If pattern not found, mark for manual review
                        self.manual_review.append(link_info)
                else:
                    # No suggested fix
                    self.manual_review.append(link_info)

            # Write back the updated content if changes were made
            if content != updated_content:
                full_path = os.path.join(ROOT_DIR, source_file)
                with open(full_path, 'w', encoding='utf-8') as f:
                    f.write(updated_content)
                print(
                    f"{GREEN}Fixed {len(file_changes)} links in {source_file}{RESET}")

                # Show the changes made
                for change in file_changes:
                    print(
                        f"  {RED}{change['old']}{RESET} → {GREEN}{change['new']}{RESET}")

        print(f"{GREEN}Fixed {len(self.fixed_links)} links automatically{RESET}")
        print(f"{YELLOW}{len(self.manual_review)} links need manual review{RESET}")

    def generate_report(self):
        """Generate a detailed report of link issues and fixes"""
        print(f"{BLUE}Generating report...{RESET}")

        with open(REPORT_PATH, 'w', encoding='utf-8') as report:
            report.write("# Link Campaign Report\n\n")
            report.write(f"Generated: June 17, 2025\n\n")

            # Summary
            report.write("## Summary\n\n")
            report.write(f"- Total markdown files: {len(self.all_files)}\n")
            report.write(f"- Broken links found: {len(self.broken_links)}\n")
            report.write(
                f"- Links fixed automatically: {len(self.fixed_links)}\n")
            report.write(
                f"- Links requiring manual review: {len(self.manual_review)}\n\n")

            # Links fixed
            if self.fixed_links:
                report.write("## Links Fixed Automatically\n\n")
                report.write(
                    "| Source File | Link Text | Old Path | New Path |\n")
                report.write(
                    "|-------------|-----------|----------|----------|\n")

                for link in self.fixed_links:
                    report.write(
                        f"| {link['source']} | {link['text']} | {link['link']} | {link['suggested_fix']} |\n")

                report.write("\n")

            # Links needing manual review
            if self.manual_review:
                report.write("## Links Requiring Manual Review\n\n")
                report.write(
                    "| Source File | Link Text | Broken Path | Suggested Fix |\n")
                report.write(
                    "|-------------|-----------|-------------|---------------|\n")

                for link in self.manual_review:
                    fix = link['suggested_fix'] if link['suggested_fix'] else "No suggestion"
                    report.write(
                        f"| {link['source']} | {link['text']} | {link['link']} | {fix} |\n")

                report.write("\n")

            # Guidance
            report.write("## Next Steps\n\n")
            report.write(
                "1. Review this report for any incorrectly fixed links\n")
            report.write(
                "2. Manually update links in the 'Manual Review' section\n")
            report.write(
                "3. Run the link checker again to verify all issues are resolved\n")
            report.write("4. Update custom mappings for problematic links\n\n")

            report.write("## How to Create Custom Link Mappings\n\n")
            report.write(
                "Edit the file `scripts/link_mappings.json` with entries like:\n\n")
            report.write("```json\n")
            report.write(
                "{\n  \"broken/path.md\": \"correct/path.md\",\n  \"another/broken.md\": \"../fixed.md\"\n}\n")
            report.write("```\n")

        print(f"{GREEN}Report generated at {REPORT_PATH}{RESET}")

        # Create or update the link mappings file
        if not os.path.exists(LINK_MAPPING_FILE):
            with open(LINK_MAPPING_FILE, 'w', encoding='utf-8') as f:
                json.dump({}, f, indent=2)
            print(
                f"{GREEN}Created empty link mappings file at {LINK_MAPPING_FILE}{RESET}")


def main():
    import argparse

    parser = argparse.ArgumentParser(
        description='Link Campaign - Find and fix broken links')
    parser.add_argument('--fix', action='store_true',
                        help='Automatically fix links where possible')
    parser.add_argument('--report', action='store_true',
                        help='Only generate a report, no fixes')
    args = parser.parse_args()

    db = LinkDatabase()

    print(f"{BLUE}Link Campaign - Starting{RESET}")

    # Initialize the database
    db.load_custom_mappings()
    db.scan_all_files()

    # Check all links
    db.check_links(dry_run=not args.fix)

    # Generate report
    db.generate_report()

    print(f"{BLUE}Link Campaign - Complete{RESET}")

    if args.report:
        print(f"\n{GREEN}View the detailed report at: {REPORT_PATH}{RESET}\n")
    elif args.fix:
        print(
            f"\n{GREEN}Links fixed! View the detailed report at: {REPORT_PATH}{RESET}\n")
        db.save_custom_mappings()
    else:
        print(f"\n{YELLOW}Dry run complete. Use --fix to apply changes.{RESET}\n")
        print(f"{YELLOW}View the detailed report at: {REPORT_PATH}{RESET}\n")

    return 0 if not db.manual_review else 1


if __name__ == "__main__":
    sys.exit(main())
