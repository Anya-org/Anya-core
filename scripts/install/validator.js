// Add BIP-341/342 validation to installer
const BIP_VALIDATION = {
  'BIP-341': {
    check: (config) => /SILENT_LEAF/.test(config.taprootScript),
    error: 'Missing SILENT_LEAF in Taproot configuration'
  },
  'BIP-342': {
    check: (config) => /OP_CHECKSIGADD/.test(config.tapscript),
    error: 'Missing OP_CHECKSIGADD in Tapscript'
  }
};

async function validateInstallation(installPath) {
  log(`Validating installation at: ${installPath}`);

  // 1. Check if installation path exists
  if (!fs.existsSync(installPath)) {
    throw new Error(`Installation path not found: ${installPath}`);
  }

  // Existing validation checks
  if (!fs.existsSync(installPath)) {
    throw new Error('Install path does not exist');
  }

  // BIP compliance checks
  await verifyBIPCompliance(installPath);

  log('Installation validation successful');
  return true;
}

async function verifyBIPCompliance(installPath) {
  log('Verifying BIP compliance...');

  // BIP-341 (Taproot) check - Example: Check for SILENT_LEAF in config
  const bitcoinConfPath = path.join(installPath, 'bitcoin.conf');
  if (fs.existsSync(bitcoinConfPath)) {
    const bitcoinConfContent = fs.readFileSync(bitcoinConfPath, 'utf8');
    if (!/SILENT_LEAF/.test(bitcoinConfContent)) {
      throw new Error('BIP-341 Violation: Missing SILENT_LEAF in bitcoin.conf');
    }
  } else {
    warn('bitcoin.conf not found, skipping BIP-341 check');
  }

  // Add more BIP checks here (BIP-342, BIP-174 etc.) as needed

  log('BIP compliance checks passed (preliminary)');
}

function validateInstallationSync(config) {
  let valid = true;
  const errors = [];
  
  // Existing validation checks
  if (!fs.existsSync(config.installPath)) {
    valid = false;
    errors.push('Install path does not exist');
  }

  // BIP compliance checks
  for (const [bip, validator] of Object.entries(BIP_VALIDATION)) {
    if (!validator.check(config)) {
      valid = false;
      errors.push(`BIP-${bip} Error: ${validator.error}`);
    }
  }

  return { valid, errors };
} 