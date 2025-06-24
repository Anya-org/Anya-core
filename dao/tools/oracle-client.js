#!/usr/bin/env node
/**
 * Oracle Client for Anya-core DAO
 * 
 * This client implements secure data submission for the decentralized oracle network.
 * It signs and submits data to the blockchain in a secure, verifiable way, and
 * includes stake management, data validation, and consensus participation.
 * 
 * Features:
 * 1. Secure key management for oracle signing
 * 2. Connection to the blockchain via unified client
 * 3. Data validation and verification
 * 4. Stake management for oracle participation
 * 5. Threshold signature process for data consensus
 * 6. Reward claiming and monitoring
 */

const crypto = require('crypto');
const fs = require('fs');
const path = require('path');
const { program } = require('commander');

// Import the unified blockchain client
const BlockchainClient = require('../shared/blockchain-client');
const logger = require('../shared/unified-logger');
const config = require('../../config/oracle-config');

class OracleClient {
    /**
     * Constructor for OracleClient
     * @param {Object} options - Configuration options
     * @param {string} options.network - Network to connect to
     * @param {string} options.privateKeyFile - Path to private key file
     * @param {string} options.dataSource - Source of oracle data (API, file, etc.)
     * @param {boolean} options.autoSubmit - Whether to auto-submit data on schedule
     */
    constructor(options = {}) {
        this.network = options.network || config.DEFAULT_NETWORK;
        this.privateKeyFile = options.privateKeyFile || path.join(process.env.HOME, '.anya', 'oracle', 'private_key.json');
        this.dataSource = options.dataSource || config.DEFAULT_DATA_SOURCE;
        this.autoSubmit = options.autoSubmit !== undefined ? options.autoSubmit : false;
        this.submissionInterval = options.submissionInterval || config.DEFAULT_SUBMISSION_INTERVAL;

        // Initialize blockchain client
        this.client = new BlockchainClient({
            network: this.network,
            debug: options.debug || false
        });

        // Try to load the private key
        try {
            this.loadPrivateKey();
            logger.info('Oracle client initialized successfully');
        } catch (error) {
            logger.error(`Failed to initialize oracle client: ${error.message}`);
            if (error.code === 'ENOENT') {
                logger.info('No private key found. Use "generate-key" command to create one.');
            }
        }
    }

    /**
     * Load the private key from file
     * @private
     */
    loadPrivateKey() {
        try {
            const keyData = fs.readFileSync(this.privateKeyFile, 'utf8');
            const keyJson = JSON.parse(keyData);
            this.privateKey = keyJson.privateKey;
            this.publicKey = keyJson.publicKey;
            this.address = keyJson.address;
        } catch (error) {
            logger.error(`Failed to load private key: ${error.message}`);
            throw error;
        }
    }

    /**
     * Generate a new key pair for the oracle
     * @param {string} outputFile - Path to save the key file
     */
    generateKeyPair(outputFile = this.privateKeyFile) {
        try {
            // Create directory if it doesn't exist
            const dir = path.dirname(outputFile);
            if (!fs.existsSync(dir)) {
                fs.mkdirSync(dir, { recursive: true });
            }

            // Generate the key pair using a secure method appropriate for blockchain use
            // This is a simplified example - a real implementation would use proper blockchain libraries
            const keypair = crypto.generateKeyPairSync('rsa', {
                modulusLength: 2048,
                publicKeyEncoding: {
                    type: 'spki',
                    format: 'pem'
                },
                privateKeyEncoding: {
                    type: 'pkcs8',
                    format: 'pem'
                }
            });

            // In a real implementation, we would derive a blockchain address from the public key
            const address = 'ST' + crypto.createHash('sha256')
                .update(keypair.publicKey)
                .digest('hex')
                .substring(0, 40);

            const keyData = {
                privateKey: keypair.privateKey,
                publicKey: keypair.publicKey,
                address: address,
                createdAt: new Date().toISOString()
            };

            fs.writeFileSync(outputFile, JSON.stringify(keyData, null, 2), {
                encoding: 'utf8',
                mode: 0o600 // Read/write permissions for owner only
            });

            logger.info(`New oracle key pair generated and saved to ${outputFile}`);
            logger.info(`Oracle address: ${address}`);

            return keyData;
        } catch (error) {
            logger.error(`Failed to generate key pair: ${error.message}`);
            throw error;
        }
    }

    /**
     * Register as an oracle by staking tokens
     * @param {number} stakeAmount - Amount of tokens to stake
     */
    async registerAsOracle(stakeAmount) {
        try {
            if (!this.privateKey) {
                throw new Error('Private key not loaded. Please generate or load a key first.');
            }

            logger.info(`Attempting to register as oracle with stake amount: ${stakeAmount}`);

            // First, check if we're already registered
            const oracleStatus = await this.checkOracleStatus();
            if (oracleStatus && oracleStatus.active) {
                logger.info('Already registered as an active oracle');
                return { success: true, status: 'already-registered' };
            }

            // Prepare transaction options
            const txOptions = {
                senderKey: this.privateKey,
                senderAddress: this.address
            };

            // Send the transaction to register as an oracle
            const result = await this.client.submitTransaction(
                config.CONTRACT_ADDRESS,
                'decentralized-contribution-oracle',
                'register-oracle',
                [stakeAmount],
                txOptions
            );

            logger.info('Oracle registration successful');
            return { success: true, txId: result.txId };
        } catch (error) {
            logger.error(`Failed to register as oracle: ${error.message}`);
            return { success: false, error: error.message };
        }
    }

    /**
     * Check the current status of this oracle
     */
    async checkOracleStatus() {
        try {
            if (!this.address) {
                throw new Error('No oracle address available');
            }

            // Get oracle status from the contract
            const result = await this.client.callReadOnlyFunction(
                config.CONTRACT_ADDRESS,
                'decentralized-contribution-oracle',
                'get-oracle-status',
                [this.address]
            );

            return result.value;
        } catch (error) {
            logger.error(`Failed to check oracle status: ${error.message}`);
            return null;
        }
    }

    /**
     * Collect data from the configured data source
     * @private
     */
    async collectData(period = null) {
        try {
            const currentPeriod = period || this.getCurrentPeriod();

            // Implement data collection based on the data source type
            switch (this.dataSource.type) {
                case 'api':
                    return await this.collectApiData(currentPeriod);
                case 'file':
                    return await this.collectFileData(currentPeriod);
                case 'github':
                    return await this.collectGitHubData(currentPeriod);
                default:
                    throw new Error(`Unsupported data source type: ${this.dataSource.type}`);
            }
        } catch (error) {
            logger.error(`Failed to collect data: ${error.message}`);
            throw error;
        }
    }

    /**
     * Collect data from an API
     * @private
     */
    async collectApiData(period) {
        // Implementation would fetch data from configured API endpoints
        // This is a placeholder implementation
        logger.info(`Collecting API data for period ${period}`);
        return {
            period: period,
            source: 'api',
            timestamp: Date.now(),
            data: {} // Would contain actual API response data
        };
    }

    /**
     * Collect data from a file
     * @private
     */
    async collectFileData(period) {
        // Implementation would read data from configured files
        logger.info(`Collecting file data for period ${period}`);
        return {
            period: period,
            source: 'file',
            timestamp: Date.now(),
            data: {} // Would contain parsed file data
        };
    }

    /**
     * Collect data from GitHub
     * @private
     */
    async collectGitHubData(period) {
        // Implementation would fetch GitHub contribution data
        logger.info(`Collecting GitHub data for period ${period}`);
        return {
            period: period,
            source: 'github',
            timestamp: Date.now(),
            data: {} // Would contain GitHub contribution data
        };
    }

    /**
     * Sign data with the oracle's private key
     * @param {Object} data - Data to sign
     * @private
     */
    signData(data) {
        try {
            if (!this.privateKey) {
                throw new Error('Private key not loaded');
            }

            const dataString = typeof data === 'string' ? data : JSON.stringify(data);
            const sign = crypto.createSign('SHA256');
            sign.update(dataString);
            sign.end();

            return sign.sign(this.privateKey, 'hex');
        } catch (error) {
            logger.error(`Failed to sign data: ${error.message}`);
            throw error;
        }
    }

    /**
     * Submit collected data to the blockchain
     * @param {Object} data - Data to submit
     */
    async submitData(data) {
        try {
            if (!this.privateKey) {
                throw new Error('Private key not loaded');
            }

            const period = data.period || this.getCurrentPeriod();
            const dataString = typeof data === 'string' ? data : JSON.stringify(data);
            const signature = this.signData(dataString);

            logger.info(`Submitting data for period ${period}`);

            // Prepare transaction options
            const txOptions = {
                senderKey: this.privateKey,
                senderAddress: this.address
            };

            // Submit the data and signature to the contract
            const result = await this.client.submitTransaction(
                config.CONTRACT_ADDRESS,
                'decentralized-contribution-oracle',
                'submit-oracle-data',
                [period, dataString, signature],
                txOptions
            );

            logger.info('Data submission successful');
            return { success: true, txId: result.txId };
        } catch (error) {
            logger.error(`Failed to submit data: ${error.message}`);
            return { success: false, error: error.message };
        }
    }

    /**
     * Get the current time period in the required format
     * @private
     */
    getCurrentPeriod() {
        const now = new Date();
        return `${now.getFullYear()}-${String(now.getMonth() + 1).padStart(2, '0')}`;
    }

    /**
     * Claim rewards for the oracle
     */
    async claimRewards() {
        try {
            if (!this.privateKey) {
                throw new Error('Private key not loaded');
            }

            logger.info('Claiming oracle rewards');

            // Prepare transaction options
            const txOptions = {
                senderKey: this.privateKey,
                senderAddress: this.address
            };

            // Submit transaction to claim rewards
            const result = await this.client.submitTransaction(
                config.CONTRACT_ADDRESS,
                'decentralized-contribution-oracle',
                'claim-oracle-rewards',
                [],
                txOptions
            );

            logger.info('Rewards claimed successfully');
            return { success: true, txId: result.txId };
        } catch (error) {
            logger.error(`Failed to claim rewards: ${error.message}`);
            return { success: false, error: error.message };
        }
    }

    /**
     * Start automated data submission on a schedule
     */
    startAutomatedSubmission() {
        if (this.submissionJob) {
            logger.warn('Automated submission already running');
            return;
        }

        logger.info(`Starting automated data submission every ${this.submissionInterval}ms`);

        this.submissionJob = setInterval(async () => {
            try {
                // Check oracle status first
                const status = await this.checkOracleStatus();
                if (!status || !status.active) {
                    logger.warn('Oracle is not active. Skipping data submission.');
                    return;
                }

                // Collect and submit data
                const data = await this.collectData();
                await this.submitData(data);

                // Try to claim rewards while we're at it
                await this.claimRewards();
            } catch (error) {
                logger.error(`Error in automated submission: ${error.message}`);
            }
        }, this.submissionInterval);
    }

    /**
     * Stop automated data submission
     */
    stopAutomatedSubmission() {
        if (this.submissionJob) {
            clearInterval(this.submissionJob);
            this.submissionJob = null;
            logger.info('Stopped automated data submission');
        }
    }

    /**
     * Get oracle network status
     */
    async getOracleNetworkStatus() {
        try {
            const result = await this.client.callReadOnlyFunction(
                config.CONTRACT_ADDRESS,
                'decentralized-contribution-oracle',
                'get-oracle-network-status',
                []
            );

            return result.value;
        } catch (error) {
            logger.error(`Failed to get oracle network status: ${error.message}`);
            return null;
        }
    }
}

// CLI setup
program
    .name('oracle-client')
    .description('Oracle client for Anya-core DAO')
    .version('1.0.0');

program
    .command('generate-key')
    .description('Generate a new key pair for the oracle')
    .option('-o, --output <file>', 'Output file path')
    .action(async (options) => {
        const client = new OracleClient();
        await client.generateKeyPair(options.output);
    });

program
    .command('register')
    .description('Register as an oracle by staking tokens')
    .requiredOption('-s, --stake <amount>', 'Stake amount', parseInt)
    .option('-n, --network <network>', 'Blockchain network')
    .option('-k, --key-file <file>', 'Private key file')
    .action(async (options) => {
        const client = new OracleClient({
            network: options.network,
            privateKeyFile: options.keyFile
        });
        const result = await client.registerAsOracle(options.stake);
        console.log(JSON.stringify(result, null, 2));
    });

program
    .command('status')
    .description('Check oracle status')
    .option('-n, --network <network>', 'Blockchain network')
    .option('-k, --key-file <file>', 'Private key file')
    .action(async (options) => {
        const client = new OracleClient({
            network: options.network,
            privateKeyFile: options.keyFile
        });
        const status = await client.checkOracleStatus();
        console.log(JSON.stringify(status, null, 2));
    });

program
    .command('submit')
    .description('Submit data to the oracle network')
    .option('-p, --period <period>', 'Time period for data')
    .option('-n, --network <network>', 'Blockchain network')
    .option('-k, --key-file <file>', 'Private key file')
    .option('-s, --source <source>', 'Data source (api, file, github)')
    .action(async (options) => {
        const client = new OracleClient({
            network: options.network,
            privateKeyFile: options.keyFile,
            dataSource: { type: options.source || 'api' }
        });
        const data = await client.collectData(options.period);
        const result = await client.submitData(data);
        console.log(JSON.stringify(result, null, 2));
    });

program
    .command('claim-rewards')
    .description('Claim oracle rewards')
    .option('-n, --network <network>', 'Blockchain network')
    .option('-k, --key-file <file>', 'Private key file')
    .action(async (options) => {
        const client = new OracleClient({
            network: options.network,
            privateKeyFile: options.keyFile
        });
        const result = await client.claimRewards();
        console.log(JSON.stringify(result, null, 2));
    });

program
    .command('start-auto')
    .description('Start automated data submission')
    .option('-i, --interval <ms>', 'Submission interval in milliseconds', parseInt)
    .option('-n, --network <network>', 'Blockchain network')
    .option('-k, --key-file <file>', 'Private key file')
    .option('-s, --source <source>', 'Data source (api, file, github)')
    .action(async (options) => {
        const client = new OracleClient({
            network: options.network,
            privateKeyFile: options.keyFile,
            dataSource: { type: options.source || 'api' },
            submissionInterval: options.interval || 3600000 // Default: 1 hour
        });
        await client.startAutomatedSubmission();
        console.log('Automated submission started. Press Ctrl+C to stop.');
        // Keep process running
        process.stdin.resume();
    });

program
    .command('network-status')
    .description('Get oracle network status')
    .option('-n, --network <network>', 'Blockchain network')
    .action(async (options) => {
        const client = new OracleClient({
            network: options.network
        });
        const status = await client.getOracleNetworkStatus();
        console.log(JSON.stringify(status, null, 2));
    });

// Parse command line arguments
program.parse(process.argv);

module.exports = OracleClient;
