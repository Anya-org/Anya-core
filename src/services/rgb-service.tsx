import { useState, useEffect, useCallback } from 'react';
import { Web5 } from '@web5/api';

// RGB protocol service for Bitcoin asset management in React
export interface RGBConfig {
  nodeEndpoint: string;
  network: 'mainnet' | 'testnet' | 'regtest';
  dataDir?: string;
}

export interface RGBAsset {
  assetId: string;
  name: string;
  description: string;
  supply: number;
  precision: number;
  genesis: string;
  metadata?: Record<string, any>;
}

export interface AssetTransfer {
  assetId: string;
  toAddress: string;
  amount: number;
  metadata?: Record<string, any>;
}

export interface RGBTransaction {
  id: string;
  assetId: string;
  amount: number;
  fromAddress: string;
  toAddress: string;
  status: 'pending' | 'confirmed' | 'failed';
  timestamp: number;
  txHash?: string;
}

export class RGBServiceError extends Error {
  constructor(message: string) {
    super(message);
    this.name = 'RGBServiceError';
  }
}

export class RGBService {
  private web5: Web5;
  private config: RGBConfig;

  constructor(web5: Web5, config: RGBConfig) {
    this.web5 = web5;
    this.config = config;
  }

  async createAsset(params: {
    name: string;
    description: string;
    supply: number;
    precision: string;
    metadata?: Record<string, any>;
  }): Promise<RGBAsset> {
    try {
      const response = await this.makeRequest('create_asset', {
        name: params.name,
        description: params.description,
        supply: params.supply,
        precision: params.precision,
        metadata: params.metadata,
      });

      return {
        assetId: response.asset_id,
        name: params.name,
        description: params.description,
        supply: params.supply,
        precision: parseInt(params.precision),
        genesis: response.genesis,
        metadata: params.metadata,
      };
    } catch (error) {
      throw new RGBServiceError(`Failed to create asset: ${error}`);
    }
  }

  async transferAsset(transfer: AssetTransfer): Promise<RGBTransaction> {
    try {
      const response = await this.makeRequest('transfer_asset', {
        asset_id: transfer.assetId,
        to_address: transfer.toAddress,
        amount: transfer.amount,
        metadata: transfer.metadata,
      });

      return {
        id: response.transfer_id,
        assetId: transfer.assetId,
        amount: transfer.amount,
        fromAddress: response.from_address,
        toAddress: transfer.toAddress,
        status: 'pending',
        timestamp: Date.now(),
        txHash: response.tx_hash,
      };
    } catch (error) {
      throw new RGBServiceError(`Failed to transfer asset: ${error}`);
    }
  }

  async getAsset(assetId: string): Promise<RGBAsset> {
    try {
      const response = await this.makeRequest('get_asset', {
        asset_id: assetId,
      });

      return {
        assetId: response.asset_id,
        name: response.name,
        description: response.description,
        supply: response.supply,
        precision: response.precision,
        genesis: response.genesis,
        metadata: response.metadata,
      };
    } catch (error) {
      throw new RGBServiceError(`Failed to get asset: ${error}`);
    }
  }

  async listAssets(): Promise<RGBAsset[]> {
    try {
      const response = await this.makeRequest('list_assets');
      return response.assets.map((asset: any) => ({
        assetId: asset.asset_id,
        name: asset.name,
        description: asset.description,
        supply: asset.supply,
        precision: asset.precision,
        genesis: asset.genesis,
        metadata: asset.metadata,
      }));
    } catch (error) {
      throw new RGBServiceError(`Failed to list assets: ${error}`);
    }
  }

  async getBalance(assetId: string): Promise<number> {
    try {
      const response = await this.makeRequest('get_balance', {
        asset_id: assetId,
      });
      return response.balance;
    } catch (error) {
      throw new RGBServiceError(`Failed to get balance: ${error}`);
    }
  }

  async getTransactionHistory(assetId?: string): Promise<RGBTransaction[]> {
    try {
      const response = await this.makeRequest('get_transaction_history', {
        asset_id: assetId,
      });

      return response.transactions.map((tx: any) => ({
        id: tx.id,
        assetId: tx.asset_id,
        amount: tx.amount,
        fromAddress: tx.from_address,
        toAddress: tx.to_address,
        status: tx.status,
        timestamp: tx.timestamp,
        txHash: tx.tx_hash,
      }));
    } catch (error) {
      throw new RGBServiceError(`Failed to get transaction history: ${error}`);
    }
  }

  async validateContract(contractId: string): Promise<boolean> {
    try {
      const response = await this.makeRequest('validate_contract', {
        contract_id: contractId,
      });
      return response.valid;
    } catch (error) {
      throw new RGBServiceError(`Failed to validate contract: ${error}`);
    }
  }

  async generateAddress(assetId: string): Promise<string> {
    try {
      const response = await this.makeRequest('generate_address', {
        asset_id: assetId,
      });
      return response.address;
    } catch (error) {
      throw new RGBServiceError(`Failed to generate address: ${error}`);
    }
  }

  private async makeRequest(method: string, params?: any): Promise<any> {
    const url = `${this.config.nodeEndpoint}/api/v1/${method}`;
    
    const response = await fetch(url, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
      },
      body: JSON.stringify(params || {}),
    });

    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`);
    }

    return response.json();
  }
}

// React hook for RGB protocol operations
export function useRGBService(config: RGBConfig) {
  const [assets, setAssets] = useState<RGBAsset[]>([]);
  const [transactions, setTransactions] = useState<RGBTransaction[]>([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const web5 = new Web5(); // Initialize Web5
  const rgbService = new RGBService(web5, config);

  const fetchAssets = useCallback(async () => {
    setLoading(true);
    setError(null);
    try {
      const assetList = await rgbService.listAssets();
      setAssets(assetList);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, [rgbService]);

  const fetchTransactions = useCallback(async (assetId?: string) => {
    setLoading(true);
    setError(null);
    try {
      const txHistory = await rgbService.getTransactionHistory(assetId);
      setTransactions(txHistory);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
    } finally {
      setLoading(false);
    }
  }, [rgbService]);

  const createAsset = useCallback(async (params: {
    name: string;
    description: string;
    supply: number;
    precision: string;
    metadata?: Record<string, any>;
  }) => {
    setLoading(true);
    setError(null);
    try {
      const asset = await rgbService.createAsset(params);
      setAssets(prev => [...prev, asset]);
      return asset;
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
      throw err;
    } finally {
      setLoading(false);
    }
  }, [rgbService]);

  const transferAsset = useCallback(async (transfer: AssetTransfer) => {
    setLoading(true);
    setError(null);
    try {
      const transaction = await rgbService.transferAsset(transfer);
      setTransactions(prev => [...prev, transaction]);
      return transaction;
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
      throw err;
    } finally {
      setLoading(false);
    }
  }, [rgbService]);

  const getBalance = useCallback(async (assetId: string) => {
    setError(null);
    try {
      return await rgbService.getBalance(assetId);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Unknown error');
      throw err;
    }
  }, [rgbService]);

  useEffect(() => {
    fetchAssets();
    fetchTransactions();
  }, [fetchAssets, fetchTransactions]);

  return {
    assets,
    transactions,
    loading,
    error,
    rgbService,
    fetchAssets,
    fetchTransactions,
    createAsset,
    transferAsset,
    getBalance,
  };
}
