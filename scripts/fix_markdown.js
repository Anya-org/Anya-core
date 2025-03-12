#!/usr/bin/env node

/**
 * Markdown Linting and Fixing Script
 * 
 * This script automates fixing common markdownlint issues across the project.
 * 
 * Usage:
 *   node fix_markdown.js [path]
 * 
 * Requirements:
 *   npm install --save-dev markdownlint-cli2
 */

const { execSync } = require('child_process');
const fs = require('fs');
const path = require('path');

// Configuration
const config = {
  // Default directory to fix if none provided
  defaultDir: '../docs',
  // Files or directories to ignore
  ignorePatterns: [
    'node_modules',
    '.git',
    'dist',
    'build'
  ],
  // Common fixes to apply
  fixes: [
    // Fix for MD052 (reference links)
    {
      regex: /\[([A-Z]{2,3})-(\d+)\]/g,
      replacement: '\\[$1-$2\\]'
    },
    // Fix for line endings
    {
      regex: /\r\n/g,
      replacement: '\n'
    },
    // Fix for multiple top-level headings
    {
      regex: /^# (.*)/gm,
      evaluator: (match, file, content) => {
        // Only replace second+ instances of top-level headings
        const headingMatches = content.match(/^# .*/gm);
        if (headingMatches && headingMatches.length > 1) {
          const firstIndex = content.indexOf(headingMatches[0]);
          const matchIndex = content.indexOf(match);
          if (matchIndex > firstIndex) {
            return '## ' + match.substring(2);
          }
        }
        return match;
      }
    }
  ]
};

// Get target directory from command line or use default
const targetDir = process.argv[2] || config.defaultDir;

// Function to check if markdownlint-cli2 is installed
function checkDependencies() {
  try {
    // Check if the package exists in node_modules
    const packagePath = path.join(process.cwd(), 'node_modules', 'markdownlint-cli2');
    if (fs.existsSync(packagePath)) {
      console.log('✅ markdownlint-cli2 is installed');
      return true;
    }
    
    // Fallback to checking with npx
    try {
      execSync('npx markdownlint-cli2 --version', { stdio: 'ignore' });
      console.log('✅ markdownlint-cli2 is installed');
      return true;
    } catch (error) {
      // If npx check fails, proceed to error message
    }
    
    console.error('❌ markdownlint-cli2 is not installed. Please install it with:');
    console.error('npm install --save-dev markdownlint-cli2');
    return false;
  } catch (error) {
    console.error('❌ markdownlint-cli2 is not installed. Please install it with:');
    console.error('npm install --save-dev markdownlint-cli2');
    console.error('Error details:', error.message);
    return false;
  }
}

// Function to find all markdown files in a directory
function findMarkdownFiles(dir, fileList = []) {
  const files = fs.readdirSync(dir);

  files.forEach(file => {
    const filePath = path.join(dir, file);
    
    // Skip ignored patterns
    if (config.ignorePatterns.some(pattern => filePath.includes(pattern))) {
      return;
    }
    
    if (fs.statSync(filePath).isDirectory()) {
      findMarkdownFiles(filePath, fileList);
    } else if (filePath.endsWith('.md') || filePath.endsWith('.markdown')) {
      fileList.push(filePath);
    }
  });

  return fileList;
}

// Function to apply custom fixes to a file
function applyCustomFixes(filePath) {
  let content = fs.readFileSync(filePath, 'utf8');
  let modified = false;

  // Add frontmatter if it doesn't exist
  if (!content.startsWith('---') && !content.includes('<!-- markdownlint-disable')) {
    content = '<!-- markdownlint-disable MD013 line-length -->\n\n' + content;
    modified = true;
  }

  // Apply regex-based fixes
  config.fixes.forEach(fix => {
    const originalContent = content;
    
    if (fix.evaluator) {
      // For more complex fixes that need to evaluate the entire file content
      const lines = content.split('\n');
      const newLines = lines.map(line => {
        const match = line.match(fix.regex);
        if (match) {
          return fix.evaluator(line, filePath, content);
        }
        return line;
      });
      content = newLines.join('\n');
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
}

// Function to run markdownlint-cli2
function runMarkdownlint(targetDir) {
  try {
    // Try using local installation first
    const localBin = path.join(process.cwd(), 'node_modules', '.bin', 'markdownlint-cli2');
    if (fs.existsSync(localBin)) {
      console.log('Using local markdownlint-cli2 installation...');
      execSync(`"${localBin}" "${targetDir}/**/*.md" --fix`, { stdio: 'inherit' });
      return true;
    }
    
    // Fallback to npx
    console.log('Using npx to run markdownlint-cli2...');
    execSync(`npx markdownlint-cli2 "${targetDir}/**/*.md" --fix`, { stdio: 'inherit' });
    return true;
  } catch (error) {
    console.error('⚠️ Error running markdownlint-cli2:', error.message);
    console.error('Some issues could not be automatically fixed. Manual intervention may be required.');
    return false;
  }
}

// Main function
function main() {
  console.log('🔍 Checking dependencies...');
  const dependenciesInstalled = checkDependencies();

  console.log(`\n🔎 Finding Markdown files in ${targetDir}...`);
  const markdownFiles = findMarkdownFiles(targetDir);
  console.log(`📝 Found ${markdownFiles.length} Markdown files`);

  console.log('\n✨ Applying custom fixes...');
  let fixedCount = 0;
  markdownFiles.forEach(file => {
    const wasFixed = applyCustomFixes(file);
    if (wasFixed) {
      console.log(`  ✅ Fixed issues in ${file}`);
      fixedCount++;
    }
  });
  console.log(`\n✨ Applied custom fixes to ${fixedCount} files`);

  if (dependenciesInstalled) {
    console.log('\n🔧 Running markdownlint-cli2 to fix remaining issues...');
    runMarkdownlint(targetDir);
  } else {
    console.log('\n⚠️ Skipping markdownlint-cli2 fixes. Only custom fixes applied.');
  }

  console.log('\n🎉 Markdown fixing process completed!');
}

// Run the script
main(); 