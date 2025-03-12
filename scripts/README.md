# Documentation Scripts \[AIR-1\]\[AIT-1\]

<!-- markdownlint-disable MD013 line-length -->

This directory contains scripts for managing project documentation.

## Markdown Linting and Fixing

We use markdownlint to ensure consistent formatting and style in our markdown documentation. The configuration is in `.markdownlint.json` at the project root.

### Automated Fixing Script

The `fix_markdown.js` script automates fixing common markdownlint issues across the project:

1. **Installation**:
   ```bash
   npm install --save-dev markdownlint-cli2
   ```

2. **Usage**:
   ```bash
   # Fix markdown files in the docs directory (default)
   node scripts/fix_markdown.js

   # Fix markdown files in a specific directory
   node scripts/fix_markdown.js path/to/directory
   ```

3. **What It Does**:
   - Escapes AI labelling tags like `[AIR-3]` to `\[AIR-3\]` to prevent markdownlint errors
   - Adds markdownlint disable comments for line length where needed
   - Fixes line endings and other common issues
   - Runs markdownlint-cli2 with the --fix flag for additional fixes

### Manual Fixes

Some markdownlint issues require manual fixing:

1. **Multiple top-level headings (MD025)**:
   - Each markdown file should have only one H1 (`#`) heading
   - Change subsequent H1 headings to H2 (`##`)

2. **Duplicate headings (MD024)**:
   - Avoid duplicate heading text at the same level
   - Add additional text or use different wording

3. **Reference-style links missing definitions (MD052)**:
   - Escape AI labelling tags with backslashes: `\[AIR-3\]`
   - Or add the missing reference definitions

## Documentation Style Guide

Please refer to [MARKDOWN_STYLE_GUIDE.md](../docs/MARKDOWN_STYLE_GUIDE.md) for comprehensive documentation standards.

## Common Issues and Solutions

| Issue | Rule | Solution |
|-------|------|----------|
| Line too long | MD013 | Disabled in config, no action needed |
| Multiple top-level headings | MD025 | Keep only one H1 heading per file |
| AI tags interpreted as references | MD052 | Escape tags with backslashes: `\[AIR-3\]` |
| HTML-like tags | MD033 | Disabled in config, but minimize usage |
| No code block language specified | MD040 | Add language after backticks: ```rust |
| Emphasis used as heading | MD036 | Use proper heading syntax instead |

## Integration with CI/CD

The markdown linting process is integrated with our CI/CD pipeline:

1. **Pull Requests**: Automatically checks markdown files for linting issues
2. **Pre-commit Hook**: Validates markdown files before commits (optional setup)
3. **Documentation Generation**: Ensures consistently formatted documentation for generated sites

## Last Updated

2025-03-12 