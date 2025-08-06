#!/usr/bin/env python3
"""
Simple Documentation Duplication Checker

This script provides a simple way to detect potential duplication in documentation
files without relying on the Rust implementation that currently has issues.

Usage:
  python3 simple_doc_duplication_check.py [--path PATH] [--ext EXTENSIONS] [--threshold THRESHOLD]

Options:
  --path PATH           Directory to scan for documentation files (default: ./docs)
  --ext EXTENSIONS      Comma-separated list of file extensions to scan (default: md,txt,rst)
  --threshold THRESHOLD Similarity threshold (0.0-1.0) for detecting duplicates (default: 0.85)
"""

import os
import sys
import re
import hashlib
import glob
from collections import defaultdict
import argparse


def normalize_content(text):
    """Normalize content by removing formatting, whitespace variations, etc."""
    # Convert to lowercase
    text = text.lower()
    # Remove markdown formatting
    text = re.sub(r'[#*_~`]', '', text)
    # Remove HTML tags
    text = re.sub(r'<[^>]+>', '', text)
    # Normalize whitespace
    text = re.sub(r'\s+', ' ', text)
    return text.strip()


def extract_sections(file_path):
    """Extract sections from a document file."""
    sections = []
    try:
        with open(file_path, 'r', encoding='utf-8') as f:
            content = f.read()
            
        # Split by headings
        heading_pattern = r'^(#{1,6}\s+.+)$'
        parts = re.split(heading_pattern, content, flags=re.MULTILINE)
        
        if len(parts) <= 1:
            # No headings found, treat entire file as one section
            sections.append({
                'title': os.path.basename(file_path),
                'content': content,
                'file_path': file_path,
                'word_count': len(content.split())
            })
        else:
            current_title = ""
            current_content = ""
            
            for i, part in enumerate(parts):
                if i % 2 == 0:  # Content part
                    current_content = part
                    if i > 0:  # Not the first part
                        sections.append({
                            'title': current_title.strip('# '),
                            'content': current_content,
                            'file_path': file_path,
                            'word_count': len(current_content.split())
                        })
                else:  # Heading part
                    current_title = part
                    
            # Add the last section
            if current_title and current_content:
                sections.append({
                    'title': current_title.strip('# '),
                    'content': current_content,
                    'file_path': file_path,
                    'word_count': len(current_content.split())
                })
    
    except Exception as e:
        print(f"Error processing {file_path}: {e}")
    
    return sections


def compute_fingerprint(text):
    """Compute a fingerprint for the text using Blake3 or SHA-256."""
    try:
        import blake3
        return blake3.blake3(text.encode('utf-8')).digest()
    except ImportError:
        # Fallback to SHA-256 if Blake3 is not available
        return hashlib.sha256(text.encode('utf-8')).digest()


def get_similarity(text1, text2):
    """Calculate similarity between two texts using Jaccard similarity."""
    # Simple implementation of Jaccard similarity on word sets
    words1 = set(normalize_content(text1).split())
    words2 = set(normalize_content(text2).split())
    
    if not words1 or not words2:
        return 0.0
        
    intersection = words1.intersection(words2)
    union = words1.union(words2)
    
    return len(intersection) / len(union)


def find_duplicates(sections, threshold):
    """Find duplicate sections based on similarity threshold."""
    duplicate_groups = []
    
    # Group sections by content hash to quickly identify exact duplicates
    content_hash_map = defaultdict(list)
    for section in sections:
        normalized_content = normalize_content(section['content'])
        if len(normalized_content) > 20:  # Only consider sections with sufficient content
            content_hash = compute_fingerprint(normalized_content)
            content_hash_map[content_hash].append(section)
    
    # Find exact duplicates
    for content_hash, group in content_hash_map.items():
        if len(group) > 1:
            duplicate_groups.append({
                'similarity': 1.0,
                'entries': group
            })
    
    # Find similar but not exact duplicates
    # This is a simple n^2 comparison - not efficient for large document sets
    if threshold < 1.0:
        processed = set()
        for i, section1 in enumerate(sections):
            if len(normalize_content(section1['content'])) < 20:
                continue
                
            for j, section2 in enumerate(sections[i+1:], i+1):
                if len(normalize_content(section2['content'])) < 20:
                    continue
                    
                pair_key = f"{i}_{j}"
                if pair_key in processed:
                    continue
                    
                # Skip exact duplicates that we already found
                hash1 = compute_fingerprint(normalize_content(section1['content']))
                hash2 = compute_fingerprint(normalize_content(section2['content']))
                if hash1 == hash2:
                    continue
                
                similarity = get_similarity(section1['content'], section2['content'])
                if similarity >= threshold:
                    duplicate_groups.append({
                        'similarity': similarity,
                        'entries': [section1, section2]
                    })
                    processed.add(pair_key)
    
    return duplicate_groups


def find_documentation_files(directory, extensions):
    """Find all documentation files with specified extensions."""
    files = []
    for ext in extensions:
        pattern = os.path.join(directory, f"**/*.{ext}")
        files.extend(glob.glob(pattern, recursive=True))
    return files


def main():
    parser = argparse.ArgumentParser(description='Simple documentation duplication checker')
    parser.add_argument('--path', default='./docs', help='Directory to scan')
    parser.add_argument('--ext', default='md,txt,rst', help='File extensions to scan')
    parser.add_argument('--threshold', type=float, default=0.85, help='Similarity threshold (0.0-1.0)')
    
    args = parser.parse_args()
    
    scan_path = args.path
    extensions = args.ext.split(',')
    threshold = args.threshold
    
    print(f"Scanning {scan_path} for documentation duplication...")
    print(f"File extensions: {', '.join(extensions)}")
    print(f"Similarity threshold: {threshold}")
    
    # Find all documentation files
    files = find_documentation_files(scan_path, extensions)
    print(f"Found {len(files)} documentation files")
    
    # Extract sections from files
    sections = []
    for file_path in files:
        file_sections = extract_sections(file_path)
        sections.extend(file_sections)
    print(f"Extracted {len(sections)} sections from documentation")
    
    # Find duplicates
    duplicate_groups = find_duplicates(sections, threshold)
    print(f"Found {len(duplicate_groups)} duplicate groups")
    
    # Print report
    print("\n=== Documentation Duplication Report ===\n")
    print(f"Files scanned: {len(files)}")
    print(f"Sections analyzed: {len(sections)}")
    print(f"Duplicate groups found: {len(duplicate_groups)}")
    
    if duplicate_groups:
        print("\nDuplicate Groups:\n")
        for i, group in enumerate(duplicate_groups, 1):
            print(f"Group {i} (Similarity: {group['similarity']:.2f}):")
            for entry in group['entries']:
                print(f"  - {entry['file_path']} (Section: {entry['title'][:50]}...)")
            print(f"    Word count: {group['entries'][0]['word_count']}")
            print(f"    Content preview: {group['entries'][0]['content'][:100]}...\n")
    
    return 0


if __name__ == "__main__":
    sys.exit(main())
