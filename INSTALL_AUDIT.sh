#!/bin/bash
# [AIS-3] Security audit tool installation
cargo install --git https://github.com/anya-org/audit-tool \
    --features "strict-mode bitcoin-compliance" \
    --locked --force 