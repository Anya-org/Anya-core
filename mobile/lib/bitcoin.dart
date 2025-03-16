class BitcoinMobileSDK {
  /// New SPV verification
  Future<bool> verifySPVProof(String txHash) async {
    return await _channel.invokeMethod('verifySPV', {'txHash': txHash});
  }

  /// Existing Lightning integration
  @override
  Future<LightningInvoice> createInvoice(int amount) { ... }
} 