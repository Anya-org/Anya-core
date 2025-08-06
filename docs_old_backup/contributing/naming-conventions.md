---
title: "Naming Conventions"
description: "Standardized naming conventions for Anya Core documentation"
category: "contributing"
tags: ["standards", "documentation", "naming"]
last_updated: "2025-08-06"
compliance: ["AIR-3", "AIS-3"]
---

# Naming Conventions

## Overview
This document defines the standardized naming conventions for all documentation files and directories in the Anya Core project.

## Table of Contents
- [File Naming](#file-naming)
- [Directory Naming](#directory-naming)
- [Internal Links](#internal-links)
- [Code References](#code-references)
- [Compliance Tags](#compliance-tags)

## File Naming

### Markdown Files
All markdown documentation files must follow these conventions:

1. **Use kebab-case**: All lowercase with words separated by hyphens
   - ✅ `installation-guide.md`
   - ❌ `installationGuide.md`
   - ❌ `installation_guide.md`

2. **Be descriptive but concise**: File names should clearly indicate content
   - ✅ `api-authentication.md`
   - ❌ `auth.md`
   - ❌ `authentication-methods-for-application-programming-interfaces.md`

3. **Omit articles and unnecessary words**:
   - ✅ `transaction-validation.md`
   - ❌ `the-transaction-validation.md`
   - ❌ `a-guide-to-transaction-validation.md`

4. **Include extension**: Always use `.md` extension
   - ✅ `getting-started.md`
   - ❌ `getting-started`

### Non-Markdown Files
Non-markdown files should follow these conventions:

1. **Use kebab-case** for configuration files:
   - ✅ `mkdocs-config.yml`
   - ❌ `mkdocsConfig.yml`

2. **Use standard conventions** for specific file types:
   - ✅ `styles.css`
   - ✅ `main.js`

## Directory Naming

Directories must follow these conventions:

1. **Use kebab-case**: All lowercase with words separated by hyphens
   - ✅ `getting-started/`
   - ❌ `gettingStarted/`
   - ❌ `getting_started/`

2. **Be descriptive**: Directory names should clearly indicate content
   - ✅ `api-reference/`
   - ❌ `api/`
   - ❌ `refs/`

3. **Avoid deep nesting**: Aim for a maximum directory depth of 3-4 levels
   - ✅ `architecture/bitcoin/transaction-handling/`
   - ❌ `architecture/bitcoin/core/transaction/handling/validation/`

4. **Use singular form** for conceptual directories:
   - ✅ `architecture/`
   - ❌ `architectures/`

5. **Use plural form** for collections:
   - ✅ `guides/`
   - ❌ `guide/`

## Internal Links

When linking between documentation files:

1. **Use relative paths**:
   - ✅ `[Installation Guide](../getting-started/installation.md)`
   - ❌ `[Installation Guide](/docs/getting-started/installation.md)`

2. **Include file extension**:
   - ✅ `[API Reference](./api-reference.md)`
   - ❌ `[API Reference](./api-reference)`

3. **Link to specific sections** when appropriate:
   - ✅ `[Authentication Methods](./api-reference.md#authentication)`
   - ❌ `See the API Reference for authentication methods`

## Code References

When referencing code elements:

1. **Use backticks** for inline code references:
   - ✅ The `Transaction` struct is defined in `src/bitcoin/types.rs`
   - ❌ The Transaction struct is defined in src/bitcoin/types.rs

2. **Use fully qualified names** for functions and methods:
   - ✅ `bitcoin::transaction::validate_signature()`
   - ❌ `validate_signature()`

3. **Include module path** for types:
   - ✅ `bitcoin::types::Transaction`
   - ❌ `Transaction`

## Compliance Tags

Documentation should include appropriate compliance tags in frontmatter:

1. **Always include relevant tags**:
   ```yaml
   compliance: ["AIR-3", "AIS-3", "BPC-3"]
   ```

2. **Tag meanings**:
   - `AIR-3`: Anya Implementation Requirement Level 3
   - `AIS-3`: Anya Implementation Standard Level 3
   - `AIT-3`: Anya Implementation Test Level 3
   - `BPC-3`: Bitcoin Protocol Compliance Level 3
   - `RES-3`: Resource Efficiency Standard Level 3

## See Also
- [Documentation Standards](./documentation-standards.md)
- [Markdown Style Guide](../standards/markdown-style-guide.md)
- [File Organization](./file-organization.md)

---

*Last updated: 2025-08-06*
