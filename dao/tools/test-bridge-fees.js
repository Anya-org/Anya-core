/**
 * Bridge Fee Structure Test Script
 * 
 * This script tests the standardized 5% fee structure across all bridge operations
 */

const {
    loadBridgeConfig,
    calculateBridgeFee,
    MultiTokenHandler,
    loadConfig
} = require('./blockchain-integrations');

// Load configurations
const config = loadConfig();
const bridgeConfig = loadBridgeConfig();

/**
 * Test fee calculations
 */
function testFeeCalculations() {
    console.log('\n==== TESTING FEE CALCULATIONS ====');

    const testAmounts = [1000, 5000, 10000, 50000, 100000];
    const bridgeTypes = [
        'stacksToBitcoin',
        'stacksToEthereum',
        'bitcoinToStacks',
        'ethereumToStacks'
    ];

    for (const bridgeType of bridgeTypes) {
        console.log(`\n=== Bridge Type: ${bridgeType} ===`);
        console.log('Amount\tFee\tNet\tTreasury\tCommunity');
        console.log('----------------------------------------------------');

        for (const amount of testAmounts) {
            const feeDetails = calculateBridgeFee(amount, bridgeType, 'testnet');
            console.log(
                `${amount}\t${feeDetails.feeAmount}\t${feeDetails.netAmount}\t` +
                `${feeDetails.treasuryFee}\t${feeDetails.communityFee}`
            );

            // Validate calculations
            const expectedFee = amount * 0.05;
            const expectedNetAmount = amount - expectedFee;
            const expectedTreasuryFee = expectedFee * 0.8;
            const expectedCommunityFee = expectedFee * 0.2;

            if (
                Math.abs(feeDetails.feeAmount - expectedFee) > 0.001 ||
                Math.abs(feeDetails.netAmount - expectedNetAmount) > 0.001 ||
                Math.abs(feeDetails.treasuryFee - expectedTreasuryFee) > 0.001 ||
                Math.abs(feeDetails.communityFee - expectedCommunityFee) > 0.001
            ) {
                console.error('TEST FAILED: Fee calculations do not match expected values');
                console.error('Expected:', {
                    fee: expectedFee,
                    netAmount: expectedNetAmount,
                    treasuryFee: expectedTreasuryFee,
                    communityFee: expectedCommunityFee
                });
                console.error('Actual:', feeDetails);
            }
        }
    }
}

/**
 * Test bridge operations
 */
async function testBridgeOperations() {
    console.log('\n==== TESTING BRIDGE OPERATIONS ====');

    const multiTokenHandler = new MultiTokenHandler(config, 'testnet', true);

    const testCases = [
        {
            from: 'SIP-010',
            to: 'SRC-20',
            recipient: 'tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7',
            amount: 5000,
            name: 'Stacks -> Bitcoin'
        },
        {
            from: 'SIP-010',
            to: 'tBTC',
            recipient: '0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B',
            amount: 10000,
            name: 'Stacks -> Ethereum'
        },
        {
            from: 'SRC-20',
            to: 'SIP-010',
            recipient: 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM',
            amount: 2000,
            name: 'Bitcoin -> Stacks'
        },
        {
            from: 'tBTC',
            to: 'SIP-010',
            recipient: 'ST1PQHQKV0RJXZFY1DGX8MNSNYVE3VGZJSRTPGZGM',
            amount: 1500,
            name: 'Ethereum -> Stacks'
        }
    ];

    for (const testCase of testCases) {
        console.log(`\n=== Testing Bridge: ${testCase.name} ===`);
        console.log(`From: ${testCase.from}, To: ${testCase.to}`);
        console.log(`Amount: ${testCase.amount}, Recipient: ${testCase.recipient}`);

        try {
            const result = await multiTokenHandler.bridge(
                testCase.from,
                testCase.to,
                testCase.recipient,
                testCase.amount
            );

            console.log('Bridge Result:', result);

            // Calculate expected values
            const bridgeType = multiTokenHandler.getBridgeType(testCase.from, testCase.to);
            const expectedFees = calculateBridgeFee(testCase.amount, bridgeType, 'testnet');

            console.log('Calculated Fees:');
            console.log(`- Original Amount: ${expectedFees.originalAmount}`);
            console.log(`- Fee Amount (5%): ${expectedFees.feeAmount}`);
            console.log(`- Net Amount: ${expectedFees.netAmount}`);
            console.log(`- Treasury (80%): ${expectedFees.treasuryFee}`);
            console.log(`- Community (20%): ${expectedFees.communityFee}`);
        } catch (error) {
            console.error(`Error in ${testCase.name} bridge:`, error.message);
        }
    }
}

/**
 * Test minimum amount checks
 */
async function testMinimumAmounts() {
    console.log('\n==== TESTING MINIMUM AMOUNT RULES ====');

    const multiTokenHandler = new MultiTokenHandler(config, 'testnet', true);

    const testCases = [
        {
            from: 'SIP-010',
            to: 'SRC-20',
            recipient: 'tb1qrp33g0q5c5txsp9arysrx4k6zdkfs4nce4xj0gdcccefvpysxf3q0sl5k7',
            amount: 500, // Below minimum of 1000
            name: 'Stacks -> Bitcoin (Below Minimum)'
        },
        {
            from: 'SIP-010',
            to: 'tBTC',
            recipient: '0xAb5801a7D398351b8bE11C439e05C5B3259aeC9B',
            amount: 200, // Below minimum of 500
            name: 'Stacks -> Ethereum (Below Minimum)'
        }
    ];

    for (const testCase of testCases) {
        console.log(`\n=== Testing: ${testCase.name} ===`);
        console.log(`From: ${testCase.from}, To: ${testCase.to}`);
        console.log(`Amount: ${testCase.amount}, Recipient: ${testCase.recipient}`);

        try {
            await multiTokenHandler.bridge(
                testCase.from,
                testCase.to,
                testCase.recipient,
                testCase.amount
            );
            console.error('TEST FAILED: Should have rejected amount below minimum');
        } catch (error) {
            console.log('Correctly rejected with error:', error.message);
        }
    }
}

// Run all tests
async function runTests() {
    console.log('=== BRIDGE FEE STRUCTURE TESTS ===');

    testFeeCalculations();
    await testBridgeOperations();
    await testMinimumAmounts();

    console.log('\n=== TESTS COMPLETED ===');
}

runTests().catch(err => console.error('Test error:', err));
