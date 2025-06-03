# Documentation Status Report Generator

[AIR-3][AIS-3][BPC-3][RES-3]

## Overview

This script generates a comprehensive status report for the Anya Core documentation. It provides valuable insights into the documentation's health, recent changes, and areas that may need attention.

## Features

- **Documentation Statistics**: Counts files, words, and Markdown files
- **Issue Tracking**: Identifies TODOs and FIXMEs in documentation
- **Link Health**: Integrates with the link checker to report broken links
- **Change Tracking**: Shows recent documentation changes from git history
- **Actionable Recommendations**: Provides suggestions for documentation improvements

## Prerequisites

- Bash 4.0+
- Git
- `markdown-link-check` (for link checking, optional)

## Usage

### Basic Usage

```bash
./scripts/doc_status.sh
```

This will generate a status report in `docs/status/REPORT_YYYY-MM-DD.md` and display a preview in the terminal.

### Schedule Regular Reports

To generate weekly reports, add this to your crontab:

```bash
# Run every Monday at 9 AM
0 9 * * 1 cd /path/to/anya-core && ./scripts/doc_status.sh
```

## Report Contents

Each status report includes:

1. **Documentation Statistics**
   - Total files and Markdown files
   - Word count
   - Number of TODOs/FIXMEs

2. **Link Status**
   - Number of broken links (if link checker is available)

3. **Recent Changes**
   - Git history of documentation changes from the past week

4. **Recommendations**
   - Actionable suggestions for improving documentation quality

## Integration

### CI/CD Pipeline

Add this to your CI/CD pipeline to fail on critical documentation issues:

```yaml
- name: Check Documentation Status
  run: |
    ./scripts/doc_status.sh
    TODOS=$(grep -r -i -E 'TODO|FIXME' docs/ | wc -l)
    if [ "$TODOS" -gt 10 ]; then
      echo "Too many TODOs/FIXMEs in documentation"
      exit 1
    fi
```

### Pre-commit Hook

To check documentation status before each commit, add this to `.git/hooks/pre-commit`:

```bash
#!/bin/bash

# Only run in the root of the repository
if [ -f "scripts/doc_status.sh" ]; then
    echo "Checking documentation status..."
    if ! ./scripts/doc_status.sh; then
        echo "Documentation issues found. Please check the report."
        exit 1
    fi
fi

exit 0
```

## Customization

### Configuration

The script uses the following environment variables for customization:

- `DOCS_DIR`: Path to the documentation directory (default: `./docs`)
- `REPORT_FILE`: Output file path (default: `docs/status/REPORT_YYYY-MM-DD.md`)

### Ignoring Files

To exclude files from the status report, add them to `.gitignore` or use the `--exclude` option with `find` in the script.

## Best Practices

1. **Regular Reviews**: Run the status report weekly to catch issues early
2. **Address TODOs**: Keep the number of TODOs low by addressing them promptly
3. **Monitor Changes**: Review the recent changes section to ensure documentation stays up to date
4. **Automate Checks**: Integrate with CI/CD for automated quality control

## License

This script is part of the Anya Core project and is licensed under the [MIT License](../LICENSE).
