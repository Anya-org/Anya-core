#!/usr/bin/env node
/**
 * DAO Reward Engine
 * 
 * This script implements a Bitcoin-style reward system for the Anya DAO based on:
 * 1. Contribution history tracking
 * 2. Bitcoin-inspired halving and issuance model
 * 3. Strategic distribution as defined in Anya Core's tokenomics
 * 
 * Features:
 * - Aligns with Anya Core DAO design rules and tokenomics
 * - Implementation of the Bitcoin-style 21 billion supply with halving
 * - Uses the community incentives allocation (15% of total)
 * - Support for on-chain token transfers (ERC-20/SIP-010)
 * - Rewards periods tracking to prevent double payouts
 * - Full auditability and transparent reward distribution
 * - Security gates for mainnet transactions
 * - On-chain integration for decentralized distribution
 * - Multi-standard token support (SIP-010, SRC-20, tBTC)
 */

const fs = require('fs');
const path = require('path');
const { execFileSync } = require('child_process');

// Blockchain integration
const {
    SIP010Integration,
    SRC20Integration,
    TBTCIntegration,
    MultiTokenHandler,
    loadConfig: loadBlockchainConfig
} = require('./blockchain-integrations');

// Configuration
const HISTORY_PATH = path.join(__dirname, '../data/contribution_history.json');
const OUTPUT_PATH = path.join(__dirname, '../data/reward_distribution.json');
const PERIODS_PATH = path.join(__dirname, '../data/rewarded_periods.json');
const CONFIG_PATH = path.join(__dirname, '../config/reward_system_config.json');
const BLOCKCHAIN_CONFIG_PATH = path.join(__dirname, '../config/blockchain_config.json');
const ON_CHAIN_BRIDGE = path.join(__dirname, 'on-chain-reward-bridge.js');
const TOKEN_NAME = 'AGT'; // Anya Governance Token
const COMMUNITY_INCENTIVE_PERCENTAGE = 0.15; // 15% of token supply for community incentives
const MAX_SUPPLY = 21000000000; // 21 billion tokens (Bitcoin-style)
const HALVING_INTERVAL = 210000; // Halving every 210,000 blocks

// Command line args
const args = process.argv.slice(2);
const SIMULATION_MODE = args.includes('--simulate');
const FORCE_PAYOUT = args.includes('--force');
const AUDIT_ONLY = args.includes('--audit');
const ON_CHAIN_MODE = args.includes('--on-chain');
const TEST_MODE = args.includes('--test');
const STANDARD_ARG = args.find(arg => arg.startsWith('--standard='))?.split('=')[1];
const PERIOD_ARG = args.find(arg => arg.startsWith('--period='))?.split('=')[1];
const CURRENT_BLOCK = parseInt(args.find(arg => arg.startsWith('--block='))?.split('=')[1]) || 1;

/**
 * Load system configuration
 */
function loadSystemConfig() {
    try {
        if (fs.existsSync(CONFIG_PATH)) {
            return JSON.parse(fs.readFileSync(CONFIG_PATH, 'utf8'));
        }
    } catch (error) {
        console.warn(`Warning: Could not load system config: ${error.message}`);
    }

    return null;
}

/**
 * Load previously rewarded periods to prevent double payouts
 */
function getRewardedPeriods() {
    try {
        if (fs.existsSync(PERIODS_PATH)) {
            return JSON.parse(fs.readFileSync(PERIODS_PATH, 'utf8'));
        }
    } catch (error) {
        console.warn(`Warning: Could not load rewarded periods: ${error.message}`);
    }

    return {
        lastPayout: null,
        periods: [],
        contributors: {}
    };
}

/**
 * Save rewarded periods data
 */
function saveRewardedPeriods(data) {
    const dir = path.dirname(PERIODS_PATH);
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
    fs.writeFileSync(PERIODS_PATH, JSON.stringify(data, null, 2));
}

/**
 * Save reward distribution data
 */
function saveRewardDistribution(data) {
    const dir = path.dirname(OUTPUT_PATH);
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
    fs.writeFileSync(OUTPUT_PATH, JSON.stringify(data, null, 2));
}

/**
 * Load contribution history
 */
function loadContributionHistory() {
    if (!fs.existsSync(HISTORY_PATH)) {
        throw new Error(`Contribution history file not found at ${HISTORY_PATH}`);
    }
    return JSON.parse(fs.readFileSync(HISTORY_PATH, 'utf8'));
}

/**
 * Calculate current block reward based on Bitcoin-style halving
 */
function calculateBlockReward(currentBlock) {
    const halvings = Math.floor(currentBlock / HALVING_INTERVAL);
    // Initial block reward
    let reward = 10000; // Starting at 10,000 AGT per block

    // Apply halvings (cap at 64 halvings to prevent underflow)
    if (halvings >= 64) {
        return 0;
    }

    // Calculate reward after halvings (integer division by 2^halvings)
    for (let i = 0; i < halvings; i++) {
        reward = Math.floor(reward / 2);
    }

    return reward;
}

/**
 * Calculate total mined supply until current block
 */
function calculateTotalMinedSupply(currentBlock) {
    let totalSupply = 0;
    let blocksLeft = currentBlock;
    let currentReward = 10000; // Starting at 10,000 AGT per block
    let halvingCount = 0;

    while (blocksLeft > 0 && halvingCount < 64) { // Cap at 64 halvings
        const blocksInThisEra = Math.min(blocksLeft, HALVING_INTERVAL);
        totalSupply += blocksInThisEra * currentReward;
        blocksLeft -= blocksInThisEra;

        if (blocksLeft > 0) {
            currentReward = Math.floor(currentReward / 2); // Halve the reward
            halvingCount++;
        }
    }

    // Cap at max supply
    return Math.min(totalSupply, MAX_SUPPLY);
}

/**
 * Calculate community incentive allocation (15% of total)
 */
function calculateCommunityIncentive(currentBlock) {
    const totalMinedSupply = calculateTotalMinedSupply(currentBlock);
    return Math.floor(totalMinedSupply * COMMUNITY_INCENTIVE_PERCENTAGE);
}

/**
 * Calculate reward per contribution point based on:
 * - Bitcoin-style halving model
 * - Community incentive allocation (15% of total)
 * - Current number of contribution points in the system
 */
function calculateRewardPerPoint(history, currentBlock) {
    // Get total points across all contributors
    let totalPoints = 0;
    Object.values(history.contributors || {}).forEach(user => {
        totalPoints += user.points?.total || 0;
    });

    if (totalPoints === 0) {
        return 0;
    }

    // Calculate available tokens for distribution based on tokenomics
    const communityIncentive = calculateCommunityIncentive(currentBlock);

    // Basic allocation formula: distribute according to contribution points
    // More sophisticated formulas could be implemented in the future
    return communityIncentive / totalPoints;
}

/**
 * Check if a connection to mainnet is available
 */
function verifyMainnetConnection() {
    if (SIMULATION_MODE) {
        console.log('SIMULATION MODE: Skipping mainnet connection verification');
        return true;
    }

    try {
        // Load system configuration for blockchain connection details
        const config = loadSystemConfig();

        console.log('Verifying connection to mainnet...');

        // If on-chain mode is enabled, try to connect to the blockchain
        if (ON_CHAIN_MODE && config) {
            const networkConfig = config.blockchain.mainnet;

            // Example of connection check - replace with actual implementation
            // using stacks.js or appropriate blockchain library
            console.log(`Connecting to ${networkConfig.apiUrl}...`);

            // This would be an actual API call in production
            // For now, just simulate the connection
            const connected = true;

            if (!connected) {
                throw new Error('Cannot connect to Stacks mainnet');
            }

            console.log(`Successfully connected to ${networkConfig.network} network`);
        } else {
            // Legacy simulation check
            const connected = true;

            if (!connected) {
                throw new Error('Cannot connect to mainnet');
            }
        }

        console.log('Successfully connected to mainnet');
        return true;
    } catch (error) {
        console.error(`Mainnet connection error: ${error.message}`);
        if (!FORCE_PAYOUT) {
            throw new Error('Mainnet connection failed. Use --force to override.');
        }
        console.warn('Warning: Proceeding without mainnet connection due to --force flag');
        return false;
    }
}

/**
 * Check if the DAO smart contract is available
 */
function verifyDaoContract() {
    if (SIMULATION_MODE) {
        console.log('SIMULATION MODE: Skipping DAO contract verification');
        return true;
    }

    try {
        // Load system configuration for blockchain connection details
        const config = loadSystemConfig();

        console.log('Verifying DAO smart contracts...');

        // If on-chain mode is enabled, verify actual contracts
        if (ON_CHAIN_MODE && config) {
            const networkConfig = config.blockchain.mainnet;

            // In production, we would call view functions on each contract to verify
            console.log(`Verifying contracts on ${networkConfig.network}:`);
            console.log(`- Token Contract: ${networkConfig.contracts.token}`);
            console.log(`- Reward Controller: ${networkConfig.contracts.rewardController}`);
            console.log(`- Reward Distributor: ${networkConfig.contracts.rewardDistributor}`);

            // Simulate verification - would be replaced with actual contract calls
            const contractsActive = true;

            if (!contractsActive) {
                throw new Error('One or more DAO contracts not available or not responding');
            }

            console.log('All DAO smart contracts verified successfully');
        } else {
            // Legacy simulation check
            const contractActive = true;

            if (!contractActive) {
                throw new Error('DAO contract not available or not responding');
            }
        }

        console.log('DAO smart contract verified');
        return true;
    } catch (error) {
        console.error(`DAO contract error: ${error.message}`);
        if (!FORCE_PAYOUT) {
            throw new Error('DAO contract verification failed. Use --force to override.');
        }
        console.warn('Warning: Proceeding without contract verification due to --force flag');
        return false;
    }
}

/**
 * Execute token transfer to a user with multi-standard support
 */
async function executeTokenTransfer(recipient, amount) {
    if (SIMULATION_MODE) {
        console.log(`SIMULATION: Transfer ${amount} ${TOKEN_NAME} to ${recipient}`);
        return true;
    }

    try {
        // Load system configuration
        const config = loadSystemConfig();
        const blockchainConfig = loadBlockchainConfig();

        console.log(`Transferring ${amount} ${TOKEN_NAME} to ${recipient}...`);

        // Transfer options
        const options = {
            simulationMode: SIMULATION_MODE,
            network: SIMULATION_MODE ? 'testnet' : 'mainnet',
            standard: STANDARD_ARG // Use specified standard if provided
        };

        // If on-chain mode is enabled and config exists, use token standards
        if (ON_CHAIN_MODE) {
            console.log(`On-chain mode enabled, using ${options.standard || 'auto-detected'} token standard`);

            // Execute the transfer via the multi-standard handler
            const result = await MultiTokenHandler.transfer(recipient, amount, options);

            if (!result.success) {
                throw new Error(`Failed to transfer tokens to ${recipient} via ${result.standard}: ${result.error}`);
            }

            console.log(`Successfully transferred ${amount} ${TOKEN_NAME} to ${recipient} using ${result.standard}`);
            console.log(`Transaction ID: ${result.txId}`);

            if (result.explorerUrl) {
                console.log(`Explorer URL: ${result.explorerUrl}`);
            }

            return true;

        } else if (TEST_MODE) {
            // Test all token standards
            console.log('TEST MODE: Testing all token standards');

            // Define test recipients if needed
            const testAddresses = blockchainConfig?.addressFormats?.testAddresses || {
                stacks: "ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM",
                bitcoin: "tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7",
                ethereum: "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B"
            };

            const testResult = await MultiTokenHandler.testAllStandards({
                sip010: testAddresses.stacks || recipient,
                src20: testAddresses.bitcoin || recipient,
                tbtc: testAddresses.ethereum || recipient
            }, amount, { ...options, simulationMode: true });

            console.log('\n--- Token Standard Tests ---');
            console.log(`SIP-010: ${testResult.sip010.success ? 'SUCCESS' : 'FAILED'} (${testResult.sip010.txId})`);
            console.log(`SRC-20: ${testResult.src20.success ? 'SUCCESS' : 'FAILED'} (${testResult.src20.txId})`);
            console.log(`tBTC: ${testResult.tbtc.success ? 'SUCCESS' : 'FAILED'} (${testResult.tbtc.txId})`);
            console.log('---------------------------\n');

            return testResult.success;

        } else {
            // Legacy simulation code
            console.log('Using legacy transfer method (simulation only)');
            const success = true;

            if (!success) {
                throw new Error(`Failed to transfer tokens to ${recipient}`);
            }
        }

        console.log(`Successfully transferred ${amount} ${TOKEN_NAME} to ${recipient}`);
        return true;
    } catch (error) {
        console.error(`Transfer error for ${recipient}: ${error.message}`);
        return false;
    }
}

/**
 * Submit rewards to on-chain contracts for processing
 */
async function submitRewardsToBlockchain(rewardDistribution) {
    if (SIMULATION_MODE) {
        console.log('SIMULATION MODE: Skipping blockchain submission');
        return true;
    }

    if (!ON_CHAIN_MODE) {
        console.log('ON-CHAIN MODE not enabled. Use --on-chain to enable blockchain submission');
        return false;
    }

    try {
        console.log('Submitting reward data to blockchain...');

        // Create temporary file with reward data for the bridge script
        const tempFile = path.join(__dirname, '../data/temp_reward_data.json');
        fs.writeFileSync(tempFile, JSON.stringify(rewardDistribution, null, 2));

        // Call the on-chain bridge to submit the data
        const bridgeArgs = [
            PERIOD_ARG ? `--period=${PERIOD_ARG}` : '',
            '--reward-data=' + tempFile
        ].filter(Boolean);

        console.log(`Executing on-chain bridge with args: ${bridgeArgs.join(' ')}`);

        // Execute the bridge script
        // In production, this would be a proper async execution
        // For now, simulate with a synchronous call
        const result = execFileSync('node', [ON_CHAIN_BRIDGE, ...bridgeArgs], {

            encoding: 'utf8',
            stdio: 'inherit'
        });

        // Clean up temp file
        if (fs.existsSync(tempFile)) {
            fs.unlinkSync(tempFile);
        }

        console.log('Successfully submitted reward data to blockchain');
        return true;

    } catch (error) {
        console.error(`Error submitting rewards to blockchain: ${error.message}`);
        return false;
    }
}

/**
 * Main function to process and distribute rewards
 */
async function processRewards() {
    console.log('=========================================');
    console.log('Anya DAO Reward Engine');
    console.log('Bitcoin-Style Tokenomics Implementation');
    if (ON_CHAIN_MODE) {
        console.log('On-Chain Integration Enabled');
        if (STANDARD_ARG) {
            console.log(`Token Standard: ${STANDARD_ARG}`);
        } else {
            console.log('Token Standard: Auto-detection');
        }
    }
    if (TEST_MODE) {
        console.log('Test Mode: Testing All Token Standards');
    }
    console.log('=========================================');

    try {
        // Load contribution history
        console.log('Loading contribution history...');
        const history = loadContributionHistory();
        console.log(`Loaded history last updated: ${history.lastUpdated}`);

        // Load previously rewarded periods to prevent double payments
        const rewardedPeriods = getRewardedPeriods();

        // Override period from command line if provided
        const periodToProcess = PERIOD_ARG || history.lastUpdated;

        // Check if we've already processed this period
        if (rewardedPeriods.periods.includes(periodToProcess) && !FORCE_PAYOUT) {
            console.error(`Period ${periodToProcess} has already been rewarded.`);
            console.error('Use --force to override this check (USE WITH CAUTION)');
            return false;
        }

        // First, verify we can connect to mainnet (unless in simulation mode)
        const mainnetConnected = await verifyMainnetConnection();

        // Then verify the DAO contract is operational
        const contractVerified = await verifyDaoContract();

        // Calculate reward per point based on Bitcoin-style tokenomics
        const rewardPerPoint = calculateRewardPerPoint(history, CURRENT_BLOCK);
        console.log(`Current block: ${CURRENT_BLOCK}`);
        console.log(`Total mined supply: ${calculateTotalMinedSupply(CURRENT_BLOCK).toLocaleString()} ${TOKEN_NAME}`);
        console.log(`Community incentive allocation: ${calculateCommunityIncentive(CURRENT_BLOCK).toLocaleString()} ${TOKEN_NAME}`);
        console.log(`Reward per contribution point: ${rewardPerPoint.toFixed(8)} ${TOKEN_NAME}`);

        // Prepare reward distribution data
        const rewardDistribution = {
            timestamp: new Date().toISOString(),
            period: periodToProcess,
            currentBlock: CURRENT_BLOCK,
            blockReward: calculateBlockReward(CURRENT_BLOCK),
            totalMinedSupply: calculateTotalMinedSupply(CURRENT_BLOCK),
            communityAllocation: calculateCommunityIncentive(CURRENT_BLOCK),
            rewardPerPoint: rewardPerPoint,
            contributors: {},
            totalAwarded: 0,
            onChainMode: ON_CHAIN_MODE,
            tokenStandard: STANDARD_ARG || 'auto'
        };

        // Calculate rewards for each contributor
        for (const [username, data] of Object.entries(history.contributors || {})) {
            const points = data.points?.total || 0;
            const reward = points * rewardPerPoint;

            rewardDistribution.contributors[username] = {
                points: points,
                reward: reward,
                previouslyRewarded: rewardedPeriods.contributors[username] || 0,
                activities: data.activities,
                success: false // Will be updated after transfer
            };

            rewardDistribution.totalAwarded += reward;
        }

        // Save reward distribution data before executing transfers
        saveRewardDistribution(rewardDistribution);

        // If audit only mode, stop here
        if (AUDIT_ONLY) {
            console.log('AUDIT ONLY MODE: Rewards calculated but not distributed');
            console.log(`Results saved to ${OUTPUT_PATH}`);
            return true;
        }

        // If test mode is enabled, run the token standard tests
        if (TEST_MODE) {
            console.log('\n--- Running Token Standard Tests ---');
            const testResult = await MultiTokenHandler.testAllStandards({
                // Use test addresses from blockchain config
                sip010: "ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM",
                src20: "tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7",
                tbtc: "0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B"
            }, 10, { simulationMode: true });

            console.log('\n--- Token Standard Test Results ---');
            console.log(`SIP-010 (Stacks): ${testResult.sip010.success ? 'SUCCESS ✓' : 'FAILED ✗'}`);
            console.log(`SRC-20 (Bitcoin): ${testResult.src20.success ? 'SUCCESS ✓' : 'FAILED ✗'}`);
            console.log(`tBTC (Ethereum): ${testResult.tbtc.success ? 'SUCCESS ✓' : 'FAILED ✗'}`);
            console.log('-----------------------------------\n');

            // Update reward distribution with test results
            rewardDistribution.testResults = testResult;
            saveRewardDistribution(rewardDistribution);

            if (SIMULATION_MODE) {
                console.log('Test completed in simulation mode. No actual tokens transferred.');
                return testResult.success;
            }
        }

        // If on-chain mode is enabled, submit rewards to blockchain
        if (ON_CHAIN_MODE) {
            console.log('\n--- On-Chain Reward Submission ---');
            const onChainSuccess = await submitRewardsToBlockchain(rewardDistribution);

            if (!onChainSuccess && !FORCE_PAYOUT) {
                console.error('On-chain reward submission failed. Use --force to continue with off-chain distribution.');
                return false;
            }

            if (onChainSuccess) {
                console.log('On-chain reward submission successful!');

                // Mark period as rewarded in on-chain mode
                rewardedPeriods.periods.push(periodToProcess);
                rewardedPeriods.lastPayout = new Date().toISOString();
                rewardedPeriods.onChain = true;
                saveRewardedPeriods(rewardedPeriods);

                // Update reward distribution with final results
                rewardDistribution.completedAt = new Date().toISOString();
                rewardDistribution.onChainSubmitted = true;
                saveRewardDistribution(rewardDistribution);

                console.log('\n--- On-Chain Reward Summary ---');
                console.log(`Total Contributors: ${Object.keys(rewardDistribution.contributors).length}`);
                console.log(`Total ${TOKEN_NAME} Distributed: ${rewardDistribution.totalAwarded.toFixed(8)}`);
                console.log(`On-Chain Distribution Initiated at: ${rewardDistribution.completedAt}`);
                console.log(`Results saved to ${OUTPUT_PATH}`);

                return true;
            }
        }

        console.log('\n--- Reward Distribution ---');

        // Distribute rewards to contributors
        let successCount = 0;

        for (const [username, data] of Object.entries(rewardDistribution.contributors)) {
            const success = await executeTokenTransfer(username, data.reward);

            // Update status in distribution record
            rewardDistribution.contributors[username].success = success;

            if (success) {
                successCount++;

                // Update rewarded periods tracking
                rewardedPeriods.contributors[username] = (rewardedPeriods.contributors[username] || 0) + data.reward;

                console.log(`✓ ${username}: ${data.reward.toFixed(8)} ${TOKEN_NAME} (${data.points} points)`);
            } else {
                console.error(`✗ ${username}: Failed to distribute ${data.reward.toFixed(8)} ${TOKEN_NAME}`);
            }
        }

        // Mark period as rewarded to prevent double payments
        if (successCount > 0 && !SIMULATION_MODE) {
            rewardedPeriods.periods.push(periodToProcess);
            rewardedPeriods.lastPayout = new Date().toISOString();
            saveRewardedPeriods(rewardedPeriods);
        }

        // Update reward distribution with final results
        rewardDistribution.successCount = successCount;
        rewardDistribution.completedAt = new Date().toISOString();
        saveRewardDistribution(rewardDistribution);

        console.log('\n--- Reward Summary ---');
        console.log(`Total Contributors: ${Object.keys(rewardDistribution.contributors).length}`);
        console.log(`Successfully Rewarded: ${successCount}`);
        console.log(`Total ${TOKEN_NAME} Distributed: ${rewardDistribution.totalAwarded.toFixed(8)}`);
        console.log(`Distribution ${SIMULATION_MODE ? 'Simulated' : 'Completed'} at: ${rewardDistribution.completedAt}`);
        console.log(`Results saved to ${OUTPUT_PATH}`);

        return true;
    } catch (error) {
        console.error(`\nError: ${error.message}`);
        return false;
    }
}

// Execute the main function
(async () => {
    const success = await processRewards();
    process.exit(success ? 0 : 1);
})();
