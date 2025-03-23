#[derive(Clone)]
struct ChannelPersister {
    storage: Arc<dyn ChannelStorage>,
}

impl Persist for ChannelPersister {
    fn persist_channel(&self, channel_id: &ChannelId, data: &ChannelMonitor) -> Result<()> {
        self.storage.store_channel(channel_id, data.encode())
    }
} 