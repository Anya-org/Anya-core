import { NativeModules } from 'react-native';

/**
 * Bitcoin Mobile SDK v2.5
 * Updated: 2025-02-24
 * Compliant with BIP-341/342/174/370
 */

interface MobileSDK {
  // Core Bitcoin
  createWallet(mnemonic: string): Promise<void>;
  sendTransaction(recipient: string, amount: number): Promise<string>;
  getWalletInfo(): Promise<WalletInfo>;
  
  // Lightning Network 
  createInvoice(amount: number): Promise<string>;
  payInvoice(invoice: string): Promise<string>;
  
  // HSM Integration
  initHsm(config: HsmConfig): Promise<void>;
  signWithHsm(message: string): Promise<string>;

  createTaprootWallet(mnemonic: string): Promise<TaprootAddress>;
  signPsbt(psbt: string): Promise<string>;
  verifyBitcoinPayment(proof: BitcoinSPV): Promise<boolean>;
}

interface WalletInfo {
  balance: number;
  address: string;
  lastSync: string;
  transactionCount: number;
}

interface HsmConfig {
  type: 'yubihsm'|'ledger';
  path: string;
  network: 'mainnet'|'testnet';
}

export default NativeModules.BitcoinMobileSDK as MobileSDK; 