#[hexagonal]
struct ServerCore {
    bitcoin_adapter: Arc<dyn BitcoinInterface>,
    auth_port: Arc<dyn AuthProvider>,
    metrics_port: Arc<dyn MetricsCollector>,
    // Adheres to BIP-341 SILENT_LEAF requirements
    taproot_engine: TaprootVerifier 
}