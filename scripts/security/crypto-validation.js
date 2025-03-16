#!/usr/bin/env node
/**
 * Cryptographic Implementation Validator
 * [AIR-3][AIS-3][BPC-3][AIT-3][RES-3]
 * 
 * This script validates cryptographic implementations according to
 * Bitcoin Development Framework v2.5 standards and security best practices.
 */

const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const glob = require('glob');

// Define critical crypto files to check
const CRITICAL_FILES = [
  'scripts/bitcoin/mcp-server.js',
  'src/bitcoin/taproot.rs',
  'src/bitcoin/protocol.rs',
  'src/bitcoin/validation.rs',
  'src/bitcoin/dlc/**/*.rs'
];

// Define crypto security checks
const CRYPTO_CHECKS = [
  {
    name: 'Secure RNG Usage',
    description: 'Check for proper use of cryptographically secure random number generation',
    check: (filePath, content) => {
      const fileExt = path.extname(filePath);
      
      if (fileExt === '.js') {
        // JavaScript checks
        const hasSecureRandom = content.includes('crypto.randomBytes');
        const hasInsecureRandom = content.includes('Math.random()');
        
        return {
          passed: hasSecureRandom && !hasInsecureRandom,
          details: hasInsecureRandom ? 
            'Using insecure Math.random() instead of crypto.randomBytes()' : 
            (hasSecureRandom ? 'Using secure RNG' : 'No RNG usage detected')
        };
      } else if (fileExt === '.rs') {
        // Rust checks
        const hasSecureRandom = content.includes('OsRng') || 
                               content.includes('rand::thread_rng()') ||
                               content.includes('rand::rngs::ThreadRng');
        const hasInsecureRandom = content.includes('rand::Rng::gen');
        
        return {
          passed: hasSecureRandom && !hasInsecureRandom,
          details: hasInsecureRandom && !hasSecureRandom ? 
            'Using potentially insecure RNG instead of OsRng' : 
            (hasSecureRandom ? 'Using secure RNG' : 'No RNG usage detected')
        };
      }
      
      return { passed: true, details: 'File type not analyzed for RNG usage' };
    }
  },
  {
    name: 'Constant-Time Operations',
    description: 'Check for constant-time operations for cryptographic comparisons',
    check: (filePath, content) => {
      const fileExt = path.extname(filePath);
      
      if (fileExt === '.js') {
        // JavaScript checks
        const hasConstantTimeComparison = content.includes('crypto.timingSafeEqual') ||
                                         content.includes('constant-time');
        const hasRegularComparison = content.includes('===') || 
                                    content.includes('==') || 
                                    content.includes('equals(');
        
        // Only flag if we detect both cryptographic operations and regular comparisons
        const hasCryptoOps = content.includes('sign') || 
                            content.includes('verify') || 
                            content.includes('hash');
        
        return {
          passed: !hasCryptoOps || hasConstantTimeComparison,
          details: hasCryptoOps && !hasConstantTimeComparison ? 
            'Cryptographic operations detected without constant-time comparisons' : 
            'Constant-time operations used or not needed'
        };
      } else if (fileExt === '.rs') {
        // Rust checks
        const hasConstantTimeComparison = content.includes('ct_eq') || 
                                        content.includes('constant_time') ||
                                        content.includes('ConstantTimeEq');
        
        // Look for crypto operations combined with standard equality
        const hasCryptoOps = content.includes('sign') || 
                           content.includes('verify') || 
                           content.includes('hash');
        const hasRegularComparison = content.includes('==') || 
                                   content.includes('eq(');
        
        return {
          passed: !hasCryptoOps || !hasRegularComparison || hasConstantTimeComparison,
          details: hasCryptoOps && hasRegularComparison && !hasConstantTimeComparison ? 
            'Crypto operations with non-constant-time comparisons' : 
            'Constant-time operations used or not needed'
        };
      }
      
      return { passed: true, details: 'File type not analyzed for constant-time operations' };
    }
  },
  {
    name: 'Key Size Security',
    description: 'Check for appropriate key sizes in cryptographic operations',
    check: (filePath, content) => {
      // Look for key size declarations
      const keySizes = [];
      
      // Extract key sizes for analysis
      const keyBitPattern = /(\d+)\s*(?:bit|bytes)/g;
      let match;
      while ((match = keyBitPattern.exec(content)) !== null) {
        keySizes.push(parseInt(match[1], 10));
      }
      
      // Also check for explicit small numbers that might be key sizes
      if (content.includes('16 bytes') || content.includes('128 bit') || 
          content.includes('size: 16') || content.includes('length: 16')) {
        keySizes.push(128);
      }
      
      const tooSmallSizes = keySizes.filter(size => {
        // Convert bytes to bits if necessary
        const sizeInBits = size <= 64 ? size * 8 : size;
        return sizeInBits < 256; // 256 bits minimum for modern crypto
      });
      
      return {
        passed: tooSmallSizes.length === 0,
        details: tooSmallSizes.length > 0 ? 
          `Found potentially insecure key sizes: ${tooSmallSizes.join(', ')} bits` : 
          'No insecure key sizes detected'
      };
    }
  },
  {
    name: 'Modern Cryptographic Algorithms',
    description: 'Check for usage of modern, secure cryptographic algorithms',
    check: (filePath, content) => {
      // Define insecure algorithms
      const insecureAlgorithms = [
        'MD5', 'SHA1', 'RC4', 'DES', '3DES', 'ECB'
      ];
      
      // Define secure algorithms to look for
      const secureAlgorithms = [
        'SHA256', 'SHA3', 'AES-GCM', 'ChaCha20', 'Poly1305', 'HMAC'
      ];
      
      // Check for usage
      const usedInsecureAlgorithms = insecureAlgorithms.filter(algo => 
        content.includes(algo.toLowerCase()) || content.includes(algo)
      );
      
      const usedSecureAlgorithms = secureAlgorithms.filter(algo => 
        content.includes(algo.toLowerCase()) || content.includes(algo)
      );
      
      return {
        passed: usedInsecureAlgorithms.length === 0,
        details: usedInsecureAlgorithms.length > 0 ? 
          `Using insecure algorithms: ${usedInsecureAlgorithms.join(', ')}` : 
          (usedSecureAlgorithms.length > 0 ? 
            `Using secure algorithms: ${usedSecureAlgorithms.join(', ')}` : 
            'No explicit cryptographic algorithms detected')
      };
    }
  },
  {
    name: 'Proper Error Handling',
    description: 'Check for proper error handling in cryptographic operations',
    check: (filePath, content) => {
      const fileExt = path.extname(filePath);
      
      if (fileExt === '.js') {
        // JavaScript checks
        const hasTryCatch = content.includes('try {') && content.includes('catch');
        const hasCryptoOps = content.includes('sign') || 
                           content.includes('verify') || 
                           content.includes('hash') || 
                           content.includes('encrypt') || 
                           content.includes('decrypt');
        
        return {
          passed: !hasCryptoOps || hasTryCatch,
          details: hasCryptoOps && !hasTryCatch ? 
            'Cryptographic operations without error handling' : 
            'Proper error handling in place or not needed'
        };
      } else if (fileExt === '.rs') {
        // Rust checks
        const hasResultHandling = content.includes('Result<') && 
                               (content.includes('match') || content.includes('?') || 
                                content.includes('.unwrap_or') || content.includes('.map_err'));
        
        const hasCryptoOps = content.includes('sign') || 
                          content.includes('verify') || 
                          content.includes('hash') || 
                          content.includes('encrypt') || 
                          content.includes('decrypt');
        
        return {
          passed: !hasCryptoOps || hasResultHandling,
          details: hasCryptoOps && !hasResultHandling ? 
            'Cryptographic operations without proper Result handling' : 
            'Proper error handling in place or not needed'
        };
      }
      
      return { passed: true, details: 'File type not analyzed for error handling' };
    }
  },
  {
    name: 'Hardcoded Secrets',
    description: 'Check for hardcoded cryptographic secrets or keys',
    check: (filePath, content) => {
      // Look for patterns that might indicate hardcoded secrets
      const secretPatterns = [
        /['"]([0-9a-fA-F]{32,})['"]/, // Hex strings of 32+ chars
        /private[-_\s]*key\s*[:=]\s*['"][^'"]+['"]/, // private key assignments
        /secret\s*[:=]\s*['"][^'"]+['"]/, // secret assignments
        /password\s*[:=]\s*['"][^'"]+['"]/ // password assignments
      ];
      
      // Skip test files
      if (filePath.includes('test') || filePath.includes('mock')) {
        return { passed: true, details: 'Test file, skipping hardcoded secret check' };
      }
      
      for (const pattern of secretPatterns) {
        if (pattern.test(content)) {
          return {
            passed: false,
            details: 'Potential hardcoded secret or key detected'
          };
        }
      }
      
      return { passed: true, details: 'No hardcoded secrets detected' };
    }
  }
];

// Banner
console.log('\n===== Cryptographic Implementation Validation =====');
console.log('Bitcoin Development Framework v2.5');
console.log('Checking for secure cryptographic implementations...\n');

// Find all relevant files
let filesToCheck = [];
for (const pattern of CRITICAL_FILES) {
  try {
    const matched = glob.sync(pattern);
    filesToCheck = [...filesToCheck, ...matched];
  } catch (err) {
    console.error(`Error while globbing ${pattern}: ${err}`);
  }
}

filesToCheck = [...new Set(filesToCheck)]; // Remove duplicates

if (filesToCheck.length === 0) {
  console.error('No critical crypto files found to validate');
  process.exit(1);
}

console.log(`Found ${filesToCheck.length} files to check:\n`);

// Track validation results
const results = {
  timestamp: new Date().toISOString(),
  overallResult: true,
  filesChecked: filesToCheck.length,
  fileResults: [],
  issues: []
};

// Process each file
for (const filePath of filesToCheck) {
  console.log(`Checking ${filePath}...`);
  
  if (!fs.existsSync(filePath)) {
    console.log(`  ❌ File not found: ${filePath}`);
    results.issues.push({
      file: filePath,
      check: 'File Existence',
      description: 'File not found',
      severity: 'high'
    });
    results.overallResult = false;
    continue;
  }
  
  const content = fs.readFileSync(filePath, 'utf8');
  const fileResult = {
    file: filePath,
    checkResults: []
  };
  
  let filePassed = true;
  
  // Run each check
  for (const check of CRYPTO_CHECKS) {
    const checkResult = check.check(filePath, content);
    
    fileResult.checkResults.push({
      name: check.name,
      description: check.description,
      passed: checkResult.passed,
      details: checkResult.details
    });
    
    if (!checkResult.passed) {
      filePassed = false;
      results.issues.push({
        file: filePath,
        check: check.name,
        description: checkResult.details,
        severity: check.name.includes('Secure RNG') || 
                 check.name.includes('Hardcoded Secrets') ? 'critical' : 'high'
      });
      
      console.log(`  ❌ ${check.name}: ${checkResult.details}`);
    } else {
      console.log(`  ✅ ${check.name}: ${checkResult.details}`);
    }
  }
  
  fileResult.passed = filePassed;
  if (!filePassed) {
    results.overallResult = false;
  }
  
  results.fileResults.push(fileResult);
  console.log(`  Result: ${filePassed ? 'PASSED ✅' : 'FAILED ❌'}\n`);
}

// Output overall results
console.log('===== Overall Results =====');
console.log(`Files checked: ${results.filesChecked}`);
console.log(`Issues found: ${results.issues.length}`);
console.log(`Overall result: ${results.overallResult ? 'PASSED ✅' : 'FAILED ❌'}`);

if (results.issues.length > 0) {
  console.log('\nIssues summary:');
  
  const criticalIssues = results.issues.filter(i => i.severity === 'critical');
  const highIssues = results.issues.filter(i => i.severity === 'high');
  
  if (criticalIssues.length > 0) {
    console.log(`\nCritical Issues (${criticalIssues.length}):`);
    for (const issue of criticalIssues) {
      console.log(`  - [${issue.check}] ${issue.description} in ${issue.file}`);
    }
  }
  
  if (highIssues.length > 0) {
    console.log(`\nHigh Issues (${highIssues.length}):`);
    for (const issue of highIssues) {
      console.log(`  - [${issue.check}] ${issue.description} in ${issue.file}`);
    }
  }
}

// Save results to file
const reportDir = path.join(process.cwd(), 'reports');
if (!fs.existsSync(reportDir)) {
  fs.mkdirSync(reportDir, { recursive: true });
}

const reportPath = path.join(reportDir, `crypto-validation-report-${Date.now()}.json`);
fs.writeFileSync(reportPath, JSON.stringify(results, null, 2));
console.log(`\nDetailed report saved to: ${reportPath}`);

// Exit with appropriate code
process.exit(results.overallResult ? 0 : 1); 