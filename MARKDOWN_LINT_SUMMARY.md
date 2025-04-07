# Markdown Linting Summary

## Common Issues Found

After analyzing the codebase with markdownlint-cli2 v0.17.2, we identified the following common issues:

1. **MD022/blanks-around-headings** - Headings should have blank lines before and after them
2. **MD031/blanks-around-fences** - Code blocks should have blank lines before and after them
3. **MD032/blanks-around-lists** - Lists should have blank lines before and after them
4. **MD040/fenced-code-language** - Code blocks should specify a language
5. **MD047/single-trailing-newline** - Files should end with a single newline
6. **MD024/no-duplicate-heading** - Multiple headings with the same content
7. **MD029/ol-prefix** - Ordered list item prefix (numbering issues)

## Fix Approach

We implemented a multi-step approach to fix these issues:

1. Created an automated script `scripts/fix_markdown_lint.js` to address common issues
2. Manually fixed complex formatting issues in critical documentation files
3. Used `npx markdownlint-cli2 --fix` for selected files

## Automated Fix Script

The script provides a comprehensive solution to the most common markdown lint issues:

```javascript
#!/usr/bin/env node

/**
 * Markdown Lint Auto-Fix Script
 * 
 * This script automatically fixes common markdown lint issues:
 * - MD022: Headings should be surrounded by blank lines
 * - MD031: Fenced code blocks should be surrounded by blank lines
 * - MD032: Lists should be surrounded by blank lines
 * - MD047: Files should end with a single newline character
 * - MD040: Fenced code blocks should have a language specified
 */

const fs = require('fs').promises;
const path = require('path');
const globModule = require('glob');

// Create a promisified version of glob
function globPromise(pattern, options) {
  return new Promise((resolve, reject) => {
    globModule.glob(pattern, options, (err, files) => {
      if (err) {
        reject(err);
      } else {
        resolve(files);
      }
    });
  });
}

async function fixMarkdownFiles() {
  // Find markdown files
  const files = await globPromise(targetPattern, { ignore: 'node_modules/**' });
  
  for (const file of files) {
    let content = await fs.readFile(file, 'utf8');
    const originalContent = content;
    
    // Fix MD022: Blank lines around headings
    content = fixHeadings(content);
    
    // Fix MD031: Blank lines around code blocks
    content = fixCodeBlocks(content);
    
    // Fix MD032: Blank lines around lists
    content = fixLists(content);
    
    // Fix MD040: Add language to code blocks that don't have one
    content = fixCodeBlockLanguage(content);
    
    // Fix MD047: Ensure single trailing newline
    content = fixTrailingNewline(content);
    
    // Only write if changes were made
    if (content !== originalContent) {
      await fs.writeFile(file, content, 'utf8');
    }
  }
}
```

## Key Files Fixed

We prioritized fixing the most critical documentation files:

1. `docs/CLI_REFERENCE.md` - Fixed nested code blocks and added proper blank lines
2. `docs/BIP353_IMPLEMENTATION_GUIDE.md` - Added language specifiers to code blocks
3. `CHANGELOG.md` - Fixed duplicate heading issues by adding version identifiers

## Usage Guidelines

### Using the Fix Script

```bash
# Install required dependencies
npm install glob

# Run the script on selected files
node scripts/fix_markdown_lint.js "docs/*.md"

# Run the script on all markdown files in the repo
node scripts/fix_markdown_lint.js
```

### Using markdownlint-cli2 Directly

```bash
# Install markdownlint-cli2
npm install -g markdownlint-cli2

# Check a specific file
npx markdownlint-cli2 path/to/file.md

# Fix a specific file
npx markdownlint-cli2 --fix path/to/file.md

# Check multiple files
npx markdownlint-cli2 "**/*.md"
```

## Best Practices for Future Markdown Files

1. Always include blank lines before and after:
   - Headings
   - Code blocks
   - Lists
   - Tables

2. Always include a language specifier for code blocks:

   ```text
   # Using a plain code block without language is not recommended:
   ```

   Instead, specify the language:

   ```javascript
   // This is proper JavaScript code with language specified
   ```

3. Ensure files end with exactly one newline

4. Use unique headings or include additional context in duplicate headings

5. Use consistent ordered list numbering

## Configuration

The project uses `.markdownlint.json` to customize lint rules. Notable settings:

```json
{
  "MD013": false,
  "MD024": { "allow_different_nesting": true },
  "MD033": false
}
```

- `MD013: false` - Disables line length restrictions
- `MD024: { "allow_different_nesting": true }` - Allows headers with the same content at different nesting levels
- `MD033: false` - Allows inline HTML when needed

## Remaining Issues

Some files still have formatting issues that would require more extensive changes:

1. Complex document structures that would break if reformatted
2. Files with intentional markdown formatting for specific rendering requirements
3. Generated documentation files that would be overwritten

## Conclusion

By implementing these fixes, we've significantly improved the markdown quality across the codebase. The automated script provides a maintainable way to handle future formatting issues, and the established best practices will help ensure consistency moving forward. 
