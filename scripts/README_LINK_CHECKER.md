# Documentation Link Checker

[AIR-3][AIS-3][BPC-3][RES-3]

## Overview

This script checks for broken links in the Anya Core documentation. It's an essential tool for maintaining documentation quality and ensuring all references are valid.

## Features

- Recursively checks all Markdown files in the `docs/` directory
- Validates both internal and external links
- Supports custom ignore patterns for local development URLs
- Provides detailed error reporting
- Can be integrated into CI/CD pipelines

## Prerequisites

- Node.js (v12 or higher)
- npm (comes with Node.js)
- `fd` (a fast alternative to `find`)

## Installation

1. Install the required Node.js package globally:

   ```bash
   npm install -g markdown-link-check
   ```

2. Install `fd` (replace with your package manager):

   ```bash
   # Ubuntu/Debian
   sudo apt install fd-find
   
   # macOS (with Homebrew)
   brew install fd
   
   # Windows (with Chocolatey)
   choco install fd
   ```

## Usage

### Basic Usage

```bash
./scripts/check_links.sh
```

This will check all Markdown files in the `docs/` directory and the root `README.md`.

### Integration with Git Hooks

To automatically check links before pushing changes, add this to your `.git/hooks/pre-push`:

```bash
#!/bin/bash

# Only run in the root of the repository
if [ -f "scripts/check_links.sh" ]; then
    echo "Checking documentation links..."
    if ! ./scripts/check_links.sh; then
        echo "Error: Broken links found. Please fix them before pushing."
        exit 1
    fi
fi

exit 0
```

Make the hook executable:

```bash
chmod +x .git/hooks/pre-push
```

## Configuration

The script uses a configuration file (`.markdown-link-check.json`) in the project root. You can customize:

- `ignorePatterns`: Regular expressions for URLs to ignore
- `replacementPatterns`: Rules for rewriting URLs before checking

Example configuration:

```json
{
    "ignorePatterns": [
        "^https?://localhost",
        "^#",
        "^mailto:",
        "^ftp:",
        "^/"
    ]
}
```

## Common Issues

### Local Development URLs

Local development URLs (like `http://localhost:8000`) are ignored by default. To check these, remove them from `ignorePatterns` in the config file.

### Relative Links

Relative links should work automatically as long as they're relative to the project root.

### Authentication Required

Some links might require authentication. These will be reported as broken unless added to `ignorePatterns`.

## Performance

The script processes files in parallel for better performance. However, checking external links can be slow due to network latency.

## License

This script is part of the Anya Core project and is licensed under the [MIT License](../LICENSE).
