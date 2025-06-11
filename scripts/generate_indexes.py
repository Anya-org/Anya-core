#!/usr/bin/env python3
import os

def slug_to_title(name):
    return name.replace('.md','').replace('-',' ').replace('_',' ').title()

# Generate index.md for each first-level docs directory
root = 'docs'
exclude_dirs = {'_layouts','ai','assets','pages','templates','system'}
for entry in os.listdir(root):
    dirpath = os.path.join(root, entry)
    if not os.path.isdir(dirpath):
        continue
    if entry in exclude_dirs:
        continue
    md_files = [f for f in os.listdir(dirpath) if f.endswith('.md')]
    if not md_files:
        continue
    # Create or overwrite index.md
    index_path = os.path.join(dirpath, 'index.md')
    title = slug_to_title(entry)
    lines = []
    # Front matter
    lines.append('---')
    lines.append('layout: default')
    lines.append(f'title: {title}')
    lines.append('---')
    lines.append('')
    # Heading
    lines.append(f'# {title}')
    lines.append('')
    # List pages
    for f in sorted(md_files):
        if f == 'index.md':
            continue
        link = f
        display = slug_to_title(f)
        lines.append(f'- [{display}]({link})')
    # Write file
    with open(index_path, 'w', encoding='utf-8') as f:
        f.write('\n'.join(lines) + '\n')

print('Generated index.md for docs subdirectories.')
