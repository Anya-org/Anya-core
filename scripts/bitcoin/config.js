/**
 * Bitcoin MCP Server Configuration
 * Contains all configuration parameters for the Bitcoin MCP server
 */

module.exports = {
  // Server configuration
  server: {
    port: process.env.MCP_PORT || 3000,
    host: process.env.MCP_HOST || 'localhost',
    logLevel: process.env.LOG_LEVEL || 'info'
  },
  
  // Bitcoin network configuration
  network: {
    type: process.env.BITCOIN_NETWORK || 'testnet', // 'mainnet', 'testnet', 'regtest'
    rpcUrl: process.env.BITCOIN_RPC_URL || 'http://localhost:18332',
    rpcUser: process.env.BITCOIN_RPC_USER || '',
    rpcPassword: process.env.BITCOIN_RPC_PASSWORD || ''
  },
  
  // Cryptographic settings
  crypto: {
    // Use Rust implementations when available for better performance
    preferRust: process.env.PREFER_RUST === 'true' || false,
    
    // Fallback to JavaScript if Rust is not available
    allowJsFallback: true,
    
    // Security settings
    useConstantTime: true,
    secureRandomSource: 'crypto.randomBytes'
  },
  
  // Privacy enhancements
  privacy: {
    useSilentLeaf: true,
    coinJoinMinParticipants: 3,
    defaultMixDepth: 3
  },
  
  // API limitations
  limits: {
    maxPsbtSize: 100000, // bytes
    maxScriptSize: 10000, // bytes
    maxInputs: 1000,
    maxOutputs: 1000
  }
};