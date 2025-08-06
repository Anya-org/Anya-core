---
title: "Quickstart"
description: "Documentation for Quickstart"
last_updated: 2025-05-30
---

[AIR-3][AIS-3][BPC-3][RES-3]

# Documentation Quick Start

## Overview

Add a brief overview of this document here.

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


This guide will help you quickly set up and start using the Anya Core documentation system.

## Prerequisites

- Python 3.8 or higher
- pip (Python package manager)
- Git

## Setup

1. **Clone the repository** (if you haven't already):

   ```bash
   git clone https://github.com/anya-org/anya-core.git
   cd anya-core
   ```

2. **Set up the documentation environment**:

   ```bash
   ./scripts/setup_docs.sh
   ```

   This will:
   - Create a Python virtual environment
   - Install MkDocs and required plugins
   - Set up the documentation structure

## Viewing Documentation Locally

To view the documentation locally while you work:

```bash
./scripts/serve_docs.sh
```

This will start a local development server at http://127.0.0.1:8000 that automatically reloads when you make changes.

## Creating New Documentation

1. **Use the template** to create a new document:

   ```bash
   cp docs/.template.md docs/guides/my-new-guide.md
   ```

2. **Edit the metadata** at the top of the file:

   ```yaml
   ---
   title: "My New Guide"
   description: "A brief description of this guide"
   ---
   ```

3. **Add your content** using Markdown syntax.

4. **Update the navigation** in `mkdocs.yml` if you want your new document to appear in the site navigation.

## Documentation Standards

- Follow the [Markdown Style Guide](../standards/MARKDOWN_STYLE_GUIDE.md)
- Include proper AI labeling at the top of each file
- Keep lines under 100 characters
- Use descriptive link text

## Building for Production

To build the documentation for production:

```bash
./scripts/deploy_docs.sh
```

Select option 2 to deploy to GitHub Pages, or option 1 to preview the production build locally.

## Troubleshooting

### Common Issues

1. **Missing dependencies**:
   ```bash
   pip install -r requirements-docs.txt
   ```

2. **Broken links**:
   ```bash
   ./scripts/verify_docs.sh
   ```

3. **Formatting issues**:
   - Ensure all headers have blank lines before and after
   - Check for trailing whitespace
   - Verify all code blocks have language specified

## Getting Help

- Check the [Documentation System Guide](../DOCUMENTATION_SYSTEM.md)
- Review the [Markdown Style Guide](../standards/MARKDOWN_STYLE_GUIDE.md)
- [Open an issue](https://github.com/anya-org/anya-core/issues) if you need assistance

## Next Steps

- [Explore the API Reference](../api/README.md)
- [Read the Architecture Documentation](../architecture/README.md)
- [Learn about Contribution Guidelines](../CONTRIBUTING.md)

## See Also

- [Related Document](#related-document)

