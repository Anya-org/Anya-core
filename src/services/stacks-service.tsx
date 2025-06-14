import React, { createContext, useContext, useCallback, useState } from 'react';

export enum StacksNetwork {
  MAINNET = 'mainnet',
  TESTNET = 'testnet',
}

export interface StacksTransaction {
  txId: string;
  status: 'pending' | 'success' | 'failed';
  blockHeight?: number;
  fee: number;
  contractCall?: {
    contractId: string;
    functionName: string;
    args: any[];
  };
  stxTransfer?: {
    recipient: string;
    amount: number;
    memo?: string;
  };
}

export interface StacksWallet {
  address: string;
  privateKey: string;
  publicKey: string;
  mnemonic?: string;
  network: StacksNetwork;
  balance: {
    stx: number;
    fungibleTokens: Record<string, number>;
  };
}

export interface StacksServiceConfig {
  network: StacksNetwork;
  apiKey?: string;
  timeout?: number;
}

class StacksServiceImpl {
  private baseUrl: string;
  private network: StacksNetwork;
  private apiKey?: string;
  private timeout: number;

  private static readonly MAINNET_URL = 'https://stacks-node-api.mainnet.stacks.co';
  private static readonly TESTNET_URL = 'https://stacks-node-api.testnet.stacks.co';

  constructor(config: StacksServiceConfig) {
    this.network = config.network;
    this.baseUrl = config.network === StacksNetwork.MAINNET 
      ? StacksServiceImpl.MAINNET_URL 
      : StacksServiceImpl.TESTNET_URL;
    this.apiKey = config.apiKey;
    this.timeout = config.timeout || 30000;
  }

  private async makeRequest(endpoint: string, options: RequestInit = {}): Promise<any> {
    const url = `${this.baseUrl}${endpoint}`;
    const headers: Record<string, string> = {
      'Content-Type': 'application/json',
      ...options.headers as Record<string, string>,
    };

    if (this.apiKey) {
      headers['Authorization'] = `Bearer ${this.apiKey}`;
    }

    try {
      const response = await fetch(url, {
        ...options,
        headers,
        signal: AbortSignal.timeout(this.timeout),
      });

      if (!response.ok) {
        throw new Error(`HTTP ${response.status}: ${response.statusText}`);
      }

      return await response.json();
    } catch (error) {
      console.error('Stacks API request failed:', error);
      throw new Error(`Failed to fetch from Stacks API: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  }

  async getAccountInfo(address: string): Promise<{
    balance: string;
    nonce: number;
    balance_proof: string;
    nonce_proof: string;
  }> {
    return this.makeRequest(`/extended/v1/address/${address}/balances`);
  }

  async getTransactionInfo(txId: string): Promise<StacksTransaction> {
    const response = await this.makeRequest(`/extended/v1/tx/${txId}`);
    
    return {
      txId: response.tx_id,
      status: response.tx_status === 'success' ? 'success' : 
              response.tx_status === 'abort_by_response' || response.tx_status === 'abort_by_post_condition' ? 'failed' : 'pending',
      blockHeight: response.block_height,
      fee: parseInt(response.fee_rate),
      contractCall: response.tx_type === 'contract_call' ? {
        contractId: response.contract_call.contract_id,
        functionName: response.contract_call.function_name,
        args: response.contract_call.function_args,
      } : undefined,
      stxTransfer: response.tx_type === 'token_transfer' ? {
        recipient: response.token_transfer.recipient_address,
        amount: parseInt(response.token_transfer.amount),
        memo: response.token_transfer.memo,
      } : undefined,
    };
  }

  async broadcastTransaction(txHex: string): Promise<{ txid: string; error?: string }> {
    try {
      const response = await this.makeRequest('/v2/transactions', {
        method: 'POST',
        body: txHex,
        headers: {
          'Content-Type': 'application/octet-stream',
        },
      });
      
      return { txid: response.txid || response };
    } catch (error) {
      return { 
        txid: '',
        error: error instanceof Error ? error.message : 'Failed to broadcast transaction',
      };
    }
  }

  async estimateFee(transaction: any): Promise<{ fee: number; feeRate: number }> {
    try {
      const response = await this.makeRequest('/v2/fees/transaction', {
        method: 'POST',
        body: JSON.stringify(transaction),
      });
      
      return {
        fee: parseInt(response.estimated_cost),
        feeRate: parseInt(response.estimated_cost_scalar),
      };
    } catch (error) {
      console.warn('Fee estimation failed, using default:', error);
      return { fee: 1000, feeRate: 1 };
    }
  }

  async getContractInterface(contractId: string): Promise<{
    functions: Array<{
      name: string;
      access: string;
      args: Array<{ name: string; type: string }>;
      outputs: { type: string };
    }>;
    variables: Array<{
      name: string;
      type: string;
      access: string;
    }>;
  }> {
    return this.makeRequest(`/v2/contracts/interface/${contractId}`);
  }

  async callReadOnlyFunction(
    contractId: string,
    functionName: string,
    args: string[],
    sender?: string
  ): Promise<{ okay: boolean; result: string }> {
    const body = {
      function_name: functionName,
      function_args: args,
      sender: sender || 'SP2J6ZY48GV1EZ5V2V5RB9MP66SW86PYKKNRV9EJ7',
    };

    return this.makeRequest(`/v2/contracts/call-read/${contractId}`, {
      method: 'POST',
      body: JSON.stringify(body),
    });
  }

  // Network info
  getNetwork(): StacksNetwork {
    return this.network;
  }

  getNetworkUrl(): string {
    return this.baseUrl;
  }
}

// React Context for Stacks Service
const StacksServiceContext = createContext<StacksServiceImpl | null>(null);

export interface StacksServiceProviderProps {
  children: React.ReactNode;
  config: StacksServiceConfig;
}

export const StacksServiceProvider: React.FC<StacksServiceProviderProps> = ({ 
  children, 
  config 
}) => {
  const [service] = useState(() => new StacksServiceImpl(config));

  return (
    <StacksServiceContext.Provider value={service}>
      {children}
    </StacksServiceContext.Provider>
  );
};

// Hook to use Stacks Service
export const useStacksService = (): StacksServiceImpl => {
  const service = useContext(StacksServiceContext);
  if (!service) {
    throw new Error('useStacksService must be used within a StacksServiceProvider');
  }
  return service;
};

// Hook for Stacks operations
export const useStacks = () => {
  const service = useStacksService();
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  const getAccountInfo = useCallback(async (address: string) => {
    setLoading(true);
    setError(null);
    try {
      const result = await service.getAccountInfo(address);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to get account info';
      setError(errorMessage);
      throw new Error(errorMessage);
    } finally {
      setLoading(false);
    }
  }, [service]);

  const getTransaction = useCallback(async (txId: string) => {
    setLoading(true);
    setError(null);
    try {
      const result = await service.getTransactionInfo(txId);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to get transaction';
      setError(errorMessage);
      throw new Error(errorMessage);
    } finally {
      setLoading(false);
    }
  }, [service]);

  const broadcastTransaction = useCallback(async (txHex: string) => {
    setLoading(true);
    setError(null);
    try {
      const result = await service.broadcastTransaction(txHex);
      if (result.error) {
        throw new Error(result.error);
      }
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to broadcast transaction';
      setError(errorMessage);
      throw new Error(errorMessage);
    } finally {
      setLoading(false);
    }
  }, [service]);

  const estimateFee = useCallback(async (transaction: any) => {
    setLoading(true);
    setError(null);
    try {
      const result = await service.estimateFee(transaction);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to estimate fee';
      setError(errorMessage);
      throw new Error(errorMessage);
    } finally {
      setLoading(false);
    }
  }, [service]);

  const callReadOnlyFunction = useCallback(async (
    contractId: string,
    functionName: string,
    args: string[],
    sender?: string
  ) => {
    setLoading(true);
    setError(null);
    try {
      const result = await service.callReadOnlyFunction(contractId, functionName, args, sender);
      return result;
    } catch (err) {
      const errorMessage = err instanceof Error ? err.message : 'Failed to call read-only function';
      setError(errorMessage);
      throw new Error(errorMessage);
    } finally {
      setLoading(false);
    }
  }, [service]);

  return {
    getAccountInfo,
    getTransaction,
    broadcastTransaction,
    estimateFee,
    callReadOnlyFunction,
    loading,
    error,
    service,
  };
};

export default StacksServiceImpl;
