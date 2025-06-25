/**
 * Blockchain Integration Utilities
 * 
 * This module provides integration with multiple Bitcoin-friendly token standards:
 * - SIP-010 on Stacks (primary standard)
 * - SRC-20 on Bitcoin L1
 * - tBTC for Ethereum/L2 bridging
 * 
 * Updated with standardized fee handling:
 * - 5% fee across all bridge operations
 * - 80% of fees go to DAO treasury
 * - 20% of fees go to community incentives
 * - DAO keeps difference from leftover fees for batch optimization
 */

const fetch = require('node-fetch');
const fs = require('fs');
const path = require('path');
const { execSync } = require('child_process');

// Configuration
const CONFIG_PATH = path.join(__dirname, '../config/blockchain_config.json');
const BRIDGE_CONFIG_PATH = path.join(__dirname, '../config/bridge_config.json');
const LOG_PATH = path.join(__dirname, '../logs/blockchain_operations.log');
const FEE_REPORT_PATH = path.join(__dirname, '../logs/fee_operations.log');

/**
 * Load blockchain configuration
 */
function loadConfig() {
    try {
        if (fs.existsSync(CONFIG_PATH)) {
            return JSON.parse(fs.readFileSync(CONFIG_PATH, 'utf8'));
        }
    } catch (error) {
        console.warn(`Warning: Could not load blockchain config: ${error.message}`);
    }

    // Return default configuration
    return {
        networks: {
            stacks: {
                mainnet: {
                    apiUrl: 'https://stacks-node-api.mainnet.stacks.co',
                    explorerUrl: 'https://explorer.stacks.co',
                    contracts: {
                        token: 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token',
                        rewardController: 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.reward-controller'
                    }
                },
                testnet: {
                    apiUrl: 'https://stacks-node-api.testnet.stacks.co',
                    explorerUrl: 'https://explorer.stacks.co',
                    contracts: {
                        token: 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token',
                        rewardController: 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.reward-controller'
                    }
                }
            },
            bitcoin: {
                mainnet: {
                    apiUrl: 'https://mempool.space/api',
                    explorerUrl: 'https://mempool.space',
                    inscriptionApiUrl: 'https://ordinals.com/api'
                },
                testnet: {
                    apiUrl: 'https://mempool.space/testnet/api',
                    explorerUrl: 'https://mempool.space/testnet',
                    inscriptionApiUrl: 'https://ordinals.com/testnet/api'
                }
            },
            ethereum: {
                mainnet: {
                    apiUrl: 'https://eth-mainnet.g.alchemy.com/v2/demo',
                    tbtcBridgeAddress: '0x18084fbA666a33d37592fA2633fD49a74DD93a88'
                },
                testnet: {
                    apiUrl: 'https://eth-goerli.g.alchemy.com/v2/demo',
                    tbtcBridgeAddress: '0x679874fbe6d8e7e19caad7d646beeb38179beaec'
                }
            }
        },
        defaultNetwork: 'stacks',
        defaultNetworkType: 'testnet',
        simulationMode: true
    };
}

/**
 * Load bridge configuration
 */
function loadBridgeConfig() {
    try {
        if (fs.existsSync(BRIDGE_CONFIG_PATH)) {
            return JSON.parse(fs.readFileSync(BRIDGE_CONFIG_PATH, 'utf8'));
        }
    } catch (error) {
        console.warn(`Warning: Could not load bridge config: ${error.message}`);
    }

    // Return default bridge configuration with standardized 5% fee
    return {
        testnet: {
            bridgeSettings: {
                stacksToBitcoin: {
                    enabled: true,
                    feeRate: 0.05,
                    minimumAmount: 1000,
                    confirmations: 6
                },
                stacksToEthereum: {
                    enabled: true,
                    feeRate: 0.05,
                    minimumAmount: 500,
                    confirmations: 12
                },
                bitcoinToStacks: {
                    enabled: true,
                    feeRate: 0.05,
                    minimumAmount: 1000,
                    confirmations: 3
                },
                ethereumToStacks: {
                    enabled: true,
                    feeRate: 0.05,
                    minimumAmount: 500,
                    confirmations: 12
                }
            },
            feeDistribution: {
                treasuryShare: 0.8,
                communityShare: 0.2
            }
        },
        mainnet: {
            bridgeSettings: {
                stacksToBitcoin: {
                    enabled: true,
                    feeRate: 0.05,
                    minimumAmount: 1000,
                    confirmations: 6
                },
                stacksToEthereum: {
                    enabled: true,
                    feeRate: 0.05,
                    minimumAmount: 500,
                    confirmations: 12
                },
                bitcoinToStacks: {
                    enabled: true,
                    feeRate: 0.05,
                    minimumAmount: 1000,
                    confirmations: 3
                },
                ethereumToStacks: {
                    enabled: true,
                    feeRate: 0.05,
                    minimumAmount: 500,
                    confirmations: 12
                }
            },
            feeDistribution: {
                treasuryShare: 0.8,
                communityShare: 0.2
            }
        }
    };
}

/**
 * Ensure log directory exists
 */
function ensureLogDirectory() {
    const dir = path.dirname(LOG_PATH);
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
}

/**
 * Write to blockchain operations log
 */
function logOperation(operation, data) {
    ensureLogDirectory();
    const timestamp = new Date().toISOString();
    const logEntry = {
        timestamp,
        operation,
        ...data
    };

    fs.appendFileSync(LOG_PATH, JSON.stringify(logEntry) + '\n');
    console.log(`[${timestamp}] ${operation}: ${JSON.stringify(data)}`);
}

/**
 * Log fee transactions for auditing and DAO treasury management
 */
function logFeeOperation(operation, data) {
    ensureLogDirectory();
    const timestamp = new Date().toISOString();
    const logEntry = {
        timestamp,
        operation,
        ...data
    };

    fs.appendFileSync(FEE_REPORT_PATH, JSON.stringify(logEntry) + '\n');
    console.log(`[${timestamp}] FEE_OPERATION: ${JSON.stringify(data)}`);
}

/**
 * Calculate fees for a bridge transaction
 * Uses the standardized 5% fee structure with 80/20 treasury/community split
 */
function calculateBridgeFee(amount, bridgeType, networkType = 'testnet') {
    // Load bridge config
    const bridgeConfig = loadBridgeConfig()[networkType];
    const bridgeSettings = bridgeConfig.bridgeSettings;
    const feeDistribution = bridgeConfig.feeDistribution;

    // Get specific bridge settings
    const bridgeTypeSettings = bridgeSettings[bridgeType];
    if (!bridgeTypeSettings) {
        throw new Error(`Unknown bridge type: ${bridgeType}`);
    }

    // Calculate fees (standardized at 5% for all bridges)
    const feeRate = bridgeTypeSettings.feeRate; // Should be 0.05
    const feeAmount = amount * feeRate;
    const netAmount = amount - feeAmount;

    // Calculate fee distribution (80% treasury, 20% community)
    const treasuryFee = feeAmount * feeDistribution.treasuryShare;
    const communityFee = feeAmount * feeDistribution.communityShare;

    return {
        originalAmount: amount,
        feeAmount: feeAmount,
        netAmount: netAmount,
        treasuryFee: treasuryFee,
        communityFee: communityFee,
        feeRate: feeRate,
        treasuryShare: feeDistribution.treasuryShare,
        communityShare: feeDistribution.communityShare
    };
}

/**
 * Base class for token integrations
 */
class TokenIntegrationBase {
    constructor(config, networkType = 'testnet', simulationMode = true) {
        this.config = config;
        this.networkType = networkType;
        this.simulationMode = simulationMode;
        this.bridgeConfig = loadBridgeConfig()[networkType];
    }

    /**
     * Log transaction details
     */
    logTransaction(data) {
        logOperation('TOKEN_TRANSACTION', {
            standard: this.getStandard(),
            network: this.networkType,
            simulation: this.simulationMode,
            ...data
        });
    }

    /**
     * Get transaction explorer URL
     */
    getExplorerUrl(txid) {
        return null; // Implemented by subclasses
    }

    /**
     * Validate address format
     */
    validateAddress(address) {
        throw new Error('Not implemented');
    }

    /**
     * Transfer tokens
     */
    async transfer(recipient, amount) {
        throw new Error('Not implemented');
    }

    /**
     * Get standard identifier
     */
    getStandard() {
        throw new Error('Not implemented');
    }

    /**
     * Apply fee structure to a transaction amount
     * Uses standardized 5% fee across all bridges
     */
    applyFees(amount, bridgeType) {
        const feeDetails = calculateBridgeFee(amount, bridgeType, this.networkType);

        // Log the fee operation for auditing
        logFeeOperation('FEE_CALCULATION', {
            standard: this.getStandard(),
            bridgeType: bridgeType,
            ...feeDetails
        });

        return feeDetails;
    }
}

/**
 * SIP-010 (Stacks) Token Integration
 */
class SIP010Integration extends TokenIntegrationBase {
    constructor(config, networkType = 'testnet', simulationMode = true) {
        super(config, networkType, simulationMode);
        this.network = config.networks.stacks[networkType];
    }

    getStandard() {
        return 'SIP-010';
    }

    validateAddress(address) {
        // Basic validation for Stacks addresses (starts with SP or ST)
        return typeof address === 'string' &&
            (address.startsWith('SP') || address.startsWith('ST')) &&
            address.length >= 32;
    }

    getExplorerUrl(txid) {
        return `${this.network.explorerUrl}/txid/${txid}`;
    }

    async transfer(recipient, amount, bridgeType = null) {
        if (!this.validateAddress(recipient)) {
            throw new Error(`Invalid Stacks address: ${recipient}`);
        }

        // If this is a cross-chain bridge operation, apply fees
        let feeDetails = null;
        let netAmount = amount;

        if (bridgeType) {
            feeDetails = this.applyFees(amount, bridgeType);
            netAmount = feeDetails.netAmount;
        }

        // In simulation mode, return a simulated transaction ID
        if (this.simulationMode) {
            const simulatedTxId = `simulation-sip010-${Date.now().toString(16)}`;
            this.logTransaction({
                txid: simulatedTxId,
                recipient,
                grossAmount: amount,
                netAmount: netAmount,
                fees: feeDetails,
                bridgeType: bridgeType,
                status: 'simulated'
            });
            return {
                txid: simulatedTxId,
                explorerUrl: this.getExplorerUrl(simulatedTxId),
                grossAmount: amount,
                netAmount: netAmount,
                fees: feeDetails,
                status: 'simulated'
            };
        }

        try {
            // Real transfer implementation would go here
            // For example, using the @stacks/transactions library

            // For test purposes, we're just logging and returning a successful result
            const txid = `sip010-${Date.now().toString(16)}`;

            this.logTransaction({
                txid,
                recipient,
                grossAmount: amount,
                netAmount: netAmount,
                fees: feeDetails,
                bridgeType: bridgeType,
                status: 'pending'
            });

            return {
                txid,
                explorerUrl: this.getExplorerUrl(txid),
                grossAmount: amount,
                netAmount: netAmount,
                fees: feeDetails,
                status: 'pending'
            };
        } catch (error) {
            this.logTransaction({
                recipient,
                amount,
                error: error.message,
                status: 'failed'
            });
            throw error;
        }
    }
}

/**
 * SRC-20 (Bitcoin L1) Token Integration
 */
class SRC20Integration extends TokenIntegrationBase {
    constructor(config, networkType = 'testnet', simulationMode = true) {
        super(config, networkType, simulationMode);
        this.network = config.networks.bitcoin[networkType];
    }

    getStandard() {
        return 'SRC-20';
    }

    validateAddress(address) {
        // Basic validation for Bitcoin addresses
        const mainnetPrefixes = ['1', '3', 'bc1'];
        const testnetPrefixes = ['m', 'n', 'tb1'];
        const prefixes = this.networkType === 'mainnet' ? mainnetPrefixes : testnetPrefixes;

        return typeof address === 'string' &&
            prefixes.some(prefix => address.startsWith(prefix)) &&
            address.length >= 26;
    }

    getExplorerUrl(txid) {
        return `${this.network.explorerUrl}/tx/${txid}`;
    }

    async transfer(recipient, amount, bridgeType = null) {
        if (!this.validateAddress(recipient)) {
            throw new Error(`Invalid Bitcoin address for ${this.networkType}: ${recipient}`);
        }

        // If this is a cross-chain bridge operation, apply fees
        let feeDetails = null;
        let netAmount = amount;

        if (bridgeType) {
            feeDetails = this.applyFees(amount, bridgeType);
            netAmount = feeDetails.netAmount;
        }

        // In simulation mode, return a simulated transaction ID
        if (this.simulationMode) {
            const simulatedTxId = `simulation-src20-${Date.now().toString(16)}`;
            this.logTransaction({
                txid: simulatedTxId,
                recipient,
                grossAmount: amount,
                netAmount: netAmount,
                fees: feeDetails,
                bridgeType: bridgeType,
                status: 'simulated'
            });
            return {
                txid: simulatedTxId,
                explorerUrl: this.getExplorerUrl(simulatedTxId),
                grossAmount: amount,
                netAmount: netAmount,
                fees: feeDetails,
                status: 'simulated'
            };
        }

        try {
            // Real SRC-20 transfer implementation would go here
            // This would involve creating a Bitcoin transaction with SRC-20 protocol

            const txid = `src20-${Date.now().toString(16)}`;

            this.logTransaction({
                txid,
                recipient,
                grossAmount: amount,
                netAmount: netAmount,
                fees: feeDetails,
                bridgeType: bridgeType,
                status: 'pending'
            });

            return {
                txid,
                explorerUrl: this.getExplorerUrl(txid),
                grossAmount: amount,
                netAmount: netAmount,
                fees: feeDetails,
                status: 'pending'
            };
        } catch (error) {
            this.logTransaction({
                recipient,
                amount,
                error: error.message,
                status: 'failed'
            });
            throw error;
        }
    }
}

/**
 * tBTC (Ethereum/L2) Token Integration
 */
class TBTCIntegration extends TokenIntegrationBase {
    constructor(config, networkType = 'testnet', simulationMode = true) {
        super(config, networkType, simulationMode);
        this.network = config.networks.ethereum[networkType];
    }

    getStandard() {
        return 'tBTC';
    }

    validateAddress(address) {
        // Basic validation for Ethereum addresses (0x followed by 40 hex chars)
        return typeof address === 'string' &&
            address.startsWith('0x') &&
            address.length === 42 &&
            /^0x[0-9a-fA-F]{40}$/.test(address);
    }

    getExplorerUrl(txid) {
        return `${this.network.explorerUrl}/tx/${txid}`;
    }

    async transfer(recipient, amount, bridgeType = null) {
        if (!this.validateAddress(recipient)) {
            throw new Error(`Invalid Ethereum address: ${recipient}`);
        }

        // If this is a cross-chain bridge operation, apply fees
        let feeDetails = null;
        let netAmount = amount;

        if (bridgeType) {
            feeDetails = this.applyFees(amount, bridgeType);
            netAmount = feeDetails.netAmount;
        }

        // In simulation mode, return a simulated transaction ID
        if (this.simulationMode) {
            const simulatedTxId = `simulation-tbtc-${Date.now().toString(16)}`;
            this.logTransaction({
                txid: simulatedTxId,
                recipient,
                grossAmount: amount,
                netAmount: netAmount,
                fees: feeDetails,
                bridgeType: bridgeType,
                status: 'simulated'
            });
            return {
                txid: simulatedTxId,
                explorerUrl: this.getExplorerUrl(simulatedTxId),
                grossAmount: amount,
                netAmount: netAmount,
                fees: feeDetails,
                status: 'simulated'
            };
        }

        try {
            // Real tBTC transfer implementation would go here
            // This would involve interacting with Ethereum contracts

            const txid = `tbtc-${Date.now().toString(16)}`;

            this.logTransaction({
                txid,
                recipient,
                grossAmount: amount,
                netAmount: netAmount,
                fees: feeDetails,
                bridgeType: bridgeType,
                status: 'pending'
            });

            return {
                txid,
                explorerUrl: this.getExplorerUrl(txid),
                grossAmount: amount,
                netAmount: netAmount,
                fees: feeDetails,
                status: 'pending'
            };
        } catch (error) {
            this.logTransaction({
                recipient,
                amount,
                error: error.message,
                status: 'failed'
            });
            throw error;
        }
    }
}

/**
 * Multi-standard Token Handler
 * Automatically selects the right token standard based on address format or explicit preference
 */
class MultiTokenHandler {
    constructor(config, networkType = 'testnet', simulationMode = true) {
        this.config = config;
        this.networkType = networkType;
        this.simulationMode = simulationMode;

        // Load bridge config
        this.bridgeConfig = loadBridgeConfig()[networkType];

        // Initialize integrations
        this.sip010 = new SIP010Integration(config, networkType, simulationMode);
        this.src20 = new SRC20Integration(config, networkType, simulationMode);
        this.tbtc = new TBTCIntegration(config, networkType, simulationMode);

        // Set standard priority (default to the order in config or a default priority)
        this.standardPriority = config.standardPriority || ["SIP-010", "SRC-20", "tBTC"];
    }

    /**
     * Auto-detect token standard based on address format
     */
    detectTokenStandard(address) {
        if (this.sip010.validateAddress(address)) {
            return "SIP-010";
        } else if (this.src20.validateAddress(address)) {
            return "SRC-20";
        } else if (this.tbtc.validateAddress(address)) {
            return "tBTC";
        }
        return null;
    }

    /**
     * Get handler for a specific standard
     */
    getHandlerForStandard(standard) {
        switch (standard) {
            case "SIP-010":
                return this.sip010;
            case "SRC-20":
                return this.src20;
            case "tBTC":
                return this.tbtc;
            default:
                throw new Error(`Unsupported token standard: ${standard}`);
        }
    }

    /**
     * Determine appropriate bridge type based on source and destination standards
     */
    getBridgeType(fromStandard, toStandard) {
        if (fromStandard === "SIP-010" && toStandard === "SRC-20") {
            return "stacksToBitcoin";
        } else if (fromStandard === "SIP-010" && toStandard === "tBTC") {
            return "stacksToEthereum";
        } else if (fromStandard === "SRC-20" && toStandard === "SIP-010") {
            return "bitcoinToStacks";
        } else if (fromStandard === "tBTC" && toStandard === "SIP-010") {
            return "ethereumToStacks";
        }
        return null;
    }

    /**
     * Check if bridge operation is allowed
     */
    checkBridgeEligibility(amount, bridgeType) {
        if (!bridgeType || !this.bridgeConfig.bridgeSettings[bridgeType]) {
            throw new Error(`Invalid or unsupported bridge type: ${bridgeType}`);
        }

        const bridgeSettings = this.bridgeConfig.bridgeSettings[bridgeType];

        // Check if bridge is enabled
        if (!bridgeSettings.enabled) {
            throw new Error(`Bridge ${bridgeType} is currently disabled`);
        }

        // Check minimum amount
        if (amount < bridgeSettings.minimumAmount) {
            throw new Error(`Amount ${amount} is below minimum required (${bridgeSettings.minimumAmount}) for ${bridgeType} bridge`);
        }

        return true;
    }

    /**
     * Transfer tokens using the appropriate standard
     * @param {string} recipient - Recipient address
     * @param {number} amount - Amount to transfer
     * @param {string} preferredStandard - Optional preferred token standard
     * @param {string} sourceStandard - Source token standard (used for bridge operations)
     */
    async transfer(recipient, amount, preferredStandard = null, sourceStandard = null) {
        // Use preferred standard if specified
        if (preferredStandard) {
            const handler = this.getHandlerForStandard(preferredStandard);
            if (!handler.validateAddress(recipient)) {
                throw new Error(`Address format ${recipient} does not match specified standard ${preferredStandard}`);
            }

            // Check if this is a bridge operation
            let bridgeType = null;
            if (sourceStandard && sourceStandard !== preferredStandard) {
                bridgeType = this.getBridgeType(sourceStandard, preferredStandard);

                if (bridgeType) {
                    // Verify bridge eligibility
                    this.checkBridgeEligibility(amount, bridgeType);
                    logOperation('BRIDGE_OPERATION', {
                        from: sourceStandard,
                        to: preferredStandard,
                        bridgeType,
                        amount,
                        recipient
                    });
                }
            }

            return await handler.transfer(recipient, amount, bridgeType);
        }

        // Auto-detect standard based on address format
        const detectedStandard = this.detectTokenStandard(recipient);
        if (detectedStandard) {
            logOperation('TOKEN_STANDARD_DETECTION', {
                address: recipient,
                detectedStandard
            });

            // Check if this is a bridge operation
            let bridgeType = null;
            if (sourceStandard && sourceStandard !== detectedStandard) {
                bridgeType = this.getBridgeType(sourceStandard, detectedStandard);

                if (bridgeType) {
                    // Verify bridge eligibility
                    this.checkBridgeEligibility(amount, bridgeType);
                    logOperation('BRIDGE_OPERATION', {
                        from: sourceStandard,
                        to: detectedStandard,
                        bridgeType,
                        amount,
                        recipient
                    });
                }
            }

            return await this.getHandlerForStandard(detectedStandard).transfer(recipient, amount, bridgeType);
        }

        throw new Error(`Could not determine token standard for address: ${recipient}`);
    }

    /**
     * Cross-chain bridge operation
     * @param {string} fromStandard - Source token standard
     * @param {string} toStandard - Destination token standard
     * @param {string} recipient - Recipient address
     * @param {number} amount - Amount to transfer
     */
    async bridge(fromStandard, toStandard, recipient, amount) {
        // Determine bridge type
        const bridgeType = this.getBridgeType(fromStandard, toStandard);
        if (!bridgeType) {
            throw new Error(`Unsupported bridge operation from ${fromStandard} to ${toStandard}`);
        }

        // Verify bridge eligibility
        this.checkBridgeEligibility(amount, bridgeType);

        // Get handler for destination standard
        const handler = this.getHandlerForStandard(toStandard);
        if (!handler.validateAddress(recipient)) {
            throw new Error(`Address ${recipient} is not a valid ${toStandard} address`);
        }

        // Log the bridge operation
        logOperation('BRIDGE_OPERATION_REQUEST', {
            fromStandard,
            toStandard,
            bridgeType,
            amount,
            recipient,
            feeRate: this.bridgeConfig.bridgeSettings[bridgeType].feeRate
        });

        // Execute the bridge transfer
        return await handler.transfer(recipient, amount, bridgeType);
    }

    /**
     * Calculate fees for a bridge operation
     * @param {number} amount - Amount to transfer
     * @param {string} fromStandard - Source token standard
     * @param {string} toStandard - Destination token standard
     */
    calculateBridgeFees(amount, fromStandard, toStandard) {
        const bridgeType = this.getBridgeType(fromStandard, toStandard);
        if (!bridgeType) {
            throw new Error(`Unsupported bridge operation from ${fromStandard} to ${toStandard}`);
        }

        // Use standardized fee structure
        return calculateBridgeFee(amount, bridgeType, this.networkType);
    }

    /**
     * Test all standard transfers for development purposes
     */
    async testAllStandards() {
        const testAddresses = {
            "SIP-010": this.config.addressFormats?.testAddresses?.stacks || "ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM",
            "SRC-20": this.config.addressFormats?.testAddresses?.bitcoin || "tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7",
            "tBTC": this.config.addressFormats?.testAddresses?.ethereum || "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B"
        };

        const testAmount = 100;
        const results = {};

        for (const standard of this.standardPriority) {
            try {
                const address = testAddresses[standard];
                console.log(`Testing ${standard} transfer to ${address}...`);

                const handler = this.getHandlerForStandard(standard);
                const result = await handler.transfer(address, testAmount);

                results[standard] = {
                    status: 'success',
                    address,
                    amount: testAmount,
                    result
                };
            } catch (error) {
                results[standard] = {
                    status: 'error',
                    error: error.message
                };
            }
        }

        // Test cross-chain bridge operations with standardized fee
        try {
            console.log("Testing bridge from SIP-010 to SRC-20...");
            const bridgeResult = await this.bridge(
                "SIP-010",
                "SRC-20",
                testAddresses["SRC-20"],
                2000
            );
            results["bridge_stacks_to_bitcoin"] = {
                status: 'success',
                result: bridgeResult
            };
        } catch (error) {
            results["bridge_stacks_to_bitcoin"] = {
                status: 'error',
                error: error.message
            };
        }

        return results;
    }
}

module.exports = {
    loadConfig,
    loadBridgeConfig,
    calculateBridgeFee,
    SIP010Integration,
    SRC20Integration,
    TBTCIntegration,
    MultiTokenHandler,
    logFeeOperation
};
