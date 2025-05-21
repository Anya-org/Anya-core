# Table of Contents Generator

[AIR-3][AIS-3][BPC-3][RES-3]

## Overview

This script automatically generates or updates a table of contents (TOC) for Markdown files. It's designed to work with the Anya Core documentation system and follows the project's Markdown style guide.

## Features

- Automatically generates a TOC from Markdown headers
- Preserves YAML front matter
- Updates existing TOCs in place
- Handles multiple header levels
- Creates proper Markdown links with anchors
- Supports both in-place file updates and stdout output

## Usage

### Basic Usage

To generate a TOC and print it to stdout:

```bash
./scripts/generate_toc.sh path/to/your/file.md
```

### Update File In-Place

To update the TOC directly in the file:

```bash
./scripts/generate_toc.sh --in-place path/to/your/file.md
```

### Integration with Editors

You can integrate this script with your editor to automatically update TOCs when saving Markdown files.

#### VS Code

Add this to your `settings.json`:

```json
{
  "emeraldwalk.runonsave": {
    "commands": [
      {
        "match": "\\.md$",
        "cmd": "${workspaceFolder}/scripts/generate_toc.sh --in-place ${file}",
        "async": true
      }
    ]
  }
}
```

## How It Works

1. The script scans the Markdown file for headers (##, ###, etc.)
2. It generates a nested list of links to each header
3. For each header, it creates an anchor by:
   - Converting to lowercase
   - Replacing spaces with hyphens
   - Removing special characters
4. The TOC is inserted after the first top-level header (#) in the document

## TOC Format

The generated TOC follows this format:

```markdown
## Table of Contents

- [Section 1](#section-1)
  - [Subsection 1.1](#subsection-11)
  - [Subsection 1.2](#subsection-12)
- [Section 2](#section-2)
```

## Best Practices

1. **Update TOCs Before Committing**: Run the script before committing documentation changes
2. **Keep Headers Descriptive**: Use clear, descriptive headers for better TOC readability
3. **Check Anchor Links**: Verify that all links work correctly after generation
4. **Review Changes**: When using `--in-place`, review the changes before committing

## Dependencies

- Bash 4.0+
- GNU coreutils (sed, grep, etc.)

## License

This script is part of the Anya Core project and is licensed under the [MIT License](../LICENSE).
