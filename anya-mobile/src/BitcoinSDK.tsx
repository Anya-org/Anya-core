interface MobileSDK {
  // Core Bitcoin
  createWallet(mnemonic: string): Promise<void>;
  sendTransaction(recipient: string, amount: number): Promise<string>;
  getWalletInfo(): Promise<WalletInfo>;
  
  // BOLT11 Lightning
  createInvoice(amount: number): Promise<string>;
  payInvoice(invoice: string): Promise<string>;
  
  // FIDO2 HSM Integration
  initHsm(config: HsmConfig): Promise<void>;
  signWithHsm(message: string): Promise<string>;
}