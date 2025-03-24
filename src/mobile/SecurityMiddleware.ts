import { NativeModules } from 'react-native';

export const validateBitcoinOperation = async (
  operation: BitcoinOperation
): Promise<SecurityResult> => {
  // [AIS-3][BPC-3] Security validation
  try {
    const { BitcoinModule } = NativeModules;
    
    // Validate PSBTv2 structure
    const psbt = await BitcoinModule.createPsbtV2(operation.inputs);
    
    // Verify Taproot signature
    const isValid = await BitcoinModule.verifyTaprootSignature(
      operation.message,
      operation.signature,
      operation.pubkey
    );

    return { valid: isValid, psbt };
  } catch (error) {
    throw new BitcoinSecurityError('Validation failed', error);
  }
}; 