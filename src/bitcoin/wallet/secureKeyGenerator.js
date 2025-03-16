/**
 * Secure Key Generator for Bitcoin wallets
 * [AIR-1][AIS-1][BPC-1][AIT-1][RES-1]
 * 
 * This module provides secure key generation utilities for Bitcoin wallets
 * using cryptographically secure random number generation instead of Math.random()
 */

const crypto = require('crypto');

/**
 * Generate a secure random integer between min and max (both inclusive)
 * @param {number} min - Minimum value (inclusive)
 * @param {number} max - Maximum value (inclusive)
 * @returns {number} Random integer
 */
function secureRandomInt(min, max) {
  if (min > max) {
    throw new Error('Min must be less than or equal to max');
  }
  
  const range = max - min + 1;
  // Generate random bytes and convert to integer
  // We need enough bytes to cover the range
  const bytesNeeded = Math.ceil(Math.log2(range) / 8);
  const randomBytes = crypto.randomBytes(bytesNeeded);
  
  // Convert to integer
  let randomValue = 0;
  for (let i = 0; i < bytesNeeded; i++) {
    randomValue = (randomValue << 8) | randomBytes[i];
  }
  
  // Apply modulo to get within range and add min
  return (randomValue % range) + min;
}

/**
 * Generate a secure random floating point number between 0 and 1
 * @returns {number} Random float between 0 and 1
 */
function secureRandomFloat() {
  // Generate 8 random bytes and convert to float between 0 and 1
  const randomBytes = crypto.randomBytes(8);
  const randomValue = randomBytes.readUInt32LE(0) / 0x100000000;
  
  return randomValue;
}

/**
 * Generate a secure random bytes buffer of specified length
 * @param {number} length - Length of random bytes to generate
 * @returns {Buffer} Random bytes
 */
function secureRandomBytes(length) {
  return crypto.randomBytes(length);
}

/**
 * Generate a secure random hexadecimal string of specified length
 * @param {number} length - Length of output string in characters (must be even)
 * @returns {string} Random hex string
 */
function secureRandomHex(length) {
  if (length % 2 !== 0) {
    throw new Error('Length must be an even number');
  }
  
  return crypto.randomBytes(length / 2).toString('hex');
}

/**
 * Generate a secure Bitcoin entropy for wallet creation
 * Generates a random sequence between 128 and 256 bits depending on strength
 * @param {number} strength - Entropy strength (128, 160, 192, 224, or 256 bits)
 * @returns {Buffer} Entropy bytes
 */
function generateWalletEntropy(strength = 256) {
  // Validate strength
  const validStrengths = [128, 160, 192, 224, 256];
  if (!validStrengths.includes(strength)) {
    throw new Error(`Invalid entropy strength. Valid values: ${validStrengths.join(', ')}`);
  }
  
  // Generate entropy
  return crypto.randomBytes(strength / 8);
}

/**
 * Securely shuffle an array in-place using the Fisher-Yates algorithm
 * with cryptographically secure random numbers
 * @param {Array} array - Array to shuffle
 * @returns {Array} The shuffled array (same reference as input)
 */
function secureShuffleArray(array) {
  for (let i = array.length - 1; i > 0; i--) {
    // Generate a secure random index between 0 and i
    const j = secureRandomInt(0, i);
    // Swap elements
    [array[i], array[j]] = [array[j], array[i]];
  }
  return array;
}

/**
 * Generate a secure nonce for Bitcoin transactions
 * @returns {Buffer} Secure 32-byte nonce
 */
function generateSecureNonce() {
  return crypto.randomBytes(32);
}

/**
 * Generate a secure seed phrase with specified word count
 * Note: This is a simplified implementation. In practice, you would use a BIP39 library.
 * @param {number} wordCount - Number of words (12, 15, 18, 21, or 24)
 * @returns {string} Space-separated seed phrase
 */
function generateSeedPhrase(wordCount = 12) {
  // Validate word count
  const validWordCounts = [12, 15, 18, 21, 24];
  if (!validWordCounts.includes(wordCount)) {
    throw new Error(`Invalid word count. Valid values: ${validWordCounts.join(', ')}`);
  }
  
  // Simplified BIP39 wordlist (abbreviated for brevity - in practice use complete list)
  const wordlist = [
    'abandon', 'ability', 'able', 'about', 'above', 'absent', 'absorb', 'abstract',
    'absurd', 'abuse', 'access', 'accident', 'account', 'accuse', 'achieve', 'acid',
    // ... and so on (in practice, this would be the full 2048-word BIP39 list)
  ];
  
  // Calculate entropy bits based on word count (wordCount * 11 bits - checksum bits)
  const entropyBits = {
    12: 128, 15: 160, 18: 192, 21: 224, 24: 256
  }[wordCount];
  
  // Generate entropy
  const entropy = generateWalletEntropy(entropyBits);
  
  // Simplified seed phrase generation
  // (In practice, this would involve entropy + checksum bit manipulation as per BIP39)
  const seedWords = [];
  for (let i = 0; i < wordCount; i++) {
    const randomIndex = secureRandomInt(0, wordlist.length - 1);
    seedWords.push(wordlist[randomIndex]);
  }
  
  return seedWords.join(' ');
}

module.exports = {
  secureRandomInt,
  secureRandomFloat,
  secureRandomBytes,
  secureRandomHex,
  generateWalletEntropy,
  secureShuffleArray,
  generateSecureNonce,
  generateSeedPhrase,
}; 