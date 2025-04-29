use std::error::Error;
// [AIS-3][BPC-3] Constant-time Merkle verification
pub fn verify_merkle_proof(tx_hash: &[u8], merkle_path: &[Vec<u8>], root: &[u8]) -> Result<bool, MerkleError> {
    let mut current = tx_hash.to_vec();
    for node in merkle_path {
        current = if current.as_slice() < node.as_slice() {
            hash_pair(&current, node)
        } else {
            hash_pair(node, &current)
        };
    }
    constant_time_eq(&current, root)
}

fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut result = 0u8;
    for (x, y) in a.iter().zip(b) {
        result |= x ^ y;
    }
    result == 0
} 
