#!/usr/bin/env node
/**
 * Test Token Standards Integration
 * 
 * This script tests the implementation of multiple token standards (SIP-010, SRC-20, tBTC)
 * for the Anya DAO reward system.
 */

const {
    loadConfig,
    SIP010Integration,
    SRC20Integration,
    TBTCIntegration,
    MultiTokenHandler
} = require('./blockchain-integrations');

// Parse command-line arguments
const args = process.argv.slice(2);
const TEST_SPECIFIC_STANDARD = args.find(arg => arg.startsWith('--standard='))?.split('=')[1];
const NETWORK_TYPE = args.find(arg => arg.startsWith('--network='))?.split('=')[1] || 'testnet';
const SIMULATION = !args.includes('--live');

async function main() {
    console.log(`\n=== Anya Core DAO Token Standards Test ===`);
    console.log(`Mode: ${SIMULATION ? 'Simulation' : 'Live'}`);
    console.log(`Network: ${NETWORK_TYPE}`);

    // Load configuration
    const config = loadConfig();

    // Create multi-token handler
    const tokenHandler = new MultiTokenHandler(config, NETWORK_TYPE, SIMULATION);

    if (TEST_SPECIFIC_STANDARD) {
        // Test a specific standard
        console.log(`\n🔍 Testing ${TEST_SPECIFIC_STANDARD} standard...`);

        const handler = tokenHandler.getHandlerForStandard(TEST_SPECIFIC_STANDARD);
        const testAddress = config.addressFormats.testAddresses[
            TEST_SPECIFIC_STANDARD === 'SIP-010' ? 'stacks' :
                TEST_SPECIFIC_STANDARD === 'SRC-20' ? 'bitcoin' : 'ethereum'
        ];

        try {
            const result = await handler.transfer(testAddress, 100);
            console.log(`✅ Transfer successful:`);
            console.log(JSON.stringify(result, null, 2));
        } catch (error) {
            console.error(`❌ Transfer failed: ${error.message}`);
        }
    } else {
        // Test all standards
        console.log(`\n🔍 Testing all token standards...`);

        try {
            const results = await tokenHandler.testAllStandards();

            console.log('\n=== Test Results ===');
            for (const [standard, result] of Object.entries(results)) {
                console.log(`\n${standard}:`);
                console.log(JSON.stringify(result, null, 2));
            }

            // Summary
            const successful = Object.values(results).filter(r => r.status === 'success').length;
            const failed = Object.values(results).filter(r => r.status === 'error').length;

            console.log(`\n=== Summary ===`);
            console.log(`✅ Successful: ${successful}`);
            console.log(`❌ Failed: ${failed}`);

            if (failed > 0) {
                process.exit(1);
            }
        } catch (error) {
            console.error(`❌ Test failed: ${error.message}`);
            process.exit(1);
        }
    }
}

// Execute main function
main().catch(error => {
    console.error(`❌ Unhandled error: ${error.message}`);
    process.exit(1);
});
