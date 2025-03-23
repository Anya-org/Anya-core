## Updated Workspace Structure (v2.5-compliant)

```mermaid
graph TD
    A[Root Workspace] --> B[anya-core]
    A --> C[anya-bitcoin]
    A --> D[anya-enterprise]
    A --> E[anya-extensions]
    
    B --> F[Security Layer]
    C --> G[Bitcoin Protocol]
    D --> H[Enterprise Features]
    E --> I[Extension System]
    
    style A fill:#f9f,stroke:#333
```

**Key Changes:**
- Removed invalid mobile/web5 paths
- Unified cryptographic dependencies
- Added workspace.package metadata inheritance
- Standardized BIP feature flags across all crates

## Dependency Resolution Matrix
| Crate | BIP 341 | BIP 342 | PSBT | Miniscript | Taproot |
|-------|---------|---------|------|------------|---------|
| anya-core | ✅ | ✅ | ✅ | ✅ | ✅ |
| anya-bitcoin | ✅ | ✅ | ✅ | ✅ | ✅ |
| anya-enterprise | ✅ | ✅ | ✅ | 🔜 | ✅ |
| anya-extensions | ✅ | ✅ | ✅ | ✅ | ✅ | 