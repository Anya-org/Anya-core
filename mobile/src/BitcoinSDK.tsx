import { NativeModules } from 'react-native';

interface BitcoinMobileSDK {
  verifySPVProof(txHash: string): Promise<boolean>;
  createInvoice(amount: number): Promise<string>;
}

export default NativeModules.BitcoinMobileSDK as BitcoinMobileSDK; 