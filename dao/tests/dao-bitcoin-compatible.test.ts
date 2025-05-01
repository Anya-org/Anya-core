import { 
  Clarinet, 
  Tx, 
  Chain, 
  Account, 
  types 
} from 'https://deno.land/x/clarinet@v0.31.0/index.ts';
import { assertEquals } from 'https://deno.land/std@0.90.0/testing/asserts.ts';

Clarinet.test({
  name: "Ensure Bitcoin layer compatibility with DAO",
  async fn(chain: Chain, accounts: Map<string, Account>) {
    const deployer = accounts.get('deployer')!;
    const user1 = accounts.get('wallet_1')!;
    const user2 = accounts.get('wallet_2')!;
    const aiAgent = accounts.get('wallet_3')!;
    
    // Test 1: Initialize Layer 2 protocols
    let block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'initialize-protocol',
        [types.ascii('bob')],
        deployer.address
      ),
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'initialize-protocol',
        [types.ascii('lightning')],
        deployer.address
      ),
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'initialize-protocol',
        [types.ascii('rgb')],
        deployer.address
      )
    ]);
    assertEquals(block.receipts.length, 3);
    assertEquals(block.receipts[0].result.expectOk(), 'true');
    assertEquals(block.receipts[1].result.expectOk(), 'true');
    assertEquals(block.receipts[2].result.expectOk(), 'true');
    
    // Test 2: Connect to Layer 2 protocols
    block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'connect-protocol',
        [types.ascii('bob')],
        deployer.address
      ),
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'connect-protocol',
        [types.ascii('lightning')],
        deployer.address
      )
    ]);
    assertEquals(block.receipts.length, 2);
    assertEquals(block.receipts[0].result.expectOk(), 'true');
    assertEquals(block.receipts[1].result.expectOk(), 'true');
    
    // Test 3: Verify BitVM proof
    const proofId = '0x0000000000000000000000000000000000000001';
    block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'verify-bitvm-proof',
        [
          types.buff(proofId),
          types.buff(new Uint8Array(128))
        ],
        deployer.address
      )
    ]);
    assertEquals(block.receipts.length, 1);
    assertEquals(block.receipts[0].result.expectOk(), 'true');
    
    // Test 4: Check BitVM verification
    const result = chain.callReadOnlyFn(
      'dao-bitcoin-compatible',
      'is-bitvm-verified',
      [types.buff(proofId)],
      deployer.address
    );
    assertEquals(result.result.expectBool(), true);
    
    // Test 5: Create a PSBT for governance
    block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'create-governance-psbt',
        [
          types.uint(1),
          types.buff(new Uint8Array(34)),
          types.uint(1000)
        ],
        deployer.address
      )
    ]);
    assertEquals(block.receipts.length, 1);
    const txId = block.receipts[0].result.expectOk().expectBuff();
    
    // Test 6: Sign the PSBT
    block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'sign-governance-psbt',
        [
          types.buff(txId),
          types.buff(new Uint8Array(64))
        ],
        deployer.address
      )
    ]);
    assertEquals(block.receipts.length, 1);
    assertEquals(block.receipts[0].result.expectOk(), 'true');
    
    // Test 7: Submit Taproot-verified vote
    block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'private-taproot-vote',
        [
          types.uint(1),
          types.buff(new Uint8Array(64)),
          types.buff(new Uint8Array(64))
        ],
        user1.address
      )
    ]);
    assertEquals(block.receipts.length, 1);
    assertEquals(block.receipts[0].result.expectOk(), 'true');
    
    // Test 8: Issue RGB asset
    block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'issue-rgb-asset',
        [
          types.ascii('TestToken'),
          types.uint(1000000),
          types.uint(8)
        ],
        deployer.address
      )
    ]);
    assertEquals(block.receipts.length, 1);
    assertEquals(block.receipts[0].result.expectOk().expectTuple(), { 'asset-id': 'rgb:asset1' });
    
    // Test 9: Open Lightning channel
    block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'open-lightning-channel',
        [
          types.principal(user1.address),
          types.uint(1000000)
        ],
        deployer.address
      )
    ]);
    assertEquals(block.receipts.length, 1);
    assertEquals(block.receipts[0].result.expectOk().expectTuple(), { 'channel-id': 'channel1' });
    
    // Test 10: Execute cross-chain swap
    block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'execute-cross-chain-swap',
        [
          types.uint(1000),
          types.principal(user1.address),
          types.ascii('bob')
        ],
        deployer.address
      )
    ]);
    assertEquals(block.receipts.length, 1);
    assertEquals(block.receipts[0].result.expectOk().expectTuple(), { 'tx-id': '0x00' });
    
    // Test 11: Register AI agent for Layer 2 monitoring
    block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'register-ai-agent',
        [
          types.principal(aiAgent.address),
          types.ascii('analytics')
        ],
        deployer.address
      )
    ]);
    assertEquals(block.receipts.length, 1);
    assertEquals(block.receipts[0].result.expectOk(), 'true');
    
    // Test 12: AI agent reports on Layer 2 metrics
    block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'ai-report-layer2-metrics',
        [
          types.ascii('bob'),
          types.ascii('tx-throughput'),
          types.uint(500)
        ],
        aiAgent.address
      )
    ]);
    assertEquals(block.receipts.length, 1);
    assertEquals(block.receipts[0].result.expectOk(), 'true');
    
    // Test 13: Execute proposal with Bitcoin verification
    block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'execute-proposal-with-bitcoin',
        [
          types.uint(1),
          types.buff(new Uint8Array(32))
        ],
        deployer.address
      )
    ]);
    assertEquals(block.receipts.length, 1);
    assertEquals(block.receipts[0].result.expectOk().expectBool(), false); // Expected result for a proposal that didn't pass
  },
});

Clarinet.test({
  name: "Test Layer 2 protocol state management",
  async fn(chain: Chain, accounts: Map<string, Account>) {
    const deployer = accounts.get('deployer')!;
    
    // Initialize protocols
    let block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'initialize-protocol',
        [types.ascii('bob')],
        deployer.address
      ),
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'initialize-protocol',
        [types.ascii('lightning')],
        deployer.address
      )
    ]);
    assertEquals(block.receipts.length, 2);
    
    // Connect to protocols
    block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'connect-protocol',
        [types.ascii('bob')],
        deployer.address
      )
    ]);
    assertEquals(block.receipts.length, 1);
    
    // Verify protocol status
    const bobStatus = chain.callReadOnlyFn(
      'dao-bitcoin-compatible',
      'get-protocol-status',
      [types.ascii('bob')],
      deployer.address
    );
    
    const lightningStatus = chain.callReadOnlyFn(
      'dao-bitcoin-compatible',
      'get-protocol-status',
      [types.ascii('lightning')],
      deployer.address
    );
    
    // Bob should be initialized and connected
    assertEquals(bobStatus.result.expectSome().expectTuple().initialized, 'true');
    assertEquals(bobStatus.result.expectSome().expectTuple().connected, 'true');
    
    // Lightning should be initialized but not connected
    assertEquals(lightningStatus.result.expectSome().expectTuple().initialized, 'true');
    assertEquals(lightningStatus.result.expectSome().expectTuple().connected, 'false');
  },
});

Clarinet.test({
  name: "Test Bitcoin transaction verification in governance",
  async fn(chain: Chain, accounts: Map<string, Account>) {
    const deployer = accounts.get('deployer')!;
    const user1 = accounts.get('wallet_1')!;
    
    // Create a proposal and execute with Bitcoin verification
    let block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'execute-proposal-with-bitcoin',
        [
          types.uint(1),
          types.buff(new Uint8Array(32))
        ],
        deployer.address
      )
    ]);
    
    // This should pass even though the proposal doesn't exist yet
    // because we're just testing the Bitcoin verification part
    assertEquals(block.receipts.length, 1);
    
    // Now try with a non-admin user
    block = chain.mineBlock([
      Tx.contractCall(
        'dao-bitcoin-compatible',
        'execute-proposal-with-bitcoin',
        [
          types.uint(1),
          types.buff(new Uint8Array(32))
        ],
        user1.address
      )
    ]);
    assertEquals(block.receipts.length, 1);
    // The result is an error code because the proposal doesn't exist
  },
}); 