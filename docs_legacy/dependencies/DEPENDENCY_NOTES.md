# Dependency Management Notes

## Bitcoin Crate Versioning

As of July 2, 2025, the Anya-core project uses two versions of the Bitcoin crate:

1. **Bitcoin v0.32.6** (primary version)
   - Used for most of the codebase
   - Includes modern Taproot API (TapLeaf, TaprootBuilder, etc.)
   - Applied to all new code and main project components

2. **Bitcoin v0.30.2** (secondary version)
   - Used only by BDK v0.30.2 and its dependencies
   - Cannot be upgraded until a newer version of BDK is available that supports Bitcoin v0.32.x

### Why Two Versions?

The project standardized on Bitcoin v0.32.6 for most functionality, especially the Taproot implementations and Bitcoin protocol handling. However, BDK 0.30.x (Bitcoin Development Kit) has a hard dependency on Bitcoin 0.30.x, which cannot be overridden.

Cargo's dependency resolution allows both versions to co-exist in the dependency tree, with each crate using the version it was compiled against.

### Long-term Plan

The development roadmap includes:

1. Continue using Bitcoin v0.32.6 as the standard version for all new code
2. Monitor for updates to BDK that support newer Bitcoin versions
3. When available, upgrade BDK and remove the secondary Bitcoin dependency
4. Consider alternatives to BDK if updates are not forthcoming

## Important Compatibility Notes

- Do not attempt to pass Bitcoin objects between code using different versions
- Use appropriate adapter patterns when necessary to bridge between the versions
- Always specify exact Bitcoin version when adding new dependencies

## Standard Version

**The standard version for Anya-core is Bitcoin v0.32.6**
