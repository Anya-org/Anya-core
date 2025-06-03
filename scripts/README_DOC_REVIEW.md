# Documentation Review Script

[AIR-3][AIS-3][BPC-3][RES-3]

## Overview

This script performs a comprehensive review of all Markdown documentation in the Anya Core project. It checks for consistency, adherence to standards, and common issues across all documentation files.

## Features

- **AI Label Verification**: Ensures all files include required AI labels
- **Section Validation**: Checks for required documentation sections
- **Naming Conventions**: Validates file naming against project standards
- **TODO Tracking**: Identifies and reports TODOs and FIXMEs
- **Comprehensive Reporting**: Generates detailed HTML and Markdown reports

## Prerequisites

- Bash 4.0+
- Python 3.8+
- `grep`, `find`, and other standard Unix utilities

## Usage

### Basic Usage

```bash
./scripts/review_docs.sh
```

This will scan all `.md` files in the `docs/` directory and generate a report in `docs/REVIEW_REPORT_*.md`.

### Options

- `--html`: Generate an HTML report (requires `pandoc`)
- `--fix`: Automatically fix common issues (experimental)
- `--strict`: Treat warnings as errors

## Checks Performed

1. **AI Labels**
   - Verifies presence of required AI labels: `[AIR-3][AIS-3][BPC-3][RES-3]`

2. **Required Sections**
   - Checks for standard documentation sections:
     - Overview
     - Table of Contents
     - See Also

3. **File Naming**
   - Enforces kebab-case for filenames
   - Prohibits uppercase letters and underscores

4. **TODOs and FIXMEs**
   - Reports pending TODOs and FIXMEs
   - Can be configured to fail on TODOs in CI

## Integration

### Pre-commit Hook

Add this to `.git/hooks/pre-commit` to check documentation before each commit:

```bash
#!/bin/bash

if [ -f "scripts/review_docs.sh" ]; then
    echo "Checking documentation..."
    if ! ./scripts/review_docs.sh --strict; then
        echo "Documentation issues found. Please fix them before committing."
        exit 1
    fi
fi

exit 0
```

### CI/CD Pipeline

Add this to your CI/CD pipeline to enforce documentation standards:

```yaml
- name: Check Documentation
  run: |
    chmod +x ./scripts/review_docs.sh
    ./scripts/review_docs.sh --strict
```

## Customization

### Configuration

Create a `.docreviewrc` file in your project root to customize the script's behavior:

```json
{
  "required_sections": [
    "## Overview",
    "## Table of Contents",
    "## See Also"
  ],
  "ignore_patterns": [
    "node_modules/.*",
    "_build/.*"
  ],
  "max_line_length": 120
}
```

### Ignoring Files

Add patterns to `.gitignore` or `.docreviewignore` to exclude files from the review.

## Best Practices

1. **Run Regularly**: Include in your development workflow
2. **Fix Warnings**: Address all warnings to maintain high documentation quality
3. **Automate**: Integrate with CI/CD for automated quality control
4. **Customize**: Adapt the configuration to match your project's needs

## License

This script is part of the Anya Core project and is licensed under the [MIT License](../LICENSE).
