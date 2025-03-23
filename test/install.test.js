describe('Installation Validation', () => {
  it('should successfully validate a correct installation', async () => {
    const isValid = await validateInstallation('/path/to/valid/install'); // Replace with a valid test path
    expect(isValid).toBe(true);
  });

  it('should fail validation if installation path is missing', async () => {
    await expect(validateInstallation('/path/to/missing/install')).rejects.toThrow('Installation path not found');
  });

  // ... existing test cases for other validation failures ...

  describe('BIP Compliance Checks', () => {
    it('should pass BIP-341 compliance if SILENT_LEAF is present in config', async () => {
      // Mock file system or create test files to simulate BIP-341 compliant config
      const isValid = await verifyBIPCompliance('/path/to/bip341/compliant/install'); // Replace with test path
      expect(isValid).toBe(true);
    });

    it('should fail BIP-341 compliance if SILENT_LEAF is missing in config', async () => {
      // Mock file system or create test files to simulate BIP-341 non-compliant config
      await expect(verifyBIPCompliance('/path/to/bip341/noncompliant/install')).rejects.toThrow('BIP-341 Violation: Missing SILENT_LEAF');
    });

    // Add more BIP compliance test cases for BIP-342, BIP-174 etc.
  });

  describe('BIP Compliance', () => {
    it('should validate BIP-341 during installation', async () => {
      const result = await validateInstallation('/path/with/silent_leaf');
      expect(result.bip341).toBe(true);
      expect(result.taprootConfig).toMatch(/SILENT_LEAF/);
    });

    it('should enforce PSBT v2 per BIP-174', async () => {
      await expect(validateInstallation('/path/without/psbt_v2'))
        .rejects.toThrow('PSBT v2 required');
    });
  });
}); 