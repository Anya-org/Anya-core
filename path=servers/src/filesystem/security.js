const { validateTransaction } = require('@bitcoin-core/security');

function validateFileOperation(filePath) {
  // Validate Bitcoin protocol compliance
  if (filePath.endsWith('.psbt')) {
    const rawTx = fs.readFileSync(filePath);
    validateTransaction(rawTx); // From mcp-server.js
  }
  
  // Check file size limits
  const stats = fs.statSync(filePath);
  assert(stats.size <= parseInt(process.env.MAX_FILE_SIZE), 
    'File exceeds size limit');
} 