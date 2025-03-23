// Input adapter example
pub struct RestApi {
    handler: Arc<dyn CommandHandler>,
}

// Output adapter example
pub struct BitcoinNodeClient {
    rpc_client: Arc<Client>,
} 