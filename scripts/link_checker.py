#!/usr/bin/env python3

import os
import re

broken = []
# Regex to capture markdown links with relative paths (skip http/https)
link_regex = re.compile(r'\[.*?\]\((?!http)([^)]+)\)')

for root, dirs, files in os.walk('.'):
    # skip version control directories
    if '.git' in dirs:
        dirs.remove('.git')
    for fname in files:
        if fname.endswith('.md'):
            fpath = os.path.join(root, fname)
            with open(fpath, 'r', encoding='utf-8') as f:
                for lineno, line in enumerate(f, start=1):
                    for match in link_regex.finditer(line):
                        link = match.group(1).split('#')[0]
                        # ignore empty links
                        if not link:
                            continue
                        # resolve path
                        target = os.path.normpath(os.path.join(root, link))
                        if not os.path.exists(target):
                            broken.append((fpath, lineno, link))
# Output report
if broken:
    print("Broken links detected:")
    for fpath, lineno, link in broken:
        print(f"{fpath}:{lineno} -> {link}")
    exit(1)
else:
    print("No broken links found.")
    exit(0)
