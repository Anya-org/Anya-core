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
  web5Config?: any;
}

export const Layer2Provider: React.FC<Layer2ProviderProps> = ({ 
  children, 
  web5Config 
}) => {
  const [manager, setManager] = useState<Layer2Manager | null>(null);
  const [service, setService] = useState<Layer2Service | null>(null);
  const [protocols, setProtocols] = useState<Layer2Protocol[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    const initializeLayer2 = async () => {
      try {
        setIsLoading(true);
        setError(null);

        // Initialize Layer2Manager
        const layer2Manager = new Layer2Manager();
        await layer2Manager.initialize();

        // Initialize Layer2Service
        const layer2Service = new Layer2Service(web5Config);
        await layer2Service.initialize();

        // Get available protocols
        const availableProtocols = layer2Manager.getSupportedProtocols();

        setManager(layer2Manager);
        setService(layer2Service);
        setProtocols(availableProtocols);
      } catch (err) {
        console.error('Failed to initialize Layer 2 services:', err);
        setError(err instanceof Error ? err.message : 'Unknown error');
      } finally {
        setIsLoading(false);
      }
    };

    initializeLayer2();
  }, [web5Config]);

  const initializeProtocol = async (protocol: Layer2Protocol): Promise<void> => {
    if (!manager || !service) {
      throw new Error('Layer 2 services not initialized');
    }

    try {
      await manager.initializeProtocol(protocol);
      await service.initializeProtocol(protocol);
    } catch (err) {
      console.error(`Failed to initialize protocol ${protocol}:`, err);
      throw err;
    }
  };

  const getProtocolStatus = async (protocol: Layer2Protocol): Promise<any> => {
    if (!manager) {
      throw new Error('Layer 2 manager not initialized');
    }

    return await manager.getProtocolStatus(protocol);
  };

  const executeTransaction = async (protocol: Layer2Protocol, params: any): Promise<any> => {
    if (!service) {
      throw new Error('Layer 2 service not initialized');
    }

    return await service.executeTransaction(protocol, params);
  };

  const value: Layer2ContextType = {
    manager,
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
