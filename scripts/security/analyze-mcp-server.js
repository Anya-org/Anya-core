#!/usr/bin/env node
/**
 * MCP Server Security Analyzer
 * [AIR-3][AIS-3][AIT-3][BPC-3][RES-3]
 * 
 * This script performs security analysis on the MCP server according to
 * official Bitcoin Improvement Proposals (BIPs) and AI labeling guidelines.
 */

const fs = require('fs');
const path = require('path');
const crypto = require('crypto');
const { execSync } = require('child_process');

// Parse command line arguments
const args = process.argv.slice(2);
let targetFile = null;

for (let i = 0; i < args.length; i++) {
  if (args[i] === '--file' && i + 1 < args.length) {
    targetFile = args[i + 1];
    break;
  }
}

if (!targetFile) {
  console.error('Error: Target file must be specified with --file');
  process.exit(1);
}

// Ensure the file exists
if (!fs.existsSync(targetFile)) {
  console.error(`Error: File not found: ${targetFile}`);
  process.exit(1);
}

// Add BIP-340 verification check
function checkSchnorrImplementation(code) {
  const hasSchnorrImport = /@noble\/curves\/secp256k1|schnorr/.test(code);
  const hasVerify = /verifySchnorrSignature|schnorr\.verify/.test(code);
  const hasConstantTime = /constantTimeEqual|timingSafeEqual|utils\.equalBytes/.test(code);
  
  return {
    passed: hasSchnorrImport && hasVerify && hasConstantTime,
    issues: [
      !hasSchnorrImport && 'Missing Schnorr implementation',
      !hasVerify && 'Missing Schnorr signature verification in cryptographic functions',
      !hasConstantTime && 'Missing constant-time comparison for cryptographic operations'
    ].filter(Boolean),
    recommendation: !hasVerify ? 'Implement proper Schnorr signature verification according to BIP-340' : undefined
  };
}

// Add BIP-341 Taproot compliance check
function checkTaprootCompliance(code) {
  // DEBUG: Print what the script is actually seeing
  console.log('\n[BIP-341 DEBUG] File content snippet:');
  console.log(code.slice(0, 1000)); // Print first 1000 chars
  
  // Look for all the required components individually instead of using regex
  const hasTaprootDescriptor = code.includes('tr(0x') && code.includes(',{SILENT_LEAF})');
  const has0xc0 = code.includes('0xc0') || code.includes('192'); // 0xc0 = 192 decimal
  const hasTaprootKeyword = code.includes('BIP-341') || code.includes('Taproot');
  const hasSilentLeaf = code.includes('SILENT_LEAF');
  const hasKeyPath = code.includes('key_path') || code.includes('keyPath');
  const hasScriptPath = code.includes('script_path') || code.includes('scriptPath');
  const hasValidation = code.includes('validateTaprootStructure') || code.includes('verify_taproot');
  
  console.log('[BIP-341 DEBUG] Individual checks:');
  console.log(`- hasTaprootDescriptor: ${hasTaprootDescriptor}`);
  console.log(`- has0xc0: ${has0xc0}`);
  console.log(`- hasTaprootKeyword: ${hasTaprootKeyword}`);
  console.log(`- hasSilentLeaf: ${hasSilentLeaf}`);
  console.log(`- hasKeyPath: ${hasKeyPath}`);
  console.log(`- hasScriptPath: ${hasScriptPath}`);
  console.log(`- hasValidation: ${hasValidation}`);
  
  // Modified compliance check - prioritize presence of key components over regex match
  const isCompliant = hasTaprootDescriptor && hasSilentLeaf && 
                      (hasKeyPath || hasScriptPath) && hasValidation &&
                      hasTaprootKeyword;
                      
  if (!isCompliant) {
    const missingComponents = [];
    if (!hasTaprootDescriptor) missingComponents.push('Taproot descriptor (tr(0x...))');
    if (!hasSilentLeaf) missingComponents.push('SILENT_LEAF support');
    if (!hasKeyPath && !hasScriptPath) missingComponents.push('key_path or script_path spending');
    if (!hasValidation) missingComponents.push('Taproot validation');
    if (!hasTaprootKeyword) missingComponents.push('BIP-341 reference');
    
    return {
      passed: false,
      description: `Missing proper Taproot structure according to BIP-341: ${missingComponents.join(', ')}`,
      recommendation: 'Implement proper Taproot structure with SILENT_LEAF for privacy preservation'
    };
  }
  
  return { passed: true };
}

// AI Labeling Compliance check
function checkAILabelingCompliance(code) {
  // Check for AI labeling tags
  const hasAIRTag = /\[AIR-[1-3]\]/.test(code);
  const hasAISTag = /\[AIS-[1-3]\]/.test(code);
  const hasBPCTag = /\[BPC-[1-3]\]/.test(code);
  
  // Check for header comments in functions
  const hasFunctionComments = /\/\*\*[\s\S]*?\*\/\s+function/.test(code);
  
  return {
    passed: hasAIRTag && hasAISTag && hasBPCTag && hasFunctionComments,
    issues: [
      !hasAIRTag && 'Missing AIR (AI Responsibility) labeling tags',
      !hasAISTag && 'Missing AIS (AI Security) labeling tags',
      !hasBPCTag && 'Missing BPC (Bitcoin Protocol Compliance) labeling tags',
      !hasFunctionComments && 'Missing proper documentation in functions'
    ].filter(Boolean),
    recommendation: 'Add proper AI labels according to the docs/standards/AI_LABELING.md guidelines'
  };
}

// Security checks based on BDF v2.5 requirements
const securityChecks = [
  {
    name: 'Schnorr Implementation',
    description: 'Schnorr signature implementation validation according to BIP-340',
    severity: 'critical',
    check: checkSchnorrImplementation
  },
  {
    name: 'Secure Random Number Generation',
    description: 'Checks for proper secure random number generation',
    severity: 'critical',
    check: (code) => {
      const hasSecureRandom = code.includes('crypto.randomBytes');
      const hasInsecureRandom = code.includes('Math.random()');
      
      if (!hasSecureRandom && hasInsecureRandom) {
        return {
          passed: false,
          description: 'Using insecure Math.random() for cryptographic purposes',
          recommendation: 'Replace Math.random() with crypto.randomBytes() for cryptographic operations'
        };
      }
      
      return { passed: true };
    }
  },
  {
    name: 'Signature Verification',
    description: 'Checks for proper signature verification in DLCs',
    severity: 'critical',
    check: (code) => {
      // Look for schnorr verification in the code
      const hasSchnorrVerify = code.includes('schnorrVerify') || 
                              code.includes('verify_signature') || 
                              code.includes('verifySchnorrSignature') ||
                              code.includes('schnorr.verify');
      
      // Check if there are verification functions without timing attack protection
      const hasTimingProtection = code.includes('constant-time') || 
                                 code.includes('time invariant') ||
                                 code.includes('constantTimeEqual') ||
                                 code.includes('timingSafeEqual');
      
      if (!hasSchnorrVerify) {
        return {
          passed: false,
          description: 'Missing Schnorr signature verification in cryptographic functions',
          recommendation: 'Implement proper Schnorr signature verification according to BIP-340'
        };
      }
      
      if (!hasTimingProtection) {
        return {
          passed: false,
          description: 'Potential timing attacks in signature verification',
          recommendation: 'Implement constant-time signature verification to prevent timing attacks'
        };
      }
      
      return { passed: true };
    }
  },
  {
    name: 'Input Validation',
    description: 'Checks for proper input validation',
    severity: 'high',
    check: (code) => {
      // Check if the code validates inputs
      const hasInputValidation = code.includes('test(params') || 
                               code.includes('Invalid input format') || 
                               code.includes('isValid');
      
      if (!hasInputValidation) {
        return {
          passed: false,
          description: 'Missing input validation for user-supplied data',
          recommendation: 'Implement input validation for all user-supplied parameters'
        };
      }
      
      return { passed: true };
    }
  },
  {
    name: 'Error Handling',
    description: 'Checks for proper error handling',
    severity: 'medium',
    check: (code) => {
      // Check if the code has try-catch blocks or error handling
      const hasTryCatch = code.includes('try {') && code.includes('catch');
      const hasErrorHandling = code.includes('sendError') || 
                              code.includes('log(`Error');
      
      if (!hasTryCatch || !hasErrorHandling) {
        return {
          passed: false,
          description: 'Insufficient error handling which could lead to crashes',
          recommendation: 'Implement comprehensive error handling with try-catch blocks and proper error reporting'
        };
      }
      
      return { passed: true };
    }
  },
  {
    name: 'BIP-341 Taproot Compliance',
    description: 'Checks for proper BIP-341 Taproot implementation',
    severity: 'high',
    check: (code) => {
      // DEBUG: Print what the script is actually seeing
      console.log('\n[BIP-341 DEBUG] File content snippet:');
      console.log(code.slice(0, 1000)); // Print first 1000 chars
      
      // Look for all the required components individually instead of using regex
      const hasTaprootDescriptor = code.includes('tr(0x') && code.includes(',{SILENT_LEAF})');
      const has0xc0 = code.includes('0xc0') || code.includes('192'); // 0xc0 = 192 decimal
      const hasTaprootKeyword = code.includes('BIP-341') || code.includes('Taproot');
      const hasSilentLeaf = code.includes('SILENT_LEAF');
      const hasKeyPath = code.includes('key_path') || code.includes('keyPath');
      const hasScriptPath = code.includes('script_path') || code.includes('scriptPath');
      const hasValidation = code.includes('validateTaprootStructure') || code.includes('verify_taproot');
      
      console.log('[BIP-341 DEBUG] Individual checks:');
      console.log(`- hasTaprootDescriptor: ${hasTaprootDescriptor}`);
      console.log(`- has0xc0: ${has0xc0}`);
      console.log(`- hasTaprootKeyword: ${hasTaprootKeyword}`);
      console.log(`- hasSilentLeaf: ${hasSilentLeaf}`);
      console.log(`- hasKeyPath: ${hasKeyPath}`);
      console.log(`- hasScriptPath: ${hasScriptPath}`);
      console.log(`- hasValidation: ${hasValidation}`);
      
      // Modified compliance check - prioritize presence of key components over regex match
      const isCompliant = hasTaprootDescriptor && hasSilentLeaf && 
                          (hasKeyPath || hasScriptPath) && hasValidation &&
                          hasTaprootKeyword;
                          
      if (!isCompliant) {
        const missingComponents = [];
        if (!hasTaprootDescriptor) missingComponents.push('Taproot descriptor (tr(0x...))');
        if (!hasSilentLeaf) missingComponents.push('SILENT_LEAF support');
        if (!hasKeyPath && !hasScriptPath) missingComponents.push('key_path or script_path spending');
        if (!hasValidation) missingComponents.push('Taproot validation');
        if (!hasTaprootKeyword) missingComponents.push('BIP-341 reference');
        
        return {
          passed: false,
          description: `Missing proper Taproot structure according to BIP-341: ${missingComponents.join(', ')}`,
          recommendation: 'Implement proper Taproot structure with SILENT_LEAF for privacy preservation'
        };
      }
      
      return { passed: true };
    }
  },
  {
    name: 'AI Labeling Compliance',
    description: 'AI labeling verification',
    severity: 'high',
    check: (code) => {
      // Check for AI labeling headers
      const hasAILabels = code.includes('[AIR-') && 
                        code.includes('[AIS-') && 
                        code.includes('[BPC-');
      
      if (!hasAILabels) {
        return {
          passed: false,
          description: 'Missing or incomplete AI labeling according to project standards',
          recommendation: 'Add proper AI labels according to the docs/standards/AI_LABELING.md guidelines'
        };
      }
      
      return { passed: true };
    }
  },
  {
    name: 'PSBT Handling',
    description: 'Checks for proper PSBT handling according to BIP-174/370',
    severity: 'high',
    check: (code) => {
      const hasPSBTGeneration = code.includes('generatePSBT');
      const hasPSBTValidation = code.includes('BIP-174') && code.includes('BIP-370');
      
      if (hasPSBTGeneration && !hasPSBTValidation) {
        return {
          passed: false,
          description: 'PSBT generation without proper validation according to BIPs',
          recommendation: 'Implement proper PSBT validation according to BIP-174 and BIP-370'
        };
      }
      
      return { passed: true };
    }
  },
  {
    name: 'Lightning Invoice Security',
    description: 'Checks for secure Lightning invoice implementation',
    severity: 'high',
    check: (code) => {
      const hasLightningInvoice = code.includes('createLightningInvoice');
      const hasPreimageGeneration = code.includes('preimage') && code.includes('crypto.randomBytes');
      
      if (hasLightningInvoice && !hasPreimageGeneration) {
        return {
          passed: false,
          description: 'Lightning invoice without secure preimage generation',
          recommendation: 'Ensure payment preimages are generated using crypto.randomBytes'
        };
      }
      
      return { passed: true };
    }
  }
];

// Read file content
const fileContent = fs.readFileSync(targetFile, 'utf8');

// Run security checks
const results = {
  timestamp: new Date().toISOString(),
  target: targetFile,
  passed: true,
  checkResults: [],
  issues: []
};

for (const check of securityChecks) {
  const result = check.check(fileContent);
  
  results.checkResults.push({
    name: check.name,
    description: check.description,
    passed: result.passed
  });
  
  if (!result.passed) {
    results.passed = false;
    results.issues.push({
      name: check.name,
      severity: check.severity,
      description: result.description,
      recommendation: result.recommendation,
      file: targetFile
    });
  }
}

// Output results
console.log(`\n===== MCP Server Security Analysis =====`);
console.log(`Target: ${targetFile}`);
console.log(`Timestamp: ${results.timestamp}`);
console.log(`Overall Result: ${results.passed ? 'PASSED ✅' : 'FAILED ❌'}`);
console.log(`\nCheck Results:`);

for (const result of results.checkResults) {
  console.log(`- ${result.name}: ${result.passed ? 'PASSED ✅' : 'FAILED ❌'}`);
}

if (results.issues.length > 0) {
  console.log(`\nIssues Found (${results.issues.length}):`);
  for (const issue of results.issues) {
    console.log(`\n[${issue.severity.toUpperCase()}] ${issue.name}`);
    console.log(`  Description: ${issue.description}`);
    console.log(`  Recommendation: ${issue.recommendation}`);
  }
  
  // Generate security report for GitHub Actions
  fs.writeFileSync('security-report.json', JSON.stringify(results, null, 2));
  
  // Exit with error if issues found
  process.exit(1);
} else {
  console.log(`\nNo security issues found. All checks passed.`);
}

// Generate detailed report
const reportDir = path.join(process.cwd(), 'reports');
if (!fs.existsSync(reportDir)) {
  fs.mkdirSync(reportDir, { recursive: true });
}

const reportPath = path.join(reportDir, `mcp-server-security-report-${Date.now()}.json`);
fs.writeFileSync(reportPath, JSON.stringify(results, null, 2));

console.log(`\nDetailed report saved to: ${reportPath}`);
process.exit(0);