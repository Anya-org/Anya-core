# Documentation Update Script

[AIR-3][AIS-3][BPC-3][RES-3]

## Overview

This script automates the process of updating and standardizing documentation across the Anya Core project. It ensures all documentation files follow the project's standards for structure, formatting, and metadata.

## Features

- **Front Matter Management**: Adds/updates YAML front matter with title, description, and timestamps
- **AI Labeling**: Ensures all files include the required AI compliance labels
- **Section Standardization**: Adds missing standard sections (Overview, Table of Contents, See Also)
- **Parallel Processing**: Uses GNU parallel for faster processing when available
- **Backup System**: Creates backups before making changes
- **Detailed Logging**: Provides clear feedback on all changes made

## Prerequisites

- Bash 4.0+
- Python 3.8+
- GNU parallel (optional, for faster processing)
- Standard Unix utilities (sed, grep, awk, etc.)

## Usage

### Basic Usage

```bash
./scripts/update_docs.sh
```

This will scan all Markdown files in the `docs/` directory and update them to match the project standards.

### Options

- `--dry-run`: Show what changes would be made without modifying files
- `--help`: Show help message and usage information

## What It Does

1. **Front Matter**
   - Adds/updates YAML front matter with title, description, and last_updated fields
   - Uses the filename to generate a human-readable title

2. **AI Labels**
   - Adds the required AI compliance labels if missing:
     ```
     [AIR-3][AIS-3][BPC-3][RES-3]
     ```

3. **Standard Sections**
   - Adds missing standard sections:
     - ## Overview
     - ## Table of Contents
     - ## See Also

4. **Backup and Safety**
   - Creates a `.bak` backup of each modified file
   - Only keeps backups if changes were made
   - Shows a diff of changes before applying them in dry-run mode

## Integration

### Pre-commit Hook

Add this to `.git/hooks/pre-commit` to ensure documentation is always up to date:

```bash
#!/bin/bash

if [ -f "scripts/update_docs.sh" ]; then
    echo "Updating documentation..."
    if ! ./scripts/update_docs.sh; then
        echo "Documentation update failed"
        exit 1
    fi
    git add docs/
fi

exit 0
```

### CI/CD Pipeline

Add this to your CI/CD pipeline to ensure documentation standards:

```yaml
- name: Update Documentation
  run: |
    chmod +x ./scripts/update_docs.sh
    ./scripts/update_docs.sh --dry-run || {
      echo "Documentation needs updating. Run './scripts/update_docs.sh' locally and commit the changes."
      exit 1
    }
```

## Best Practices

1. **Run Before Committing**: Ensure all documentation is up to date
2. **Review Changes**: Always review the changes before committing
3. **Keep Backups**: The script creates `.bak` files that can be used to revert changes
4. **Use Dry Runs**: Test with `--dry-run` first to see what will be changed

## Troubleshooting

### Common Issues

1. **Permission Denied**
   ```bash
   chmod +x scripts/update_docs.sh
   ```

2. **sed: -i may not be used with stdin**
   This happens on macOS. Install GNU sed:
   ```bash
   brew install gnu-sed
   ```
   Then update the script to use `gsed` instead of `sed`.

3. **Parallel Processing Not Working**
   Install GNU parallel:
   ```bash
   # Ubuntu/Debian
   sudo apt install parallel
   
   # macOS
   brew install parallel
   ```

## License

This script is part of the Anya Core project and is licensed under the [MIT License](../LICENSE).
