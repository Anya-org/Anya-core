# Updated Mobile Architecture

```mermaid
sequenceDiagram
    Mobile SDK->>+Core: PSBT Request
    Core->>+Bitcoin: Create Transaction
    Bitcoin-->>-Core: Unsigned PSBT
    Core->>+Security: Sign Transaction
    Security-->>-Core: Signed PSBT
    Core->>+Mobile SDK: Broadcast Ready
```

**Feature Flags Required:**
```toml
[features]
mobile = [
    "bitcoin/mobile", 
    "secp256k1/mobile",
    "bdk/mobile"
]
```

**Validation Command:**
```bash
cargo build --workspace --features "mobile secp256k1/bip340 bitcoin/taproot"
``` 