// [AIS-3][BPC-3] File Write Verification with BIP Compliance
const fs = require('fs/promises');
const crypto = require('crypto');
const { validateTaprootConfig } = require('./bitcoinValidators');

async function verifyFileWrite(filePath, expectedContent) {
    try {
        // 1. Check file existence
        const stats = await fs.stat(filePath);
        if (!stats.isFile()) {
            throw new Error('Path is not a regular file');
        }

        // 2. Validate file size
        const actualSize = stats.size;
        const expectedSize = Buffer.byteLength(expectedContent);
        if (actualSize !== expectedSize) {
            throw new Error(`Size mismatch: ${actualSize} vs ${expectedSize}`);
        }

        // 3. Content verification
        const fileContent = await fs.readFile(filePath, 'utf8');
        if (fileContent !== expectedContent) {
            throw new Error('Content mismatch');
        }

        // 4. Cryptographic integrity check
        const expectedHash = crypto.createHash('sha256')
            .update(expectedContent)
            .digest('hex');
        const actualHash = crypto.createHash('sha256')
            .update(fileContent)
            .digest('hex');
        
        if (actualHash !== expectedHash) {
            throw new Error(`Hash mismatch: ${actualHash} vs ${expectedHash}`);
        }

        // 5. BIP-specific validation for Bitcoin config files
        if (filePath.endsWith('.conf')) {
            const validationError = validateTaprootConfig(fileContent);
            if (validationError) {
                throw new Error(`BIP-341 Validation Failed: ${validationError}`);
            }
            if (!/SILENT_LEAF/.test(fileContent)) {
                throw new Error('BIP-341 Violation: Missing SILENT_LEAF pattern');
            }
        }

        return true;
    } catch (error) {
        console.error(`Verification failed: ${error.message}`);
        return false;
    }
}

// [AIT-3] Example usage with Bitcoin config
async function writeAndVerifyConfig(config) {
    const configPath = '/etc/anya-core/bitcoin.conf';
    const configContent = JSON.stringify(config);
    
    try {
        await fs.writeFile(configPath, configContent);
        const isValid = await verifyFileWrite(configPath, configContent);
        
        if (!isValid) {
            await fs.unlink(configPath); // Remove invalid config
            return false;
        }
        return true;
    } catch (error) {
        console.error(`Critical error: ${error.message}`);
        return false;
    }
} 