import { describe, expect, it, beforeEach } from 'vitest';
import { 
  Chain, 
  Account, 
  types 
} from '@hirosystems/clarinet-sdk/dist/index';

describe('dao-core', () => {
    let chain: Chain;
    let deployer: Account;

    beforeEach(() => {
        chain = new Chain();
        deployer = chain.deployer;
    });

    it('should return correct DAO name', () => {
        const result = chain.callReadOnlyFn('dao-core', 'get-dao-name', [], deployer.address);
        expect(result.result).toBe(types.ok('Anya DAO'));
    });
});