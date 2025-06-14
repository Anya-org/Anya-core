import { useState, useEffect, useCallback } from 'react';
import { Web5 } from '@web5/api';

// Lightning Network service implementation for React
export interface LightningConfig {
  nodeUrl: string;
  macaroon: string;
  network: 'mainnet' | 'testnet' | 'regtest';
}

export interface NodeInfo {
  alias: string;
  identityPubkey: string;
  numActiveChannels: number;
  numPeers: number;
  blockHeight: number;
  synced: boolean;
}

export interface Invoice {
  paymentRequest: string;
  rHash: string;
  value: number;
  description: string;
  expiry: number;
  settled: boolean;
}

export interface Transaction {
  id: string;
  amount: number;
  fee: number;
  status: 'pending' | 'confirmed' | 'failed';
  timestamp: number;
  paymentHash?: string;
}

export class LightningServiceError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'LightningServiceError';
  }
}

export class LightningService {
  private web5: Web5;
  private config: LightningConfig;

  constructor(web5: Web5, config: LightningConfig) {
    this.web5 = web5;
    this.config = config;
  }

  async getNodeInfo(): Promise<NodeInfo> {
    try {
      const response = await this.makeRequest('getinfo');
      return {
        alias: response.alias,
        identityPubkey: response.identity_pubkey,
        numActiveChannels: response.num_active_channels,
        numPeers: response.num_peers,
        blockHeight: response.block_height,
        synced: response.synced_to_chain,
      };
    } catch (error) {
      throw new LightningServiceError(`Failed to get node info: ${error}`);
    }
  }

  async createInvoice(amount: number, description: string): Promise<string> {
    try {
      const response = await this.makeRequest('addinvoice', {
        value: amount,
        memo: description,
        expiry: 3600, // 1 hour
      });
      return response.payment_request;
    } catch (error) {
      throw new LightningServiceError(`Failed to create invoice: ${error}`);
    }
  }

  async payInvoice(invoice: string, maxFee?: number): Promise<Transaction> {
    try {
      // Decode invoice first to get amount and destination
      const decoded = await this.decodeInvoice(invoice);

      const response = await this.makeRequest('payinvoice', {
        payment_request: invoice,
        max_fee: maxFee,
        timeout_seconds: 60,
      });

      return {
        id: response.payment_hash,
        amount: decoded.num_satoshis,
        fee: response.payment_route?.total_fees_msat || 0,
        status: response.payment_error ? 'failed' : 'confirmed',
        timestamp: Date.now(),
        paymentHash: response.payment_hash,
      };
    } catch (error) {
      throw new LightningServiceError(`Failed to pay invoice: ${error}`);
    }
  }

  async decodeInvoice(invoice: string): Promise<any> {
    try {
      const response = await this.makeRequest('decodepayreq', {
        pay_req: invoice,
      });
      return response;
    } catch (error) {
      throw new LightningServiceError(`Failed to decode invoice: ${error}`);
    }
  }

  async openChannel(nodePubkey: string, amount: number): Promise<string> {
    try {
      const response = await this.makeRequest('openchannel', {
        node_pubkey: nodePubkey,
        local_funding_amount: amount,
        push_sat: 0,
        private: false,
      });
      return response.funding_txid_str;
    } catch (error) {
      throw new LightningServiceError(`Failed to open channel: ${error}`);
    }
  }

  async closeChannel(channelPoint: string, force: boolean = false): Promise<string> {
    try {
      const response = await this.makeRequest('closechannel', {
        channel_point: channelPoint,
        force,
      });
      return response.closing_txid;
    } catch (error) {
      throw new LightningServiceError(`Failed to close channel: ${error}`);
    }
  }

  async getChannels(): Promise<any[]> {
    try {
      const response = await this.makeRequest('listchannels');
      return response.channels || [];
    } catch (error) {
      throw new LightningServiceError(`Failed to get channels: ${error}`);
    }
  }

  private async makeRequest(method: string, params?: any): Promise<any> {
    const url = `${this.config.nodeUrl}/v1/${method}`;
    
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Grpc-Metadata-macaroon': this.config.macaroon,
      },
      body: JSON.stringify(params || {}),
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    return response.json();
  }
}

// React hook for Lightning Network operations
export function useLightningService(config: LightningConfig) {
  const [nodeInfo, setNodeInfo] = useState<NodeInfo | null>(null);
  const [channels, setChannels] = useState<any[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const web5 = new Web5(); // Initialize Web5
  const lightningService = new LightningService(web5, config);

  const fetchNodeInfo = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const info = await lightningService.getNodeInfo();
      setNodeInfo(info);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, [lightningService]);

  const fetchChannels = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const channelList = await lightningService.getChannels();
      setChannels(channelList);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, [lightningService]);

  const createInvoice = useCallback(async (amount: number, description: string) => {
    setLoading(true);
    setError(null);
    try {
      return await lightningService.createInvoice(amount, description);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
      throw err;
    } finally {
      setLoading(false);
    }
  }, [lightningService]);

  const payInvoice = useCallback(async (invoice: string, maxFee?: number) => {
    setLoading(true);
    setError(null);
    try {
      return await lightningService.payInvoice(invoice, maxFee);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
      throw err;
    } finally {
      setLoading(false);
    }
  }, [lightningService]);

  useEffect(() => {
    fetchNodeInfo();
    fetchChannels();
  }, [fetchNodeInfo, fetchChannels]);

  return {
    nodeInfo,
    channels,
    loading,
    error,
    lightningService,
    fetchNodeInfo,
    fetchChannels,
    createInvoice,
    payInvoice,
  };
}
