---
title: "Markdown_style_guide"
description: "Documentation for Markdown_style_guide"
---

# Markdown Style Guide [AIR-3][AIS-3][BPC-3][RES-3]

## Overview

Add a brief overview of this document here.

## Table of Contents

- [Section 1](#section-1)
- [Section 2](#section-2)


This guide outlines the standards for writing Markdown documentation in the Anya Core project.

## General Guidelines

### Line Length

- Keep lines to a maximum of 100 characters
- Break long lines for better readability and version control

### File Naming

- Use lowercase with hyphens (kebab-case) for file names
- Examples: `getting-started.md`, `api-reference.md`

### Headers

- Use ATX-style headers with `#`
- Put blank lines before and after headers
- Capitalize all words except articles and prepositions

```markdown
# Document Title

## Section Header

### Subsection Header
```

## Text Formatting

### Emphasis

- Use `**bold**` for strong emphasis
- Use `*italic*` for emphasis
- Use `code` for file names, paths, and commands

### Lists

- Use hyphens for unordered lists
- Use numbers for ordered lists
- Indent nested lists with 4 spaces

```markdown
- First item
- Second item
    - Nested item
    - Another nested item
- Third item
```

### Links

- Use descriptive link text
- Place links at the end of the document when they are references

```markdown
[descriptive text](url)
```

## Code Blocks

### Inline Code

- Use backticks for `code` in text
- Escape backticks inside code with double backticks

### Fenced Code Blocks

- Use triple backticks with language specification
- Include a blank line before and after code blocks
- Keep code blocks concise and focused

```rust
fn main() {
    println!("Hello, world!");
}
```

## Tables

- Use pipes to separate columns
- Include a header row with dashes
- Align columns with colons

```markdown
| Header 1 | Header 2 |
|----------|----------|
| Cell 1   | Cell 2   |
| Cell 3   | Cell 4   |
```

## Images

- Use descriptive alt text
- Place images in the `docs/assets/images/` directory
- Specify width if needed

```markdown
![Alt text](assets/images/filename.png)
```

## Metadata

Each document should start with YAML front matter:

```yaml
---
title: Page Title
description: Brief description of the page
---
```

## AI Labeling

Include the following AI labels at the top of each file:

```markdown
[AIR-3][AIS-3][BPC-3][RES-3]
```

## Best Practices

1. **Be concise** - Get to the point quickly
2. **Be consistent** - Follow existing patterns
3. **Be complete** - Include all necessary information
4. **Be accurate** - Keep documentation up to date
5. **Be organized** - Use clear structure and navigation

## Linting

All documentation is linted using markdownlint with the following rules:

- MD009 - Trailing spaces
- MD012 - Multiple consecutive blank lines
- MD013 - Line length (100 characters)
- MD022 - Headers should be surrounded by blank lines
- MD031 - Fenced code blocks should be surrounded by blank lines
- MD033 - Inline HTML
- MD040 - Fenced code blocks should have a language specified

## Review Process

1. Create a pull request with your changes
2. Ensure all CI checks pass
3. Request review from at least one team member
4. Address any feedback
5. Merge when approved

## Resources

- [CommonMark Spec](https://spec.commonmark.org/)
- [GitHub Flavored Markdown](https://github.github.com/gfm/)
- [Markdown Guide](https://www.markdownguide.org/)

## See Also

- [Related Document](#related-document)

