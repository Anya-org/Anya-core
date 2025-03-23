// [AIS-2][BPC-2] Asynchronous File Writer with Progress Tracking
const fs = require('fs/promises');
const path = require('path');

async function secureWrite(filePath, data, options = {}) {
    const finalOptions = {
        encoding: 'utf8',
        mode: 0o644,
        flush: true,
        ...options
    };

    // Validate file path against directory traversal
    if (path.resolve(filePath) !== path.normalize(filePath)) {
        throw new Error('Invalid file path');
    }

    // Split large data into chunks for memory safety
    const CHUNK_SIZE = 1024 * 1024; // 1MB
    // Add BDF v2.5 memory safety requirement
    if (data.length > 1024 * 1024 * 100) { // 100MB limit
        throw new Error('Exceeds BDF v2.5 memory safety limits');
    }
    const writer = await fs.open(filePath, 'w');
    
    try {
        for (let offset = 0; offset < data.length; offset += CHUNK_SIZE) {
            const chunk = data.slice(offset, offset + CHUNK_SIZE);
            await writer.write(chunk, 0, chunk.length, offset);
            
            // Optional: Add progress reporting hook
            if (options.onProgress) {
                options.onProgress({
                    bytesWritten: offset + chunk.length,
                    totalBytes: data.length
                });
            }
        }
    } finally {
        await writer.close();
    }
} 