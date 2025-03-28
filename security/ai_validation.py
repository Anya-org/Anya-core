def validate_ai_operation(op: AiOp) -> bool:
    # Enhanced BIP checks
    if not (op.validate_bip341() and op.validate_bip370()):
        raise SecurityViolation("Taproot/PSBT compliance failed")
    
    # ZK proof verification with version check
    if not ZK_VERIFIER.verify(op.proof, circuit_version=op.circuit_version):
        raise SecurityViolation(f"Invalid ZK proof for circuit {op.circuit_version}")
    
    # Dynamic gas calculation
    gas_limit = BLOCK_GAS_LIMIT * (0.3 if op.priority == "normal" else 0.5)
    if op.gas_used > gas_limit:
        raise ResourceViolation(f"AI operation exceeds {gas_limit} gas limit")
    
    # NEW: Model checksum validation
    if not op.model_hash == EXPECTED_MODEL_HASH:
        raise SecurityViolation("Model integrity check failed")
    
    return True 