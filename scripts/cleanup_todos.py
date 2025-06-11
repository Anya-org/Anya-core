#!/usr/bin/env python3
import os

# Remove references to TODO.md and inline TODO comments from docs
for root, dirs, files in os.walk('docs'):
    for fname in files:
        if fname.endswith('.md'):
            path = os.path.join(root, fname)
            with open(path, 'r') as f:
                lines = f.readlines()
            new_lines = []
            changed = False
            for line in lines:
                if 'TODO.md' in line or '<!-- TODO' in line:
                    changed = True
                    continue
                new_lines.append(line)
            if changed:
                with open(path, 'w') as f:
                    f.writelines(new_lines)
                print(f"Cleaned TODOs in {path}")
