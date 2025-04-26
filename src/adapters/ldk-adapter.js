import { NodeBuilder, ChannelManager } from 'lightningdevkit';

export class LDKAdapter {
  constructor() {
    this.node = null;
    this.channelManager = null;
  }

  async initialize() {
    this.node = await NodeBuilder.with_esplora_blockchain(
      'bitcoin', 
      process.env.ESPLORA_URL
    ).build();
    
    this.channelManager = new ChannelManager();
    ChannelManager.install(this.node);
  }

  createInvoice(amountMsat, description) {
    return this.node.create_invoice({ 
      amount_msat: amountMsat,
      description,
      expiry_time: 3600
    });
  }
} 