import React, { useState, useEffect, useCallback } from 'react';
import { Web5 } from '@web5/api';
import { LightningService, useLightningService, LightningConfig } from './lightning-service';
import { RGBService, useRGBService, RGBConfig } from './rgb-service';

// Comprehensive Layer 2 management for React
export interface Layer2Config {
  lightning: LightningConfig;
  rgb: RGBConfig;
  bob: {
    rpcUrl: string;
    chainId: number;
  };
  liquid: {
    nodeEndpoint: string;
    network: string;
  };
  rsk: {
    rpcUrl: string;
    network: string;
  };
  stacks: {
    rpcUrl: string;
    network: string;
  };
  taprootAssets: {
    bitcoinRpcUrl: string;
    tapdUrl: string;
    network: string;
  };
}

export type Layer2Protocol = 
  | 'lightning' 
  | 'rgb' 
  | 'bob' 
  | 'liquid' 
  | 'rsk' 
  | 'stacks' 
  | 'taproot-assets';

export interface ProtocolStatus {
  protocol: Layer2Protocol;
  connected: boolean;
  initialized: boolean;
  blockHeight?: number;
  peers?: number;
  error?: string;
}

export interface CrossLayerTransfer {
  fromProtocol: Layer2Protocol;
  toProtocol: Layer2Protocol;
  assetId: string;
  amount: number;
  fromAddress: string;
  toAddress: string;
}

export function useLayer2Manager(config: Layer2Config) {
  const [protocolStatuses, setProtocolStatuses] = useState<Record<Layer2Protocol, ProtocolStatus>>({
    lightning: { protocol: 'lightning', connected: false, initialized: false },
    rgb: { protocol: 'rgb', connected: false, initialized: false },
    bob: { protocol: 'bob', connected: false, initialized: false },
    liquid: { protocol: 'liquid', connected: false, initialized: false },
    rsk: { protocol: 'rsk', connected: false, initialized: false },
    stacks: { protocol: 'stacks', connected: false, initialized: false },
    'taproot-assets': { protocol: 'taproot-assets', connected: false, initialized: false },
  });

  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Initialize individual protocol hooks
  const lightning = useLightningService(config.lightning);
  const rgb = useRGBService(config.rgb);

  const initializeAllProtocols = useCallback(async () => {
    setLoading(true);
    setError(null);

    try {
      // Initialize Lightning Network
      await lightning.fetchNodeInfo();
      setProtocolStatuses(prev => ({
        ...prev,
        lightning: {
          ...prev.lightning,
          connected: lightning.nodeInfo !== null,
          initialized: true,
          blockHeight: lightning.nodeInfo?.blockHeight,
          peers: lightning.nodeInfo?.numPeers,
        }
      }));

      // Initialize RGB
      await rgb.fetchAssets();
      setProtocolStatuses(prev => ({
        ...prev,
        rgb: {
          ...prev.rgb,
          connected: rgb.assets.length >= 0, // Connection successful if no error
          initialized: true,
        }
      }));

      // Initialize other protocols (mock implementations for now)
      await initializeBOB();
      await initializeLiquid();
      await initializeRSK();
      await initializeStacks();
      await initializeTaprootAssets();

    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to initialize protocols');
    } finally {
      setLoading(false);
    }
  }, [lightning, rgb]);

  const initializeBOB = async () => {
    try {
      // Mock BOB initialization
      const response = await fetch(`${config.bob.rpcUrl}/health`);
      setProtocolStatuses(prev => ({
        ...prev,
        bob: {
          ...prev.bob,
          connected: response.ok,
          initialized: true,
        }
      }));
    } catch (err) {
      setProtocolStatuses(prev => ({
        ...prev,
        bob: {
          ...prev.bob,
          connected: false,
          initialized: true,
          error: 'Failed to connect to BOB',
        }
      }));
    }
  };

  const initializeLiquid = async () => {
    try {
      // Mock Liquid initialization
      setProtocolStatuses(prev => ({
        ...prev,
        liquid: {
          ...prev.liquid,
          connected: true,
          initialized: true,
        }
      }));
    } catch (err) {
      setProtocolStatuses(prev => ({
        ...prev,
        liquid: {
          ...prev.liquid,
          connected: false,
          initialized: true,
          error: 'Failed to connect to Liquid',
        }
      }));
    }
  };

  const initializeRSK = async () => {
    try {
      // Mock RSK initialization
      setProtocolStatuses(prev => ({
        ...prev,
        rsk: {
          ...prev.rsk,
          connected: true,
          initialized: true,
        }
      }));
    } catch (err) {
      setProtocolStatuses(prev => ({
        ...prev,
        rsk: {
          ...prev.rsk,
          connected: false,
          initialized: true,
          error: 'Failed to connect to RSK',
        }
      }));
    }
  };

  const initializeStacks = async () => {
    try {
      // Mock Stacks initialization
      setProtocolStatuses(prev => ({
        ...prev,
        stacks: {
          ...prev.stacks,
          connected: true,
          initialized: true,
        }
      }));
    } catch (err) {
      setProtocolStatuses(prev => ({
        ...prev,
        stacks: {
          ...prev.stacks,
          connected: false,
          initialized: true,
          error: 'Failed to connect to Stacks',
        }
      }));
    }
  };

  const initializeTaprootAssets = async () => {
    try {
      // Mock Taproot Assets initialization
      setProtocolStatuses(prev => ({
        ...prev,
        'taproot-assets': {
          ...prev['taproot-assets'],
          connected: true,
          initialized: true,
        }
      }));
    } catch (err) {
      setProtocolStatuses(prev => ({
        ...prev,
        'taproot-assets': {
          ...prev['taproot-assets'],
          connected: false,
          initialized: true,
          error: 'Failed to connect to Taproot Assets',
        }
      }));
    }
  };

  const executeCrossLayerTransfer = useCallback(async (transfer: CrossLayerTransfer) => {
    setLoading(true);
    setError(null);

    try {
      // Implementation would handle actual cross-layer bridging
      console.log('Executing cross-layer transfer:', transfer);
      
      // Mock implementation
      const transferId = `${transfer.fromProtocol}-${transfer.toProtocol}-${Date.now()}`;
      
      // This would typically involve:
      // 1. Lock assets on source protocol
      // 2. Verify lock transaction
      // 3. Mint/unlock assets on destination protocol
      // 4. Provide proof of completion
      
      return {
        transferId,
        status: 'completed',
        timestamp: Date.now(),
      };
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Cross-layer transfer failed');
      throw err;
    } finally {
      setLoading(false);
    }
  }, []);

  const getProtocolHealth = useCallback((protocol: Layer2Protocol): 'healthy' | 'degraded' | 'down' => {
    const status = protocolStatuses[protocol];
    if (!status.initialized) return 'down';
    if (!status.connected) return 'down';
    if (status.error) return 'degraded';
    return 'healthy';
  }, [protocolStatuses]);

  const getAllConnectedProtocols = useCallback((): Layer2Protocol[] => {
    return Object.entries(protocolStatuses)
      .filter(([_, status]) => status.connected && status.initialized)
      .map(([protocol, _]) => protocol as Layer2Protocol);
  }, [protocolStatuses]);

  useEffect(() => {
    initializeAllProtocols();
  }, [initializeAllProtocols]);

  return {
    protocolStatuses,
    loading,
    error,
    lightning,
    rgb,
    initializeAllProtocols,
    executeCrossLayerTransfer,
    getProtocolHealth,
    getAllConnectedProtocols,
  };
}

// React component for Layer 2 dashboard
export interface Layer2DashboardProps {
  config: Layer2Config;
}

export const Layer2Dashboard: React.FC<Layer2DashboardProps> = ({ config }) => {
  const {
    protocolStatuses,
    loading,
    error,
    lightning,
    rgb,
    initializeAllProtocols,
    executeCrossLayerTransfer,
    getProtocolHealth,
    getAllConnectedProtocols,
  } = useLayer2Manager(config);

  const getHealthColor = (health: string) => {
    switch (health) {
      case 'healthy': return 'text-green-600';
      case 'degraded': return 'text-yellow-600';
      case 'down': return 'text-red-600';
      default: return 'text-gray-600';
    }
  };

  return (
    <div className="p-6 max-w-6xl mx-auto">
      <div className="mb-6">
        <h1 className="text-3xl font-bold text-gray-900 mb-2">Layer 2 Dashboard</h1>
        <p className="text-gray-600">Monitor and manage all Bitcoin Layer 2 protocols</p>
      </div>

      {error && (
        <div className="mb-4 p-4 bg-red-100 border border-red-400 text-red-700 rounded">
          {error}
        </div>
      )}

      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-6">
        {Object.entries(protocolStatuses).map(([protocol, status]) => {
          const health = getProtocolHealth(protocol as Layer2Protocol);
          return (
            <div key={protocol} className="bg-white rounded-lg shadow p-4 border">
              <div className="flex items-center justify-between mb-2">
                <h3 className="text-lg font-semibold capitalize">{protocol.replace('-', ' ')}</h3>
                <span className={`text-sm font-medium ${getHealthColor(health)}`}>
                  {health.toUpperCase()}
                </span>
              </div>
              <div className="text-sm text-gray-600">
                <p>Connected: {status.connected ? 'Yes' : 'No'}</p>
                <p>Initialized: {status.initialized ? 'Yes' : 'No'}</p>
                {status.blockHeight && <p>Block Height: {status.blockHeight}</p>}
                {status.peers && <p>Peers: {status.peers}</p>}
                {status.error && <p className="text-red-600">Error: {status.error}</p>}
              </div>
            </div>
          );
        })}
      </div>

      <div className="grid grid-cols-1 lg:grid-cols-2 gap-6">
        {/* Lightning Network Section */}
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-xl font-semibold mb-4">Lightning Network</h2>
          {lightning.nodeInfo && (
            <div className="space-y-2 text-sm">
              <p><span className="font-medium">Alias:</span> {lightning.nodeInfo.alias}</p>
              <p><span className="font-medium">Channels:</span> {lightning.nodeInfo.numActiveChannels}</p>
              <p><span className="font-medium">Peers:</span> {lightning.nodeInfo.numPeers}</p>
              <p><span className="font-medium">Synced:</span> {lightning.nodeInfo.synced ? 'Yes' : 'No'}</p>
            </div>
          )}
        </div>

        {/* RGB Protocol Section */}
        <div className="bg-white rounded-lg shadow p-6">
          <h2 className="text-xl font-semibold mb-4">RGB Protocol</h2>
          <div className="space-y-2 text-sm">
            <p><span className="font-medium">Assets:</span> {rgb.assets.length}</p>
            <p><span className="font-medium">Transactions:</span> {rgb.transactions.length}</p>
          </div>
        </div>
      </div>

      <div className="mt-6 flex gap-4">
        <button
          onClick={initializeAllProtocols}
          disabled={loading}
          className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 disabled:opacity-50"
        >
          {loading ? 'Initializing...' : 'Reinitialize All Protocols'}
        </button>
        
        <div className="text-sm text-gray-600 flex items-center">
          Connected Protocols: {getAllConnectedProtocols().length}/7
        </div>
      </div>
    </div>
  );
};
