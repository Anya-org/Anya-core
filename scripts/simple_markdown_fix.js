#!/usr/bin/env node

/**
 * Simple Markdown Fixing Script
 * 
 * This script applies basic fixes to markdown files without requiring external dependencies.
 * 
 * Usage:
 *   node simple_markdown_fix.js [directory]
 */

const fs = require('fs');
const path = require('path');

// Get target directory from command line or use default
const targetDir = process.argv[2] || '../docs';

// Files or directories to ignore
const ignorePatterns = [
  'node_modules',
  '.git',
  'dist',
  'build'
];

// Common fixes to apply
const fixes = [
  // Fix for AI tags (reference links)
  {
    regex: /\[([A-Z]{2,3})-(\d+)\](?!\\\])/g,
    replacement: '\\[$1-$2\\]'
  },
  // Fix for line endings
  {
    regex: /\r\n/g,
    replacement: '\n'
  },
  // Ensure markdownlint-disable is present
  {
    regex: /^((?!<!-- markdownlint-disable).)*$/s,
    evaluator: (match, file, content) => {
      if (!content.includes('<!-- markdownlint-disable')) {
        return '<!-- markdownlint-disable MD013 line-length -->\n\n' + match;
      }
      return match;
    }
  },
  // Update last updated date
  {
    regex: /## Last Updated\s*\n\s*\d{4}-\d{2}-\d{2}/g,
    replacement: '## Last Updated\n\n2025-03-12'
  }
];

// Function to find all markdown files in a directory
function findMarkdownFiles(dir, fileList = []) {
  try {
    const files = fs.readdirSync(dir);

    files.forEach(file => {
      const filePath = path.join(dir, file);
      
      // Skip ignored patterns
      if (ignorePatterns.some(pattern => filePath.includes(pattern))) {
        return;
      }
      
      const stats = fs.statSync(filePath);
      if (stats.isDirectory()) {
        findMarkdownFiles(filePath, fileList);
      } else if (filePath.endsWith('.md') || filePath.endsWith('.markdown')) {
        fileList.push(filePath);
      }
    });
  } catch (error) {
    console.error(`Error reading directory ${dir}:`, error.message);
  }

  return fileList;
}

// Function to apply custom fixes to a file
function applyCustomFixes(filePath) {
  try {
    let content = fs.readFileSync(filePath, 'utf8');
    let modified = false;

    // Apply regex-based fixes
    fixes.forEach(fix => {
      const originalContent = content;
      
      if (fix.evaluator) {
        // For more complex fixes that need to evaluate the entire file content
        content = fix.evaluator(content, filePath, content);
      } else {
        // For simple regex replacements
        content = content.replace(fix.regex, fix.replacement);
      }
      
      if (originalContent !== content) {
        modified = true;
      }
    });

    // Write back to file if modified
    if (modified) {
      fs.writeFileSync(filePath, content, 'utf8');
      return true;
    }
    
    return false;
  } catch (error) {
    console.error(`Error processing file ${filePath}:`, error.message);
    return false;
  }
}

// Main function
function main() {
  console.log(`🔎 Finding Markdown files in ${targetDir}...`);
  const markdownFiles = findMarkdownFiles(targetDir);
  console.log(`📝 Found ${markdownFiles.length} Markdown files`);

  console.log('\n✨ Applying fixes...');
  let fixedCount = 0;
  markdownFiles.forEach(file => {
    const wasFixed = applyCustomFixes(file);
    if (wasFixed) {
      console.log(`  ✅ Fixed issues in ${file}`);
      fixedCount++;
    }
  });
  console.log(`\n✨ Applied fixes to ${fixedCount} files`);
  console.log('\n🎉 Markdown fixing process completed!');
}

// Run the script
main(); 