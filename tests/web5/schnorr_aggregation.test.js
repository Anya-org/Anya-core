// Tests for Web5 Schnorr Signature Aggregation Module
// [AIR-3][AIS-3][BPC-3][AIT-3][PFM-3][SCL-3][RES-3]

const assert = require('assert');
const { describe, it } = require('mocha');
const { FFI } = require('@node-rs/ffi');
const path = require('path');

// Load the Rust binary through FFI
const ffi = new FFI(path.join(__dirname, '../../target/release/libanya_core'));

describe('Schnorr Signature Aggregation', () => {
  // Test cross-input signature aggregation
  it('should correctly aggregate signatures across inputs', async () => {
    // Call Rust function through FFI
    const result = await ffi.call('test_cross_input_aggregation', []);
    const aggregationResult = JSON.parse(result);
    
    assert.equal(aggregationResult.success, true, 'Cross-input aggregation should succeed');
    assert.ok(aggregationResult.size_savings > 0, 'Should report size savings');
    assert.ok(aggregationResult.privacy_score > 0, 'Should report privacy score');
  });
  
  // Test MuSig aggregation
  it('should correctly perform MuSig key aggregation', async () => {
    const result = await ffi.call('test_musig_aggregation', []);
    const aggregationResult = JSON.parse(result);
    
    assert.equal(aggregationResult.success, true, 'MuSig aggregation should succeed');
    assert.ok(aggregationResult.size_savings > 0, 'Should report size savings');
    assert.ok(aggregationResult.privacy_score > aggregationResult.size_savings / 2, 'Privacy score should be higher for MuSig');
  });
  
  // Test verification of aggregated signatures
  it('should verify aggregated signatures correctly', async () => {
    const result = await ffi.call('test_verify_aggregated_signature', []);
    const verificationResult = JSON.parse(result);
    
    assert.equal(verificationResult.valid, true, 'Aggregated signature should verify correctly');
  });
  
  // Test integration with DLC adapter
  it('should integrate with DLC adapter', async () => {
    const result = await ffi.call('test_dlc_with_signature_aggregation', []);
    const integrationResult = JSON.parse(result);
    
    assert.equal(integrationResult.success, true, 'DLC with signature aggregation should succeed');
    assert.ok(integrationResult.transaction.length > 0, 'Should produce a valid transaction');
  });
});
