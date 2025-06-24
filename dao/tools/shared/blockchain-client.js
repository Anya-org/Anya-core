/**
 * @fileoverview Unified Blockchain Interaction Utilities
 * 
 * This module provides standardized functions for interacting with the blockchain,
 * supporting both the DAO and other Anya-core components. It reduces code duplication
 * and ensures consistent error handling and logging.
 */

const { fetch } = require('node-fetch');
const fs = require('fs');
const path = require('path');
const logger = require('./unified-logger');
const config = require('../config/blockchain-config');

/**
 * BlockchainClient class for unified blockchain interactions
 */
class BlockchainClient {
    /**
     * Constructor for BlockchainClient
     * @param {Object} options - Configuration options
     * @param {string} options.network - Network to connect to ('mainnet', 'testnet', 'regtest', 'mocknet')
     * @param {string} options.apiUrl - API URL for the blockchain node
     * @param {Object} options.credentials - Credentials for authenticated requests
     * @param {boolean} options.debug - Enable debug logging
     */
    constructor(options = {}) {
        this.network = options.network || config.DEFAULT_NETWORK;
        this.apiUrl = options.apiUrl || config.NETWORKS[this.network].apiUrl;
        this.credentials = options.credentials || null;
        this.debug = options.debug || false;
        this.retryCount = options.retryCount || 3;
        this.retryDelay = options.retryDelay || 1000;

        logger.info(`Blockchain client initialized for ${this.network}`);

        if (this.debug) {
            logger.debug(`API URL: ${this.apiUrl}`);
        }
    }

    /**
     * Execute a read-only contract call
     * @param {string} contractAddress - Contract address
     * @param {string} contractName - Contract name
     * @param {string} functionName - Function name to call
     * @param {Array} args - Arguments to pass to the function
     * @returns {Promise<Object>} - Function result
     */
    async callReadOnlyFunction(contractAddress, contractName, functionName, args = []) {
        try {
            const url = `${this.apiUrl}/v2/contracts/call-read/${contractAddress}/${contractName}/${functionName}`;

            const body = {
                arguments: args.map(arg => this._formatArgument(arg)),
                sender: config.DEFAULT_SENDER_ADDRESS
            };

            if (this.debug) {
                logger.debug(`Calling read-only function: ${contractName}.${functionName}`);
                logger.debug(`With arguments: ${JSON.stringify(args)}`);
            }

            return await this._makeRequest('POST', url, body);
        } catch (error) {
            logger.error(`Error calling read-only function ${contractName}.${functionName}: ${error.message}`);
            throw new BlockchainError('READ_ONLY_CALL_ERROR', error.message, {
                contractAddress, contractName, functionName, args
            });
        }
    }

    /**
     * Submit a transaction to the blockchain
     * @param {string} contractAddress - Contract address
     * @param {string} contractName - Contract name
     * @param {string} functionName - Function name to call
     * @param {Array} args - Arguments to pass to the function
     * @param {Object} options - Transaction options
     * @param {string} options.senderKey - Private key for signing
     * @param {number} options.fee - Transaction fee
     * @param {number} options.nonce - Transaction nonce
     * @returns {Promise<Object>} - Transaction result
     */
    async submitTransaction(contractAddress, contractName, functionName, args = [], options = {}) {
        try {
            if (!options.senderKey) {
                throw new Error('Sender key is required for transactions');
            }

            const url = `${this.apiUrl}/v2/transactions`;

            const txOptions = {
                fee: options.fee || config.DEFAULT_FEE,
                nonce: options.nonce || await this._getNonce(options.senderAddress),
                sender: options.senderAddress || config.DEFAULT_SENDER_ADDRESS,
                sponsored: options.sponsored || false
            };

            const transaction = {
                contractAddress,
                contractName,
                functionName,
                functionArgs: args.map(arg => this._formatArgument(arg)),
                ...txOptions
            };

            // Sign the transaction
            const signedTx = this._signTransaction(transaction, options.senderKey);

            if (this.debug) {
                logger.debug(`Submitting transaction: ${contractName}.${functionName}`);
                logger.debug(`With arguments: ${JSON.stringify(args)}`);
            }

            return await this._makeRequest('POST', url, { signedTx });
        } catch (error) {
            logger.error(`Error submitting transaction ${contractName}.${functionName}: ${error.message}`);
            throw new BlockchainError('TRANSACTION_SUBMIT_ERROR', error.message, {
                contractAddress, contractName, functionName
            });
        }
    }

    /**
     * Get transaction status
     * @param {string} txId - Transaction ID
     * @returns {Promise<Object>} - Transaction status
     */
    async getTransactionStatus(txId) {
        try {
            const url = `${this.apiUrl}/extended/v1/tx/${txId}`;
            return await this._makeRequest('GET', url);
        } catch (error) {
            logger.error(`Error getting transaction status for ${txId}: ${error.message}`);
            throw new BlockchainError('TRANSACTION_STATUS_ERROR', error.message, { txId });
        }
    }

    /**
     * Wait for a transaction to be confirmed
     * @param {string} txId - Transaction ID
     * @param {Object} options - Options
     * @param {number} options.timeout - Maximum time to wait in milliseconds
     * @param {number} options.pollInterval - How often to check status in milliseconds
     * @returns {Promise<Object>} - Final transaction status
     */
    async waitForTransaction(txId, options = {}) {
        const timeout = options.timeout || config.DEFAULT_CONFIRMATION_TIMEOUT;
        const pollInterval = options.pollInterval || 5000;

        const startTime = Date.now();

        while (Date.now() - startTime < timeout) {
            const status = await this.getTransactionStatus(txId);

            if (status.tx_status === 'success') {
                logger.info(`Transaction ${txId} confirmed successfully`);
                return status;
            } else if (status.tx_status === 'failed') {
                logger.error(`Transaction ${txId} failed: ${status.error || 'Unknown error'}`);
                throw new BlockchainError('TRANSACTION_FAILED', status.error || 'Transaction failed', { txId, status });
            }

            logger.debug(`Waiting for transaction ${txId} to be confirmed, current status: ${status.tx_status}`);
            await new Promise(resolve => setTimeout(resolve, pollInterval));
        }

        throw new BlockchainError('TRANSACTION_TIMEOUT', 'Timeout waiting for transaction confirmation', { txId });
    }

    /**
     * Get account information
     * @param {string} address - Account address
     * @returns {Promise<Object>} - Account info
     */
    async getAccountInfo(address) {
        try {
            const url = `${this.apiUrl}/v2/accounts/${address}`;
            return await this._makeRequest('GET', url);
        } catch (error) {
            logger.error(`Error getting account info for ${address}: ${error.message}`);
            throw new BlockchainError('ACCOUNT_INFO_ERROR', error.message, { address });
        }
    }

    /**
     * Get balance for an account
     * @param {string} address - Account address
     * @returns {Promise<string>} - Account balance
     */
    async getBalance(address) {
        try {
            const accountInfo = await this.getAccountInfo(address);
            return accountInfo.balance;
        } catch (error) {
            logger.error(`Error getting balance for ${address}: ${error.message}`);
            throw new BlockchainError('BALANCE_ERROR', error.message, { address });
        }
    }

    /**
     * Get token balance for an account
     * @param {string} address - Account address
     * @param {string} contractAddress - Token contract address
     * @param {string} contractName - Token contract name
     * @returns {Promise<string>} - Token balance
     */
    async getTokenBalance(address, contractAddress, contractName) {
        try {
            return await this.callReadOnlyFunction(
                contractAddress,
                contractName,
                'get-balance',
                [{ type: 'principal', value: address }]
            );
        } catch (error) {
            logger.error(`Error getting token balance for ${address}: ${error.message}`);
            throw new BlockchainError('TOKEN_BALANCE_ERROR', error.message, { address, contractAddress, contractName });
        }
    }

    /**
     * Get the current block height
     * @returns {Promise<number>} - Current block height
     */
    async getCurrentBlockHeight() {
        try {
            const url = `${this.apiUrl}/v2/info`;
            const info = await this._makeRequest('GET', url);
            return info.stacks_tip_height;
        } catch (error) {
            logger.error(`Error getting current block height: ${error.message}`);
            throw new BlockchainError('BLOCK_HEIGHT_ERROR', error.message);
        }
    }

    /**
     * Format arguments for the blockchain API
     * @param {any} arg - Argument to format
     * @returns {Object} - Formatted argument
     * @private
     */
    _formatArgument(arg) {
        // If it's already in the right format, return it
        if (arg && typeof arg === 'object' && 'type' in arg && 'value' in arg) {
            return arg;
        }

        // Detect the type and format appropriately
        if (typeof arg === 'number') {
            return { type: 'uint', value: arg.toString() };
        } else if (typeof arg === 'string') {
            if (arg.startsWith('0x')) {
                return { type: 'buffer', value: arg };
            } else if (arg.startsWith('ST')) {
                return { type: 'principal', value: arg };
            } else {
                return { type: 'string-ascii', value: arg };
            }
        } else if (typeof arg === 'boolean') {
            return { type: 'bool', value: arg.toString() };
        } else if (Array.isArray(arg)) {
            return { type: 'list', value: arg.map(item => this._formatArgument(item)) };
        } else {
            throw new Error(`Unsupported argument type: ${typeof arg}`);
        }
    }

    /**
     * Make a request to the blockchain API with retry logic
     * @param {string} method - HTTP method
     * @param {string} url - URL to call
     * @param {Object} body - Request body
     * @returns {Promise<Object>} - Response data
     * @private
     */
    async _makeRequest(method, url, body = null) {
        let lastError = null;

        for (let attempt = 1; attempt <= this.retryCount; attempt++) {
            try {
                const options = {
                    method,
                    headers: {
                        'Content-Type': 'application/json'
                    }
                };

                if (body) {
                    options.body = JSON.stringify(body);
                }

                if (this.credentials) {
                    options.headers['Authorization'] = `Bearer ${this.credentials.token}`;
                }

                const response = await fetch(url, options);

                if (!response.ok) {
                    const errorData = await response.json().catch(() => ({ error: 'Unknown error' }));
                    throw new Error(`API error: ${response.status} ${response.statusText} - ${errorData.error || JSON.stringify(errorData)}`);
                }

                return await response.json();
            } catch (error) {
                lastError = error;

                if (attempt < this.retryCount) {
                    logger.warn(`Request failed (attempt ${attempt}/${this.retryCount}), retrying in ${this.retryDelay}ms: ${error.message}`);
                    await new Promise(resolve => setTimeout(resolve, this.retryDelay));
                    // Exponential backoff
                    this.retryDelay *= 2;
                }
            }
        }

        throw lastError;
    }

    /**
     * Get the next nonce for an address
     * @param {string} address - Account address
     * @returns {Promise<number>} - Next nonce
     * @private
     */
    async _getNonce(address) {
        const accountInfo = await this.getAccountInfo(address);
        return accountInfo.nonce;
    }

    /**
     * Sign a transaction
     * @param {Object} transaction - Transaction to sign
     * @param {string} privateKey - Private key
     * @returns {string} - Signed transaction
     * @private
     */
    _signTransaction(transaction, privateKey) {
        // In a real implementation, this would use the appropriate signing library
        // For this example, we return a placeholder
        logger.debug('Transaction would be signed here with the actual private key');
        return JSON.stringify(transaction);
    }
}

/**
 * Custom error class for blockchain errors
 */
class BlockchainError extends Error {
    /**
     * Constructor for BlockchainError
     * @param {string} code - Error code
     * @param {string} message - Error message
     * @param {Object} details - Additional error details
     */
    constructor(code, message, details = {}) {
        super(message);
        this.name = 'BlockchainError';
        this.code = code;
        this.details = details;
    }
}

module.exports = {
    BlockchainClient,
    BlockchainError
};
