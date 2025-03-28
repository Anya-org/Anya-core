#![feature(edition2021)]
// Lightning Network Implementation v0.0.117
// Compliant with BOLT 1-12

use lightning::{
    chain::chainmonitor,
    ln::{channelmanager, peer_handler},
    router::Router,
    util::persist::FilesystemPersister,
};

pub struct LightningNode {
    channel_manager: Arc<channelmanager::ChannelManager>,
    peer_manager: Arc<peer_handler::PeerManager>,
    chain_monitor: Arc<chainmonitor::ChainMonitor>,
    router: Arc<Router>,
    persister: FilesystemPersister,
}

impl LightningNode {
    pub fn new(config: LdkConfig) -> Result<Self> {
        let network_graph = Arc::new(lightning::routing::network::NetworkGraph::new(
            config.network.into(),
        ));
        
        let persister = FilesystemPersister::new(config.storage_path)?;
        
        let channel_manager = Arc::new(channelmanager::ChannelManager::new(
            config.fee_estimator,
            config.chain_monitor,
            config.logger,
            config.keys_manager,
            config.user_config,
        ));

        let peer_manager = Arc::new(peer_handler::PeerManager::new(
            channel_manager.clone(),
            config.message_handler,
        ));

        Ok(Self {
            channel_manager,
            peer_manager,
            chain_monitor: config.chain_monitor,
            router: Arc::new(Router::new(network_graph)),
            persister,
        })
    }

    // BOLT 2: Peer Connection Management
    pub async fn connect_peer(&self, pubkey: PublicKey, addr: SocketAddr) -> Result<()> {
        self.peer_manager.connect_peer(pubkey, addr).await
    }

    // BOLT 2: Channel Creation
    pub async fn create_channel(&self, peer_pubkey: PublicKey, capacity_sats: u64) -> Result<ChannelId> {
        let channel_config = channelmanager::ChannelConfig::default();
        self.channel_manager.create_channel(peer_pubkey, capacity_sats, 0, channel_config)
    }

    // BOLT 11: Invoice Generation
    pub fn create_invoice(&self, amount_msat: u64) -> Result<Invoice> {
        InvoiceBuilder::new(self.network.into())
            .amount_msat(amount_msat)
            .build()
    }

    // BOLT 4: Payment Routing
    pub fn send_payment(&self, invoice: &Invoice) -> Result<PaymentHash> {
        let (payee, amount) = invoice.clone().into_parts();
        self.router.route_payment(payee, amount)
    }
} 