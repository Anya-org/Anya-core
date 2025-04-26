/**
 * Web5 Bitcoin Protocol Integration Tests
 * [AIR-3][AIS-3][BPC-3][AIT-3][PFM-3]
 *
 * This test suite verifies BIP-340/341/342 compliance for Web5 Bitcoin integration,
 * including key-path and script-path spending with privacy guarantees.
 */

import { Web5BipValidator } from '../../scripts/web5/validate-bip-compliance.js';
import { schnorr } from '@noble/curves/secp256k1';
import { utils } from '@noble/curves/abstract/utils';

// Test vectors for BIP-340 Schnorr signatures
const SCHNORR_TEST_VECTORS = [
  {
    // Test vector 1 from BIP-340 spec
    privkey: '0000000000000000000000000000000000000000000000000000000000000003',
    pubkey: 'F9308A019258C31049344F85F89D5229B531C845836F99B08601F113BCE036F9',
    auxRand: '0000000000000000000000000000000000000000000000000000000000000000',
    message: '0000000000000000000000000000000000000000000000000000000000000000',
    signature: 'E907831F80848D1069A5371B402410364BDF1C5F8307B0084C55F1CE2DCA821525F66A4A85EA8B71E482A74F382D2CE5EBEEE8FDB2172F477DF4900D310536C',
    verification: true,
  },
  {
    // Test vector 2 from BIP-340 spec
    privkey: 'B7E151628AED2A6ABF7158809CF4F3C762E7160F38B4DA56A784D9045190CFEF',
    pubkey: 'DFF1D77F2A671C5F36183726DB2341BE58FEAE1DA2DECED843240F7B502BA659',
    auxRand: '0000000000000000000000000000000000000000000000000000000000000001',
    message: '243F6A8885A308D313198A2E03707344A4093822299F31D0082EFA98EC4E6C89',
    signature: '6896BD60EEAE296DB48A229FF71DFE071BDE413E6D43F917DC8DCF8C78DE33418906D11AC976ABCCB20B091292BFF4EA897EFCB639EA871CFA95F6DE339E4B0A',
    verification: true,
  }
];

// Test vectors for BIP-341 Taproot key-path spending
const TAPROOT_KEY_PATH_VECTORS = [
  {
    description: "Basic P2TR key path spend with minimal witness",
    internalKey: "cc8a4bc64d897bddc5fbc2f670f7a8ba0b386779106cf1223c6fc5d7cd6fc115",
    scriptTree: null, // null indicates no script tree (key-path only)
    tweak: "0000000000000000000000000000000000000000000000000000000000000001",
    outputPubKey: "a60869f0dbcf1dc659c9cecbaf8050135ea9e8cdc487053f1dc6880949dc684c",
    scriptPath: false,
    expectedValid: true
  }
];

// Test vectors for BIP-341 Taproot script-path spending
const TAPROOT_SCRIPT_PATH_VECTORS = [
  {
    description: "Basic P2TR script path spend with single leaf",
    internalKey: "cc8a4bc64d897bddc5fbc2f670f7a8ba0b386779106cf1223c6fc5d7cd6fc115",
    scriptTree: {
      script: "20cc8a4bc64d897bddc5fbc2f670f7a8ba0b386779106cf1223c6fc5d7cd6fc115ac",
      leafVersion: 0xc0, // SILENT_LEAF with tapscript version 0
    },
    outputPubKey: "a60869f0dbcf1dc659c9cecbaf8050135ea9e8cdc487053f1dc6880949dc684c",
    merkleProof: "627a6309000521a603ab1c260610d46bde3b3f022383af3182a7eba30a0d724e",
    controlBlock: "c0cc8a4bc64d897bddc5fbc2f670f7a8ba0b386779106cf1223c6fc5d7cd6fc115627a6309000521a603ab1c260610d46bde3b3f022383af3182a7eba30a0d724e",
    scriptPath: true,
    expectedValid: true
  },
  {
    description: "Multi-leaf merkle tree with complex spending conditions",
    internalKey: "a546e36bf0527c9d3b16154b82ee5d96fc1efe398f573e185a2826f7113a7933",
    scriptTree: {
      script: "20a546e36bf0527c9d3b16154b82ee5d96fc1efe398f573e185a2826f7113a7933ad26a9ac", // OP_CHECKSIG OP_TOALTSTACK OP_CHECKSIGVERIFY
      leafVersion: 0xc0,
      branches: [
        {
          script: "82012088a914a08eae45a21e94250e98db585e30157d0e4be4c8870125b251070e29ca19043cf33ccd7324e2ddab03ecc4ae0b5e77c4fc0e5cf6c95a09b87000000000000000042a06d657461646174610c",
          leafVersion: 0xc0
        }
      ]
    },
    outputPubKey: "be4d8a11c1fcaef369e66965cbc58dfd7f272fda0b3c46ffa48bfbeb53cad2b7",
    merkleProof: "58b6360cfad4b5038ef4c054dd6ac49f511f33c4297e409d30f4b3384c556e12",
    controlBlock: "c0a546e36bf0527c9d3b16154b82ee5d96fc1efe398f573e185a2826f7113a793358b6360cfad4b5038ef4c054dd6ac49f511f33c4297e409d30f4b3384c556e12",
    scriptPath: true,
    expectedValid: true
  }
];

// Test vectors for cross-input signature aggregation
const CROSS_INPUT_AGG_VECTORS = [
  {
    description: "Two-input transaction with aggregated signature",
    inputs: [
      {
        pubkey: "cc8a4bc64d897bddc5fbc2f670f7a8ba0b386779106cf1223c6fc5d7cd6fc115",
        message: "0000000000000000000000000000000000000000000000000000000000000000"
      },
      {
        pubkey: "a546e36bf0527c9d3b16154b82ee5d96fc1efe398f573e185a2826f7113a7933",
        message: "0000000000000000000000000000000000000000000000000000000000000001"
      }
    ],
    aggregated: true,
    expectedValid: true
  }
];

// Utility function to convert hex to Uint8Array
function hexToBytes(hex) {
  if (typeof hex !== 'string') {
    throw new Error('Expected string containing hex digits');
  }
  if (hex.length % 2 !== 0) {
    throw new Error('Expected even number of hex digits');
  }
  const bytes = new Uint8Array(hex.length / 2);
  for (let i = 0; i < bytes.length; i++) {
    const j = i * 2;
    bytes[i] = Number.parseInt(hex.substring(j, j + 2), 16);
  }
  return bytes;
}

// Utility function to load test assets
async function loadTestAsset(filename) {
  // For test purposes we'll simulate loading a file
  if (filename === 'valid_psbt.base64') {
    return 'cHNidP8BAHUCAAAAASaBcTce3/KF6Tet7qSze3gADAVmy7OtZGQXE8pCFxv2AAAAAAD+////AtPf9QUAAAAAGXapFNDFmQPFusKGh2DpD9UhpGZap2UgiKiXSDBMHQAAAAAAF6kUezoAv9wU0neVwrdJAdCdpu8TNXkh9qqIrA75fyPm';
  }
  return null;
}

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

  describe('BIP-340 Schnorr Signature Verification', () => {
    test.each(SCHNORR_TEST_VECTORS)('Verifies test vector with pubkey %s', (vector) => {
      const pubkeyBytes = hexToBytes(vector.pubkey.toLowerCase());
      const msgBytes = hexToBytes(vector.message.toLowerCase());
      const sigBytes = hexToBytes(vector.signature.toLowerCase());

      const result = schnorr.verify(sigBytes, msgBytes, pubkeyBytes);
      expect(result).toBe(vector.verification);
    });
  });

  describe('BIP-341 Taproot Key-Path Spending', () => {
    test.each(TAPROOT_KEY_PATH_VECTORS)('Validates key-path spending: %s', (vector) => {
      const result = validator.validateTaprootKeyPath(vector.internalKey, vector.outputPubKey);
      expect(result.valid).toBe(vector.expectedValid);
      if (result.valid) {
        expect(result.privacy).toBe('enhanced');
      }
    });
  });

  describe('BIP-341 Taproot Script-Path Spending', () => {
    test.each(TAPROOT_SCRIPT_PATH_VECTORS)('Validates script-path spending: %s', (vector) => {
      const result = validator.validateTaprootScriptPath(
        vector.internalKey,
        vector.scriptTree.script,
        vector.controlBlock,
        vector.merkleProof
      );
      expect(result.valid).toBe(vector.expectedValid);
      if (result.valid) {
        expect(result.hasSilentLeaf).toBe(true);
        expect(result.privacy).toBe('enhanced');
      }
    });
  });

  describe('Cross-Input Signature Aggregation', () => {
    test.each(CROSS_INPUT_AGG_VECTORS)('Validates aggregated signatures: %s', (vector) => {
      const result = validator.validateCrossInputAggregation(
        vector.inputs.map(input => ({
          pubkey: input.pubkey,
          message: input.message
        })),
        vector.aggregated
      );
      expect(result.valid).toBe(vector.expectedValid);
      if (result.valid && vector.aggregated) {
        expect(result.efficiency).toBeGreaterThan(1.0); // Should be more efficient than individual sigs
        expect(result.privacy).toBe('enhanced');
      }
    });
  });
  
  describe('BIP-342 Tapscript Validation', () => {
    test('Validates tapscript with OP_CHECKSIGADD', () => {
      const script = '20cc8a4bc64d897bddc5fbc2f670f7a8ba0b386779106cf1223c6fc5d7cd6fc115ba20a546e36bf0527c9d3b16154b82ee5d96fc1efe398f573e185a2826f7113a793387';
      // OP_CHECKSIG OP_CHECKSIGADD validates multiple signatures with one opcode (threshold 2-of-2)
      const result = validator.validateTapscript(script);
      expect(result.valid).toBe(true);
      expect(result.efficiency).toBeGreaterThan(1.0);
      expect(result.standards).toContain('BIP-342');
    });
  });
});