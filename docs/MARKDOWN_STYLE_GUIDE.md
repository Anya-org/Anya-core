---
title: "Markdown_style_guide"
description: "Documentation for Markdown_style_guide"
last_updated: 2025-05-21
---

# Markdown Style Guide \[AIR-1\]\[AIT-1\]

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


<!-- markdownlint-disable MD013 line-length -->

## Overview

This guide defines the standard markdown formatting rules for all documentation in the Anya Core project. Following these guidelines ensures consistency across our documentation and prevents common markdownlint issues.

## General Rules

1. **File Names**: Use lowercase with underscores for spaces (e.g., `getting_started.md`, not `Getting Started.md`)
2. **Line Length**: No line length restrictions (handled by markdownlint config)
3. **Line Endings**: Use LF (`\n`), not CRLF (`\r\n`)
4. **File Encoding**: UTF-8 without BOM
5. **Final Newline**: Include a final newline at the end of each file

## Headers

1. **Single H1**: Each document should have exactly one H1 header at the top
2. **No Skipping Levels**: Don't skip header levels (e.g., H2 should follow H1, not H3)
3. **Spacing**: Include a blank line before and after headers
4. **Capitalization**: Use title case for headers (e.g., "Getting Started Guide" not "Getting started guide")
5. **AI Labelling**: Escape AI labelling tags in headers with backslashes: `# Component Name \[AIR-3\]\[AIS-3\]`

```markdown
## Document Title \[AIR-3\]\[AIS-3\]

## Section Title

### Subsection Title
```

## Lists

1. **Indentation**: Indent nested lists with 2 or 4 spaces
2. **Spacing**: Include a blank line before and after lists
3. **Consistency**: Use either all ordered (`1.`) or all unordered (`-`) for the same level

```markdown
- First item
- Second item
  - Nested item 1
  - Nested item 2
- Third item
```

## Code Blocks

1. **Fenced Code Blocks**: Use triple backticks with language specification
2. **Indentation**: Don't indent code blocks with spaces
3. **Syntax Highlighting**: Always specify the language for syntax highlighting

```markdown
​```rust
fn main() {
    println!("Hello, world!");
}
​```
```

## Links and Images

1. **Link Text**: Use descriptive link text, not "click here" or URLs
2. **Image Alt Text**: Always include descriptive alt text for images
3. **Relative Links**: Use relative links for internal documentation

```markdown
[API Documentation](../api/README.md)
![Architecture Diagram](../images/architecture.png "System Architecture")
```

## Tables

1. **Headers**: Always include a header row
2. **Alignment**: Use colons to specify column alignment (`:---` left, `:---:` center, `---:` right)
3. **Spacing**: Include a blank line before and after tables

```markdown
| Name | Type | Description |
|------|:----:|------------:|
| id | string | Unique identifier |
| count | number | Number of items |
```

## AI Labelling Tags

1. **Escaping**: Always escape AI labelling tags to prevent markdownlint errors
2. **Position**: Place AI labelling tags after the title/header
3. **Format**: Follow the AI Labelling Guide format: `\[CATEGORY-LEVEL\]`

```markdown
## Component Name \[AIR-3\]\[AIS-3\]

This component provides... \[AIT-2\]
```

## Comments

1. **HTML Comments**: Use HTML comments for notes that shouldn't appear in rendered markdown
2. **Markdownlint Directives**: Place markdownlint directives in HTML comments at the top of the file

```markdown
<!-- markdownlint-disable MD013 line-length -->
<!-- TODO: Add more examples to this section -->
```

## Frontmatter

1. **Format**: Use YAML format between triple-dash lines
2. **Required Fields**: Include at least `title` and `date` fields
3. **Position**: Place frontmatter at the very beginning of the file

```markdown
---
title: "Markdown_style_guide"
date: 2025-03-12
author: Anya Documentation Team
last_updated: 2025-05-21
---
[AIR-3][AIS-3][BPC-3][RES-3]

```

## Admonitions

Use the following format for admonitions (notes, warnings, etc.):

```markdown
> **Note:** This is important information that should be highlighted.

> **Warning:** This warns about potential issues or dangers.

> **Tip:** This provides helpful advice for better usage.
```

## Automated Linting

We use markdownlint to enforce these guidelines. The configuration is in `.markdownlint.json` at the project root.

To automatically fix common issues:

```bash
## Install markdownlint-cli2 if not already installed
npm install --save-dev markdownlint-cli2

## Run the fixing script
node scripts/fix_markdown.js docs
```

## Last Updated

2025-03-12 
## See Also

- [Related Document 1](./related1.md)
- [Related Document 2](./related2.md)
