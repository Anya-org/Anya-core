/**
 * @fileoverview Unified Configuration for DAO Tools
 * 
 * This module provides centralized configuration management for all DAO tools,
 * supporting multiple environments, secure credential handling, and dynamic config updates.
 */

const fs = require('fs');
const path = require('path');
const os = require('os');

// Default configuration paths
const DEFAULT_CONFIG_PATH = path.join(__dirname, 'config.json');
const USER_CONFIG_PATH = path.join(os.homedir(), '.anya', 'config', 'dao-config.json');

// Load environment variables
const NODE_ENV = process.env.NODE_ENV || 'development';

/**
 * Base configuration class for DAO tools
 */
class ConfigurationManager {
    /**
     * Constructor for ConfigurationManager
     * @param {Object} options - Configuration options
     * @param {string} options.configPath - Path to config file
     * @param {string} options.environment - Environment (development, staging, production)
     * @param {boolean} options.allowOverride - Allow environment variables to override config
     */
    constructor(options = {}) {
        this.configPath = options.configPath || DEFAULT_CONFIG_PATH;
        this.environment = options.environment || NODE_ENV;
        this.allowOverride = options.allowOverride !== undefined ? options.allowOverride : true;
        this.config = {};

        // Load configurations in order of precedence
        this._loadDefaultConfig();
        this._loadEnvironmentConfig();
        this._loadUserConfig();

        if (this.allowOverride) {
            this._applyEnvironmentOverrides();
        }
    }

    /**
     * Load default configuration
     * @private
     */
    _loadDefaultConfig() {
        try {
            if (fs.existsSync(this.configPath)) {
                const configData = fs.readFileSync(this.configPath, 'utf8');
                const configJson = JSON.parse(configData);
                this.config = { ...configJson };
            }
        } catch (error) {
            console.warn(`Failed to load default config: ${error.message}`);
        }
    }

    /**
     * Load environment-specific configuration
     * @private
     */
    _loadEnvironmentConfig() {
        try {
            const envConfigPath = this.configPath.replace('.json', `.${this.environment}.json`);
            if (fs.existsSync(envConfigPath)) {
                const envConfigData = fs.readFileSync(envConfigPath, 'utf8');
                const envConfigJson = JSON.parse(envConfigData);
                this.config = { ...this.config, ...envConfigJson };
            }
        } catch (error) {
            console.warn(`Failed to load environment config: ${error.message}`);
        }
    }

    /**
     * Load user-specific configuration
     * @private
     */
    _loadUserConfig() {
        try {
            if (fs.existsSync(USER_CONFIG_PATH)) {
                const userConfigData = fs.readFileSync(USER_CONFIG_PATH, 'utf8');
                const userConfigJson = JSON.parse(userConfigData);
                this.config = { ...this.config, ...userConfigJson };
            }
        } catch (error) {
            console.warn(`Failed to load user config: ${error.message}`);
        }
    }

    /**
     * Apply environment variable overrides
     * @private
     */
    _applyEnvironmentOverrides() {
        const ENV_PREFIX = 'ANYA_DAO_';

        // Iterate through environment variables that start with the prefix
        Object.keys(process.env)
            .filter(key => key.startsWith(ENV_PREFIX))
            .forEach(key => {
                const configPath = key.replace(ENV_PREFIX, '').toLowerCase().split('_');
                let current = this.config;

                // Navigate to the correct location in the config object
                for (let i = 0; i < configPath.length - 1; i++) {
                    const segment = configPath[i];
                    if (!current[segment]) {
                        current[segment] = {};
                    }
                    current = current[segment];
                }

                // Set the final property
                const finalProp = configPath[configPath.length - 1];
                current[finalProp] = this._parseEnvValue(process.env[key]);
            });
    }

    /**
     * Parse environment variable value to the correct type
     * @param {string} value - Environment variable value
     * @returns {any} - Parsed value
     * @private
     */
    _parseEnvValue(value) {
        // Handle boolean values
        if (value.toLowerCase() === 'true') return true;
        if (value.toLowerCase() === 'false') return false;

        // Handle numeric values
        if (!isNaN(value) && value.trim() !== '') return Number(value);

        // Handle JSON values
        try {
            if ((value.startsWith('{') && value.endsWith('}')) ||
                (value.startsWith('[') && value.endsWith(']'))) {
                return JSON.parse(value);
            }
        } catch (e) {
            // If it's not valid JSON, return as string
        }

        // Default to string
        return value;
    }

    /**
     * Get configuration value
     * @param {string} key - Configuration key (dot notation supported)
     * @param {any} defaultValue - Default value if key not found
     * @returns {any} - Configuration value
     */
    get(key, defaultValue = null) {
        if (!key) return defaultValue;

        const keyPath = key.split('.');
        let value = this.config;

        for (const segment of keyPath) {
            if (value === undefined || value === null) return defaultValue;
            value = value[segment];
        }

        return value !== undefined ? value : defaultValue;
    }

    /**
     * Set configuration value
     * @param {string} key - Configuration key (dot notation supported)
     * @param {any} value - Value to set
     */
    set(key, value) {
        if (!key) return;

        const keyPath = key.split('.');
        let current = this.config;

        for (let i = 0; i < keyPath.length - 1; i++) {
            const segment = keyPath[i];
            if (segment === "__proto__" || segment === "constructor") {
                console.warn(`Blocked attempt to set dangerous property: ${segment}`);
                return;
            }
            if (!current[segment]) {
                current[segment] = {};
            }
            current = current[segment];
        }

        const lastSegment = keyPath[keyPath.length - 1];
        if (lastSegment === "__proto__" || lastSegment === "constructor") {
            console.warn(`Blocked attempt to set dangerous property: ${lastSegment}`);
            return;
        }
        current[lastSegment] = value;
    }

    /**
     * Save configuration to file
     * @param {string} filePath - Path to save config file
     * @returns {boolean} - Success status
     */
    save(filePath = USER_CONFIG_PATH) {
        try {
            // Create directory if it doesn't exist
            const dir = path.dirname(filePath);
            if (!fs.existsSync(dir)) {
                fs.mkdirSync(dir, { recursive: true });
            }

            fs.writeFileSync(filePath, JSON.stringify(this.config, null, 2));
            return true;
        } catch (error) {
            console.error(`Failed to save config: ${error.message}`);
            return false;
        }
    }

    /**
     * Reset configuration to defaults
     */
    reset() {
        this.config = {};
        this._loadDefaultConfig();
        this._loadEnvironmentConfig();
    }
}

/**
 * DAO specific configuration manager
 * Extends the base configuration with DAO-specific settings
 */
class DAOConfigurationManager extends ConfigurationManager {
    /**
     * Constructor for DAOConfigurationManager
     * @param {Object} options - Configuration options
     */
    constructor(options = {}) {
        const defaultOptions = {
            configPath: path.join(__dirname, '../config/dao-config.json'),
            ...options
        };
        super(defaultOptions);

        // Set up default configurations if they don't exist
        this._ensureDefaults();
    }

    /**
     * Ensure default configurations exist
     * @private
     */
    _ensureDefaults() {
        // Network defaults
        if (!this.get('networks')) {
            this.set('networks', {
                mainnet: {
                    apiUrl: 'https://stacks-node-api.mainnet.stacks.co',
                    explorerUrl: 'https://explorer.stacks.co'
                },
                testnet: {
                    apiUrl: 'https://stacks-node-api.testnet.stacks.co',
                    explorerUrl: 'https://explorer.stacks.co'
                },
                regtest: {
                    apiUrl: 'http://localhost:20443',
                    explorerUrl: 'http://localhost:8000'
                },
                mocknet: {
                    apiUrl: 'http://localhost:3999',
                    explorerUrl: 'http://localhost:8000'
                }
            });
        }

        // Contract defaults
        if (!this.get('contracts')) {
            this.set('contracts', {
                daoGovernance: 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.dao-governance',
                multiSigGovernance: 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.multi-sig-governance',
                treasuryManagement: 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.treasury-management',
                decentralizedTreasuryManagement: 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.decentralized-treasury-management',
                token: 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM.token-sip010'
            });
        }

        // Oracle defaults
        if (!this.get('oracle')) {
            this.set('oracle', {
                minSubmissionInterval: 3600, // 1 hour in seconds
                minStake: 1000, // 1000 tokens minimum stake
                rewardPercentage: 0.5, // 0.5% reward per valid submission
                slashPercentage: 5, // 5% slash for invalid submission
                consensusThreshold: 67 // 67% consensus required
            });
        }

        // Threshold defaults
        if (!this.get('governance')) {
            this.set('governance', {
                minQuorum: 33, // 33% participation required
                minApproval: 51, // 51% approval required
                timelockPeriod: 144 // ~24 hours at 10 min blocks
            });
        }
    }

    /**
     * Get network configuration
     * @param {string} network - Network name
     * @returns {Object} - Network configuration
     */
    getNetworkConfig(network = 'testnet') {
        return this.get(`networks.${network}`, {});
    }

    /**
     * Get contract address
     * @param {string} contractName - Contract name
     * @returns {string} - Contract address
     */
    getContractAddress(contractName) {
        return this.get(`contracts.${contractName}`);
    }

    /**
     * Get oracle configuration
     * @returns {Object} - Oracle configuration
     */
    getOracleConfig() {
        return this.get('oracle', {});
    }

    /**
     * Get governance configuration
     * @returns {Object} - Governance configuration
     */
    getGovernanceConfig() {
        return this.get('governance', {});
    }
}

// Export a singleton instance
const config = new DAOConfigurationManager();

module.exports = config;
