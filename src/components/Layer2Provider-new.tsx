import React, { createContext, useContext, useState, useEffect, ReactNode } from 'react';
import { Layer2Protocol } from '../layer2/manager';
import { Layer2Service } from '../services/layer2-service';

interface Layer2ContextType {
  service: Layer2Service | null;
  protocols: Layer2Protocol[];
  isLoading: boolean;
  error: string | null;
  initializeProtocol: (protocol: Layer2Protocol) => Promise<void>;
  getProtocolStatus: (protocol: Layer2Protocol) => Promise<any>;
  executeTransaction: (protocol: Layer2Protocol, params: any) => Promise<any>;
}

const Layer2Context = createContext<Layer2ContextType | undefined>(undefined);

interface Layer2ProviderProps {
  children: ReactNode;
}

export const Layer2Provider: React.FC<Layer2ProviderProps> = ({ children }) => {
  const [service, setService] = useState<Layer2Service | null>(null);
  const [protocols, setProtocols] = useState<Layer2Protocol[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const initializeLayer2 = async () => {
      try {
        setIsLoading(true);
        setError(null);

        const layer2Service = new Layer2Service();
        await layer2Service.initialize();
        
        const availableProtocols = await layer2Service.getProtocols();
        
        setService(layer2Service);
        setProtocols(availableProtocols);
      } catch (err) {
        console.error('Failed to initialize Layer2:', err);
        setError(err instanceof Error ? err.message : 'Failed to initialize Layer2');
      } finally {
        setIsLoading(false);
      }
    };

    initializeLayer2();
  }, []);

  const initializeProtocol = async (protocol: Layer2Protocol): Promise<void> => {
    if (!service) {
      throw new Error('Layer2Service not initialized');
    }
    // Mock implementation
    console.log(`Initializing protocol: ${protocol}`);
  };

  const getProtocolStatus = async (protocol: Layer2Protocol): Promise<any> => {
    if (!service) {
      throw new Error('Layer2Service not initialized');
    }
    return await service.getProtocolStatus(protocol);
  };

  const executeTransaction = async (protocol: Layer2Protocol, params: any): Promise<any> => {
    if (!service) {
      throw new Error('Layer2Service not initialized');
    }
    // Mock implementation
    return {
      success: true,
      txId: `tx_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`,
      protocol,
      params,
    };
  };

  const value: Layer2ContextType = {
    service,
    protocols,
    isLoading,
    error,
    initializeProtocol,
    getProtocolStatus,
    executeTransaction,
  };

  return (
    <Layer2Context.Provider value={value}>
      {children}
    </Layer2Context.Provider>
  );
};

export const useLayer2 = (): Layer2ContextType => {
  const context = useContext(Layer2Context);
  if (!context) {
    throw new Error('useLayer2 must be used within a Layer2Provider');
  }
  return context;
};

export default Layer2Provider;
