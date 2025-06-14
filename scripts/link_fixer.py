#!/usr/bin/env python3

import os
import re
import sys

# Regex to capture markdown links with relative paths (skip http/https)
link_regex = re.compile(r"(\[.*?\]\()(?P<link>(?!http)([^)]+))(\))")

def find_target(root_dir, filename):
    for dirpath, dirs, files in os.walk(root_dir):
        if filename in files:
            return os.path.join(dirpath, filename)
    return None


def fix_links(base_dir):
    broken = []
    for root, dirs, files in os.walk(base_dir):
        # skip .git
        dirs[:] = [d for d in dirs if d != '.git']
        for fname in files:
            if not fname.endswith('.md'):
                continue
            fpath = os.path.join(root, fname)
            updated = False
            lines = []
            with open(fpath, 'r', encoding='utf-8') as f:
                for line in f:
                    def repl(m):
                        link = m.group('link').split('#')[0]
                        # skip empty or absolute paths
                        if not link or link.startswith('/'):
                            return m.group(0)
                        target = os.path.normpath(os.path.join(root, link))
                        if os.path.exists(target):
                            return m.group(0)
                        filename = os.path.basename(link)
                        new_abs = find_target(base_dir, filename)
                        if new_abs:
                            rel = os.path.relpath(new_abs, start=root)
                            sys.stdout.write(f"[FIX] {fpath}:{fname} {link} -> {rel}\n")
                            updated = True
                            return m.group(1) + rel + m.group(4)
                        return m.group(0)
                    newline = link_regex.sub(repl, line)
                    lines.append(newline)
            if updated:
                with open(fpath, 'w', encoding='utf-8') as f:
                    f.writelines(lines)
    print("Link fixing completed.")

if __name__ == '__main__':
    base_dir = os.getcwd()
    fix_links(base_dir)
