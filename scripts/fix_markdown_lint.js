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
  console.log(`Using glob pattern: ${pattern}`);
  console.log('Glob options:', options);
  
  return new Promise((resolve, reject) => {
    globModule.glob(pattern, options, (err, files) => {
      if (err) {
        console.error('Glob error:', err);
        reject(err);
      } else {
        console.log('Glob result:', files);
        resolve(files);
      }
    });
  });
}

// Default to all markdown files if no arguments provided
const targetPattern = process.argv[2] || "**/*.md";

async function fixMarkdownFiles() {
  console.log(`Searching for markdown files matching pattern: ${targetPattern}`);
  
  try {
    // Get current working directory
    const cwd = process.cwd();
    console.log(`Current working directory: ${cwd}`);
    
    // Try to read the directory
    const dirContents = await fs.readdir(cwd);
    console.log('Directory contents:', dirContents);
    
    // Find all markdown files
    const files = await globPromise(targetPattern, { ignore: 'node_modules/**' });
    console.log(`Found ${files.length} markdown files`);

    if (files.length === 0) {
      console.log('No files found. Trying a simpler approach...');
      
      // Try a different approach - read CLI_REFERENCE.md directly
      try {
        const cliRefPath = path.join(cwd, 'docs', 'CLI_REFERENCE.md');
        console.log(`Trying to read file directly: ${cliRefPath}`);
        
        const stats = await fs.stat(cliRefPath);
        console.log('File stats:', stats);
        
        if (stats.isFile()) {
          // Process this file directly
          await processFile(cliRefPath);
        }
      } catch (fileErr) {
        console.error('Error accessing file directly:', fileErr);
      }
      
      return;
    }

    // Detect duplicate headings and generate report
    console.log('Analyzing files for duplicate headings...');
    const duplicateReport = await detectDuplicateHeadings(files);
    await generateDuplicateHeadingReport(duplicateReport);
    
    let fixedFiles = 0;
    
    for (const file of files) {
      await processFile(file);
      fixedFiles++;
    }
    
    console.log(`\nCompleted! Fixed issues in ${fixedFiles} files.`);
    
  } catch (error) {
    console.error('Error:', error);
  }
}

async function processFile(file) {
  console.log(`Processing file: ${file}`);
  
  try {
    let content = await fs.readFile(file, 'utf8');
    console.log(`Read file content: ${content.length} bytes`);
    
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
      console.log(`Fixed issues in: ${file}`);
      return true;
    } else {
      console.log(`No changes needed for: ${file}`);
      return false;
    }
  } catch (err) {
    console.error(`Error processing file ${file}:`, err);
    return false;
  }
}

function fixHeadings(content) {
  // Regular expression to find headings
  const headingRegex = /^(#{1,6})\s+(.+?)$/gm;
  
  // Add blank lines around headings if they don't exist
  return content.replace(headingRegex, (match, hashes, text) => {
    const prevChar = content.substring(content.indexOf(match) - 1, content.indexOf(match));
    const nextChar = content.substring(content.indexOf(match) + match.length, content.indexOf(match) + match.length + 1);
    
    const needsPrevLine = prevChar !== '\n' && prevChar !== '';
    const needsNextLine = nextChar !== '\n' && nextChar !== '';
    
    if (needsPrevLine && needsNextLine) {
      return `\n${hashes} ${text}\n`;
    } else if (needsPrevLine) {
      return `\n${hashes} ${text}`;
    } else if (needsNextLine) {
      return `${hashes} ${text}\n`;
    }
    
    return match;
  });
}

function fixCodeBlocks(content) {
  // Regular expression to find code blocks
  const codeBlockRegex = /^```(?:[a-zA-Z0-9]+)?\s*$/gm;
  
  // Track code block state
  let inCodeBlock = false;
  let newContent = [];
  let lines = content.split('\n');
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const isCodeBlockMarker = line.match(/^```(?:[a-zA-Z0-9]+)?\s*$/);
    
    if (isCodeBlockMarker) {
      if (!inCodeBlock) {
        // Opening code block
        inCodeBlock = true;
        
        // Add blank line before code block if needed
        if (i > 0 && lines[i-1].trim() !== '') {
          newContent.push('');
        }
        
        newContent.push(line);
      } else {
        // Closing code block
        inCodeBlock = false;
        newContent.push(line);
        
        // Add blank line after code block if needed
        if (i < lines.length - 1 && lines[i+1].trim() !== '') {
          newContent.push('');
        }
      }
    } else {
      newContent.push(line);
    }
  }
  
  return newContent.join('\n');
}

function fixLists(content) {
  // Regular expressions for lists
  const unorderedListRegex = /^([\s]*)[-*+][\s].+$/gm;
  const orderedListRegex = /^([\s]*)\d+\.[\s].+$/gm;
  
  // Process content line by line to handle lists
  let lines = content.split('\n');
  let newContent = [];
  
  for (let i = 0; i < lines.length; i++) {
    const line = lines[i];
    const isUnorderedListItem = line.match(unorderedListRegex);
    const isOrderedListItem = line.match(orderedListRegex);
    
    if (isUnorderedListItem || isOrderedListItem) {
      const indentLevel = (isUnorderedListItem ? isUnorderedListItem[1] : isOrderedListItem[1]).length;
      
      // Only add blank line before top-level list items
      if (indentLevel === 0) {
        const prevLine = i > 0 ? lines[i-1] : '';
        
        // Add blank line before list if needed
        if (i > 0 && prevLine.trim() !== '' && 
            !prevLine.match(unorderedListRegex) && 
            !prevLine.match(orderedListRegex)) {
          newContent.push('');
        }
      }
    }
    
    newContent.push(line);
    
    if (isUnorderedListItem || isOrderedListItem) {
      const indentLevel = (isUnorderedListItem ? isUnorderedListItem[1] : isOrderedListItem[1]).length;
      
      // Only add blank line after the last list item at this level
      if (indentLevel === 0) {
        const nextLine = i < lines.length - 1 ? lines[i+1] : '';
        
        // Add blank line after list if needed
        if (i < lines.length - 1 && 
            nextLine.trim() !== '' && 
            !nextLine.match(unorderedListRegex) && 
            !nextLine.match(orderedListRegex)) {
          newContent.push('');
        }
      }
    }
  }
  
  return newContent.join('\n');
}

function fixCodeBlockLanguage(content) {
  // Find code blocks without language specification and add 'text'
  return content.replace(/```\s*\n/g, '```text\n');
}

function fixTrailingNewline(content) {
  // Ensure file ends with exactly one newline
  return content.replace(/\n*$/, '\n');
}

/**
 * Detects duplicate headings across all markdown files and suggests fixes
 * @param {Array<string>} files Array of file paths to scan
 * @returns {Object} Report of duplicate headings with suggested fixes
 */
async function detectDuplicateHeadings(files) {
  // Map to store all headings: { headingText: [{ file, lineNumber, headingLevel, context }] }
  const headingMap = {};
  const duplicateReport = { duplicates: {}, suggestions: {} };
  
  for (const file of files) {
    try {
      const content = await fs.readFile(file, 'utf8');
      const lines = content.split('\n');
      const fileName = path.basename(file, '.md');
      const fileDir = path.dirname(file).split(path.sep).pop();
      
      // Extract headings using regex
      const headingRegex = /^(#{1,6})\s+(.+?)$/gm;
      let match;
      
      while ((match = headingRegex.exec(content)) !== null) {
        const headingLevel = match[1].length;
        const headingText = match[2].trim();
        const lineNumber = content.substring(0, match.index).split('\n').length;
        
        // Get context (file name, directory, surrounding content)
        const prevLine = lineNumber > 1 ? lines[lineNumber - 2] : '';
        const nextLine = lineNumber < lines.length ? lines[lineNumber] : '';
        const context = {
          fileName,
          fileDir,
          prevLine: prevLine.trim(),
          nextLine: nextLine.trim()
        };
        
        // Add to heading map
        if (!headingMap[headingText]) {
          headingMap[headingText] = [];
        }
        
        headingMap[headingText].push({
          file,
          lineNumber,
          headingLevel,
          context
        });
      }
    } catch (err) {
      console.error(`Error reading file ${file}:`, err);
    }
  }
  
  // Find duplicates and generate suggestions
  for (const [headingText, occurrences] of Object.entries(headingMap)) {
    if (occurrences.length > 1) {
      duplicateReport.duplicates[headingText] = occurrences;
      
      // Generate suggestions based on context
      duplicateReport.suggestions[headingText] = occurrences.map(occurrence => {
        const { file, context } = occurrence;
        
        // Different suggestion strategies based on file context
        let suggestion;
        
        // 1. Try to use file name in heading
        if (!headingText.includes(context.fileName)) {
          suggestion = `${headingText} (${context.fileName})`;
        } 
        // 2. Try to use directory name if not already in heading
        else if (!headingText.includes(context.fileDir) && context.fileDir !== '.') {
          suggestion = `${headingText} (${context.fileDir})`;
        }
        // 3. Use surrounding content for clues
        else if (context.prevLine && !context.prevLine.startsWith('#')) {
          // Extract keywords from surrounding content
          const keywords = extractKeywords(context.prevLine);
          if (keywords.length > 0) {
            suggestion = `${headingText} (${keywords[0]})`;
          }
        }
        // 4. For changelog or version-related files
        else if (file.toLowerCase().includes('changelog')) {
          suggestion = `${headingText} in v${extractVersionNumber(file) || 'x.y.z'}`;
        }
        // 5. Default fallback
        else {
          suggestion = `${headingText} (${path.basename(file, '.md')})`;
        }
        
        return {
          file,
          original: headingText,
          suggested: suggestion
        };
      });
    }
  }
  
  return duplicateReport;
}

/**
 * Extract potential keywords from text for contextual suggestions
 * @param {string} text Text to analyze
 * @returns {Array<string>} Keywords extracted from text
 */
function extractKeywords(text) {
  // Remove common words and extract potential keywords
  const words = text.split(/\s+/).filter(word => word.length > 3);
  const stopWords = ['this', 'that', 'with', 'from', 'have', 'which', 'such', 'they', 'their'];
  
  return words
    .filter(word => !stopWords.includes(word.toLowerCase()))
    .slice(0, 3); // Take top 3 keywords
}

/**
 * Extract version number from file path or content
 * @param {string} filePath Path to the file
 * @returns {string|null} Extracted version number or null
 */
function extractVersionNumber(filePath) {
  // Try to find version in file name
  const versionMatch = filePath.match(/v?(\d+\.\d+(\.\d+)?)/i);
  return versionMatch ? versionMatch[1] : null;
}

/**
 * Generate a report file with duplicate headings and suggested fixes
 * @param {Object} report Duplicate heading report
 */
async function generateDuplicateHeadingReport(report) {
  let reportContent = '# Duplicate Headings Report\n\n';
  
  if (Object.keys(report.duplicates).length === 0) {
    reportContent += 'No duplicate headings found across files.\n';
    return;
  }
  
  reportContent += '## Detected Duplicates\n\n';
  
  for (const [heading, occurrences] of Object.entries(report.duplicates)) {
    reportContent += `### "${heading}"\n\n`;
    reportContent += 'Found in:\n\n';
    
    occurrences.forEach(occurrence => {
      reportContent += `- \`${occurrence.file}\` (line ${occurrence.lineNumber})\n`;
    });
    
    reportContent += '\nSuggested fixes:\n\n';
    
    report.suggestions[heading].forEach(suggestion => {
      reportContent += `- In \`${suggestion.file}\`: Change to "${suggestion.suggested}"\n`;
    });
    
    reportContent += '\n';
  }
  
  // Add to the usage instructions section
  reportContent += '## How to Apply Fixes\n\n';
  reportContent += 'You can apply these fixes manually, or extend the fix_markdown_lint.js script to handle them.\n\n';
  reportContent += '```javascript\n';
  reportContent += '// Example implementation:\n';
  reportContent += 'function fixDuplicateHeadings(content, file, duplicateReport) {\n';
  reportContent += '  // For each heading in the duplicate report\n';
  reportContent += '  for (const [heading, suggestions] of Object.entries(duplicateReport.suggestions)) {\n';
  reportContent += '    // Find the suggestion for this file\n';
  reportContent += '    const suggestion = suggestions.find(s => s.file === file);\n';
  reportContent += '    if (suggestion) {\n';
  reportContent += '      // Replace the heading with the suggested one\n';
  reportContent += '      const headingRegex = new RegExp(`^(#{1,6})\\\\s+${escapeRegExp(heading)}$`, "gm");\n';
  reportContent += '      content = content.replace(headingRegex, `$1 ${suggestion.suggested}`);\n';
  reportContent += '    }\n';
  reportContent += '  }\n';
  reportContent += '  return content;\n';
  reportContent += '}\n';
  reportContent += '```\n';
  
  await fs.writeFile('DUPLICATE_HEADINGS_REPORT.md', reportContent, 'utf8');
  console.log('\nDuplicate headings report generated: DUPLICATE_HEADINGS_REPORT.md');
}

// Run the script
fixMarkdownFiles();
