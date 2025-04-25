import { Web5BipValidator } from '../../scripts/web5/validate-bip-compliance.js';

describe('BIP Compliance Validation', () => {
  let validator;

  beforeAll(async () => {
    validator = new Web5BipValidator();
  });

  test('PSBT validation', async () => {
    const psbt = await loadTestAsset('valid_psbt.base64');
    const result = await validator.validatePSBT(psbt);
    expect(result.compliant).toBe(true);
    expect(result.standards).toContain('BIP-174');
  });
}); 