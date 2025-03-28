def validate_transaction(tx):
    # Existing checks
    assert tx.is_valid(), "Invalid transaction structure"
    assert tx.has_witness(), "SegWit required"
    
    # Enhanced Taproot validation
    assert check_taproot_conditions(tx), "BIP 341 compliance failed"
    
    # NEW: MCP-specific context validation
    assert validate_mcp_context(tx.context), \
        "Invalid context metadata in transaction"
    
    # PSBT compatibility check
    if tx.psbt_version < 2:
        apply_psbt_v2_upgrade(tx)  # BIP-370 compliance 