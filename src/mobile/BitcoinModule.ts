import { TurboModule } from 'react-native';
import type { TurboModuleSpec } from 'react-native/Libraries/TurboModule/RCTExport';

export interface Spec extends TurboModuleSpec {
  generateSecureSeed(): Promise<string>;
  createPsbtV2(inputs: PSBTInput[]): Promise<string>;
  verifyTaprootSignature(
    message: string,
    signature: string,
    pubkey: string
  ): Promise<boolean>;
}

export default TurboModule.getEnforcing<Spec>('BitcoinModule');

// [BPC-3] Add Taproot validation
export function validateTaproot(tx: Transaction): boolean {
  return BitcoinCore.validateTaproot(tx);
} 
export default TurboModule.getEnforcing<Spec>('BitcoinModule'); 