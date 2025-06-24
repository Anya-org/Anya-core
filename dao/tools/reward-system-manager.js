#!/usr/bin/env node
/**
 * On-Chain Reward System Manager
 * 
 * This script serves as a comprehensive manager for the DAO reward system.
 * It orchestrates the entire process from contribution tracking through
 * on-chain submission to reward distribution.
 * 
 * Features:
 * 1. Full automation of the entire reward lifecycle
 * 2. Period management and scheduling
 * 3. Integration with blockchain for all on-chain operations
 * 4. Comprehensive logging and monitoring
 * 5. Secure transaction management
 * 6. Compliance and audit trail generation
 */

const fs = require('fs');
const path = require('path');
const { execSync, spawn } = require('child_process');
const { performance } = require('perf_hooks');

// Configuration
const CONFIG_PATH = path.join(__dirname, '../config/reward_system_config.json');
const LOG_PATH = path.join(__dirname, '../logs/reward_system.log');

// Tools paths
const CONTRIBUTION_TRACKER = path.join(__dirname, 'contribution-tracker.js');
const REWARD_BRIDGE = path.join(__dirname, 'on-chain-reward-bridge.js');
const REWARD_ENGINE = path.join(__dirname, 'dao-reward-engine.js');

// Command line args
const args = process.argv.slice(2);
const DRY_RUN = args.includes('--dry-run');
const FORCE_UPDATE = args.includes('--force');
const PERIOD_ARG = args.find(arg => arg.startsWith('--period='))?.split('=')[1];
const AUTO_MODE = args.includes('--auto');

// Security measure - default to simulation unless explicitly overridden
const SIMULATION_MODE = !args.includes('--mainnet');

// Default settings
const DEFAULT_CONFIG = {
    periods: {
        currentPeriod: "2025-Q2",
        schedule: [
            { id: "2025-Q1", startDate: "2025-01-01", endDate: "2025-03-31", status: "completed" },
            { id: "2025-Q2", startDate: "2025-04-01", endDate: "2025-06-30", status: "active" },
            { id: "2025-Q3", startDate: "2025-07-01", endDate: "2025-09-30", status: "scheduled" },
            { id: "2025-Q4", startDate: "2025-10-01", endDate: "2025-12-31", status: "scheduled" }
        ]
    },
    automation: {
        trackingSchedule: "0 0 1 * *", // monthly on the 1st
        submissionSchedule: "0 12 1 * *", // monthly on the 1st at noon
        distributionSchedule: "0 12 5 * *" // monthly on the 5th at noon
    },
    security: {
        requiredConfirmations: 12,
        timelock: 86400 // 24 hours in seconds
    }
};

/**
 * Load configuration from file or use defaults
 */
function loadConfig() {
    try {
        if (fs.existsSync(CONFIG_PATH)) {
            const config = JSON.parse(fs.readFileSync(CONFIG_PATH, 'utf8'));
            return { ...DEFAULT_CONFIG, ...config };
        }
    } catch (error) {
        console.warn(`Warning: Could not load config: ${error.message}`);
    }

    return DEFAULT_CONFIG;
}

/**
 * Save configuration to file
 */
function saveConfig(config) {
    const dir = path.dirname(CONFIG_PATH);
    if (!fs.existsSync(dir)) {
        fs.mkdirSync(dir, { recursive: true });
    }
    fs.writeFileSync(CONFIG_PATH, JSON.stringify(config, null, 2));
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
 * Write to system log
 */
function logToSystem(message) {
    ensureLogDirectory();
    const timestamp = new Date().toISOString();
    fs.appendFileSync(LOG_PATH, `${timestamp} - ${message}\n`);
    console.log(message);
}

/**
 * Run a script and return the result
 */
async function runScript(scriptPath, args = []) {
    return new Promise((resolve, reject) => {
        logToSystem(`Running script: ${scriptPath} ${args.join(' ')}`);

        const process = spawn('node', [scriptPath, ...args], {
            stdio: 'inherit'
        });

        process.on('close', code => {
            if (code === 0) {
                logToSystem(`Script completed successfully: ${scriptPath}`);
                resolve(true);
            } else {
                logToSystem(`Script failed with code ${code}: ${scriptPath}`);
                reject(new Error(`Script exited with code ${code}`));
            }
        });

        process.on('error', err => {
            logToSystem(`Script error: ${err.message}`);
            reject(err);
        });
    });
}

/**
 * Update current period based on date
 */
function updateCurrentPeriod(config) {
    const now = new Date();

    for (const period of config.periods.schedule) {
        const startDate = new Date(period.startDate);
        const endDate = new Date(period.endDate);

        if (now >= startDate && now <= endDate) {
            if (config.periods.currentPeriod !== period.id) {
                logToSystem(`Updating current period from ${config.periods.currentPeriod} to ${period.id}`);
                config.periods.currentPeriod = period.id;
            }

            if (period.status !== 'active') {
                logToSystem(`Updating period ${period.id} status to active`);
                period.status = 'active';
            }
        } else if (now > endDate && period.status !== 'completed') {
            logToSystem(`Marking period ${period.id} as completed`);
            period.status = 'completed';
        } else if (now < startDate && period.status !== 'scheduled') {
            logToSystem(`Marking period ${period.id} as scheduled`);
            period.status = 'scheduled';
        }
    }

    saveConfig(config);
    return config;
}

/**
 * Run the contribution tracker for the specified period
 */
async function runContributionTracker(period, force = false) {
    logToSystem(`Running contribution tracker for period ${period}`);

    const args = [`--period=${period}`];
    if (force) {
        args.push('--force');
    }

    return await runScript(CONTRIBUTION_TRACKER, args);
}

/**
 * Submit contribution data to the blockchain
 */
async function submitContributionsToBlockchain(period, dryRun = false) {
    logToSystem(`Submitting contributions for period ${period} to blockchain`);

    const args = [`--period=${period}`];
    if (dryRun) {
        args.push('--dry-run');
    } else if (SIMULATION_MODE) {
        // Allow the bridge to simulate transactions but not submit to mainnet
    } else {
        args.push('--mainnet');
    }

    return await runScript(REWARD_BRIDGE, args);
}

/**
 * Get the appropriate period to process
 */
function getPeriodToProcess(config) {
    // If period specified via command line, use that
    if (PERIOD_ARG) {
        return PERIOD_ARG;
    }

    // In auto mode, use the current period from config
    if (AUTO_MODE) {
        return config.periods.currentPeriod;
    }

    // Otherwise ask for the period
    const completedPeriods = config.periods.schedule.filter(p => p.status === 'completed');
    if (completedPeriods.length > 0) {
        // Return the most recently completed period
        completedPeriods.sort((a, b) => new Date(b.endDate) - new Date(a.endDate));
        return completedPeriods[0].id;
    }

    // If no completed periods, use the current one
    return config.periods.currentPeriod;
}

/**
 * Main function
 */
async function main() {
    try {
        logToSystem('Starting On-Chain Reward System Manager');

        // Load and update configuration
        let config = loadConfig();
        config = updateCurrentPeriod(config);

        logToSystem(`Current period: ${config.periods.currentPeriod}`);

        if (SIMULATION_MODE) {
            logToSystem('SIMULATION MODE: No actual blockchain transactions will be submitted to mainnet');
        }

        // Get the period to process
        const periodToProcess = getPeriodToProcess(config);
        logToSystem(`Processing period: ${periodToProcess}`);

        // Run the entire reward lifecycle

        // 1. Run contribution tracker to update points
        await runContributionTracker(periodToProcess, FORCE_UPDATE);

        // 2. Submit contributions to blockchain
        await submitContributionsToBlockchain(periodToProcess, DRY_RUN);

        // 3. For simulation and testing, run the off-chain reward engine
        if (DRY_RUN || SIMULATION_MODE) {
            logToSystem('Running reward engine for simulation');
            await runScript(REWARD_ENGINE, ['--simulate', `--period=${periodToProcess}`]);
        }

        logToSystem('On-Chain Reward System Manager completed successfully');

    } catch (error) {
        logToSystem(`CRITICAL ERROR: ${error.message}`);
        console.error(error);
        process.exit(1);
    }
}

// Run the main function
main();
