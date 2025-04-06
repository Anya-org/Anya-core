#!/usr/bin/env python3
"""
Fix platform-specific path handling in Rust files
[AIR-3][AIS-3][BPC-3]
"""
import os
import re
import sys
from pathlib import Path

def fix_path_in_file(file_path):
    """Replace hardcoded path separators with std::path functions"""
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Fix format!("{}/path", var) patterns
    content = re.sub(
        r'format!\("(.*)\/([^"]*)", ([^)]*)\)',
        r'std::path::Path::new(\3).join("\2").to_string_lossy()',
        content
    )
    
    # Fix hardcoded paths with forward slashes
    content = re.sub(
        r'([^:])"/([^"]+)"',
        r'\1std::path::Path::new("/").join("\2").to_string_lossy()',
        content
    )
    
    # Fix file operations with format! paths
    content = re.sub(
        r'(std::fs::[a-z_]+)\(format!\("(.*)/([^"]*)", ([^)]*)\)(.*)\)',
        r'\1(std::path::Path::new(\4).join("\3")\5)',
        content
    )
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print(f"Fixed paths in {file_path}")

def main():
    """Scan and fix Rust files"""
    if len(sys.argv) > 1:
        root_dir = Path(sys.argv[1])
    else:
        root_dir = Path('.')
    
    for path in root_dir.glob('**/*.rs'):
        fix_path_in_file(path)

if __name__ == "__main__":
    main()
