#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# [AIS-3] Security audit tool installation
cargo install --git https://github.com/anya-org/audit-tool \
    --features "strict-mode bitcoin-compliance" \
    --locked --force 