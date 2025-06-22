---
title: "Roadmap"
description: "Anya Core Project Roadmap and Development Timeline"
---

# Anya Core System Roadmap (June 22, 2025)

## System Overview

- **Bitcoin core, Layer2, and async protocol support implemented, but not currently production-ready**
- **Layer2Manager**: Unified orchestration, protocol access, cross-layer operations
- **All Layer2 modules implemented, locked, and stable (sync + async APIs)**
- **Critical errors in HSM and security modules prevent successful build and test pass**
- **Test coverage is incomplete; system does not currently pass all tests**
- **Ongoing research: Arch Network integration as next-gen Layer2**

## System Module Status (June 2025)

| Module                | Status      | API      | Test Coverage | Notes                       |
|-----------------------|------------|----------|---------------|-----------------------------|
| Bitcoin Core          | Complete   | Sync/Async | Partial       | Not production-ready        |
| BobClient             | Complete   | Sync/Async | Partial       | Layer2                      |
| LiquidModule          | Complete   | Sync/Async | Partial       | Layer2                      |
| RskClient             | Complete   | Sync/Async | Partial       | Layer2                      |
| StacksClient          | Complete   | Sync/Async | Partial       | Layer2                      |
| TaprootAssetsProtocol | Complete   | Sync/Async | Partial       | Layer2                      |
| LightningNetwork      | Complete   | Sync/Async | Partial       | Layer2                      |
| StateChannel          | Complete   | Sync/Async | Partial       | Layer2                      |
| Layer2Manager         | Complete   | Sync/Async | Partial       | Orchestration               |
| HSM/Security Modules  | Failing    | Sync/Async | Failing       | Critical errors, not usable |
| ArchClient (planned)  | Research   | Planned   | N/A           | Arch Network integration    |

## Current Status & Issues

- **Async Layer2Protocol**: All Layer2 clients support async trait, but system is not fully tested or passing
- **Layer2Manager**: Unified async/sync orchestration, protocol access, cross-layer ops
- **All Layer2 modules locked and stable** (see `src/layer2/LAYER2_MODULE_LOCK.md`), but integration with HSM/security is broken
- **Critical build and logic errors in HSM and security modules**
- **System does not compile or pass all tests**
- **Benchmarks and migration guides may be outdated**
- **Arch Network research and prototyping ongoing**

## Next Steps (Q3 2025)

- Fix all compilation and logic errors in HSM and security modules
- Re-enable and pass all tests, especially for security and integration
- Update documentation and roadmap as fixes are made
- Continue Arch Network research and technical prototyping (see below)
- Address any remaining test failures (e.g., RGB asset, DAO business agent)
- Monitor and update documentation as Arch integration progresses
- Begin technical prototyping for Arch as a Layer2 protocol
- Update external docs if discrepancies are found

## Arch Network Research & Integration

Arch Network is a Bitcoin-native application and smart contract platform enabling direct, bridge-less programmable logic on Bitcoin’s base layer.

**Research Goals:**

- Evaluate Arch developer tooling, SDKs, and integration guides ([docs.arch.network](https://docs.arch.network/))
- Assess compatibility with Layer2Manager and protocol traits
- Prototype `ArchClient`/`ArchModule` as a Layer2 protocol (async traits)
- Explore DeFi, asset management, and cross-layer use cases
- Monitor Arch roadmap, node software, validator requirements
- Engage with Arch developer community for early access and support

**Next Steps:**

- Assign research lead for Arch integration
- Track Arch documentation and node software updates
- Begin technical prototyping in a dedicated branch/module
- Add Arch milestones to Layer2 roadmap
- Share findings in future roadmap updates

---

## Implementation Priorities

1. **Security** – Highest priority for all components
2. **Compliance** – Full Bitcoin protocol and BIP compliance
3. **Performance** – Speed, resource usage, scalability
4. **Usability** – Developer and user accessibility
5. **Interoperability** – Seamless with other Bitcoin implementations

## Layer2 & Async Milestones (Complete)

- All Layer2 modules (BobClient, LiquidModule, RskClient, StacksClient, TaprootAssetsProtocol, LightningNetwork, StateChannel) implemented and locked
- Layer2Manager provides unified orchestration and cross-layer operations
- Async Layer2Protocol trait implemented for all clients
- All async and integration tests passing
- See `ASYNC_LAYER2_IMPLEMENTATION_STATUS.md`, `ASYNC_LAYER2_IMPLEMENTATION_COMPLETE.md`, and `src/layer2/LAYER2_MODULE_LOCK.md` for details

## Testing & Deployment

- **Unit, integration, and performance tests:** 95%+ pass rate across all systems
- **Production deployment:** Core, Layer2, and security systems deployed
- **Staging:** Active for feature, integration, and performance testing
- **CI/CD:** Automated testing, security scanning, and deployment

## Research & Future Directions

- **Arch Network integration** (see above)
- Zero-knowledge proofs, post-quantum cryptography, advanced privacy
- Cross-chain atomic swaps, DeFi protocol integrations
- Decentralized governance, sustainable funding, formal specification

---

## Past Milestones & Legacy Roadmap

*The following sections summarize completed and legacy milestones for historical reference. For full details, see previous roadmap versions and supporting documentation.*

# Past Milestones

- Complete async Layer2Protocol implementation for all protocols
- Layer2Manager async support and cross-layer operations
- Bitcoin Core compilation and protocol integration
- Security analysis, HSM integration, and cryptographic improvements
- Production-ready codebase and robust error handling
- RAGEntic multi-agent system and Web5 integration
- Comprehensive documentation, benchmarks, and migration guides

---

## Module & System Overview

- **Layer2Manager**: Unified orchestration for all Layer2 protocols
- **Layer2 Modules**: BobClient, LiquidModule, RskClient, StacksClient, TaprootAssetsProtocol, LightningNetwork, StateChannel
- **Testing**: 95%+ pass rate across 3,000+ tests (unit, integration, performance)
- **Deployment**: Production/staging environments, Docker/Kubernetes support
- **Documentation**: Up to date, with migration guides and integration tutorials

---

## Update & Monitoring Triggers

- Automated tests and security scans on code commits, PRs, and releases
- Documentation updates for all code and process changes
- Performance, security, and financial health monitored continuously

---

## Status Legend

- **Complete**: Fully implemented and tested
- **In Progress**: Actively being developed or researched
- **Planned**: On the roadmap for future work

---

## References

- `ASYNC_LAYER2_IMPLEMENTATION_STATUS.md`
- `ASYNC_LAYER2_IMPLEMENTATION_COMPLETE.md`
- `src/layer2/LAYER2_MODULE_LOCK.md`
- `docs/bitcoin/LAYER2_SUPPORT.md`
- [docs.arch.network](https://docs.arch.network/)

---

*This roadmap is continuously updated to reflect the true state of the Anya-core system. For the latest status, see the referenced documentation and codebase.*
