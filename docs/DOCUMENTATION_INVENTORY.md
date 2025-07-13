# Documentation Inventory and Alignment Status

## Overview

This document tracks the alignment status between documentation and code implementation across the Anya Core repository. It identifies issues, prioritizes fixes, and establishes a consistent documentation structure.

## Inventory Structure

Each documentation file is evaluated using the following criteria:

- **Alignment Status**: How well the documentation matches the actual code implementation
  - ✅ **Aligned**: Documentation accurately reflects code implementation
  - ⚠️ **Partial**: Documentation is partially aligned with code but has inconsistencies
  - ❌ **Misaligned**: Documentation does not match current code implementation
  - 🔍 **Unknown**: Requires further investigation
  
- **Priority**: Importance of fixing this documentation
  - **High**: Critical for developer onboarding or core functionality understanding
  - **Medium**: Important but not blocking development
  - **Low**: Nice to have but not essential

- **Issues**: Specific issues identified
- **Action Required**: Recommended fixes

## Core Documentation

| File | Alignment | Priority | Issues | Action Required |
|------|-----------|----------|--------|----------------|
| `/home/botshelo/anya-core/README.md` | ✅ | High | Minor updates needed to reflect current module statuses | Update module status indicators |
| `/home/botshelo/anya-core/docs/index.md` | ⚠️ | High | Some links may point to outdated or incomplete documentation | Verify all links and update missing references |
| `/home/botshelo/anya-core/docs/ARCHITECTURE.md` | ⚠️ | High | Contains placeholder TOC sections, diagram references that may not exist | Complete architecture documentation with current implementation details |
| `/home/botshelo/anya-core/docs/API.md` | ❌ | High | Marked as deprecated, references files that may not exist | Replace with updated API documentation or redirect to current API docs |

## Module Documentation

| File | Alignment | Priority | Issues | Action Required |
|------|-----------|----------|--------|----------------|
| `/home/botshelo/anya-core/docs/bitcoin/README.md` | ❌ | High | Contains placeholder text and incomplete sections | Complete missing sections, update to reflect code structure |
| `/home/botshelo/anya-core/src/bitcoin/README.md` | ⚠️ | High | Better coverage than docs version, but may have outdated sections | Update to ensure full alignment with current code |
| `/home/botshelo/anya-core/docs/api/README.md` | ⚠️ | High | References multiple API types not fully implemented in code | Update to match actual API implementation, remove references to non-existent APIs |
| `/home/botshelo/anya-core/docs/api/API_REFERENCE.md` | 🔍 | Medium | May contain outdated API references | Verify against current API implementation |
| `/home/botshelo/anya-core/docs/api/api-reference.md` | ❌ | High | Duplicated file with different content than API_REFERENCE.md | Consolidate and standardize API reference documentation |

## Layer2 Documentation

| File | Alignment | Priority | Issues | Action Required |
|------|-----------|----------|--------|----------------|
| `/home/botshelo/anya-core/docs/layer2/README.md` | ⚠️ | High | Interface documentation doesn't match implementation (method names differ) | Update trait documentation to match actual code |
| `/home/botshelo/anya-core/docs/layer2/lightning.md` | ❌ | High | References `LightningProtocol` but code has `LightningNetwork`, config fields don't match | Update class names and config fields to match implementation |
| `/home/botshelo/anya-core/docs/layer2/rgb.md` | ⚠️ | Medium | Example code doesn't match actual implementation details | Update examples to match current RGB implementation |

## API Documentation

| File | Alignment | Priority | Issues | Action Required |
|------|-----------|----------|--------|----------------|
| `/home/botshelo/anya-core/docs/api/index.md` | ⚠️ | Medium | May contain outdated links or references | Update to reflect current API structure |
| `/home/botshelo/anya-core/docs/api/integration_guide.md` | 🔍 | Medium | Needs verification against current integration patterns | Review and update based on current API implementation |
| `/home/botshelo/anya-core/docs/api/GOVERNANCE_API.md` | 🔍 | Medium | Needs verification against governance code | Compare with actual governance implementation |
| `/home/botshelo/anya-core/docs/api/PSBT_V2_EXAMPLES.md` | ⚠️ | Medium | Contains minimal examples that may not match implementation | Expand examples and verify against current code |

## Documentation Format Issues

| Issue Type | Affected Files | Description | Action Required |
|------------|----------------|-------------|----------------|
| File Naming Inconsistency | Multiple files | Inconsistent naming conventions (uppercase, lowercase, kebab-case) | Standardize file naming conventions |
| AI Labeling | Most files | Inconsistent use of AI labeling standards | Standardize AI labeling according to documentation |
| Documentation Location | Multiple files | Documentation spread between `/docs` and source code directories | Establish consistent documentation location pattern |
| Placeholder Content | Multiple files | Many files contain placeholder content or TODOs | Replace placeholders with actual content |

## Redundancy and Consolidation Opportunities

| Documentation Group | Files | Issues | Recommendation |
|--------------------|-------|--------|----------------|
| Bitcoin Documentation | Multiple files in root and /docs | Duplicate README files in src/bitcoin and docs/bitcoin | Consolidate into a single source of truth |
| API Documentation | Multiple files in docs/api | Duplicate files with similar names but different content | Standardize API documentation and remove duplicates |
| AI Labeling | API.md and standards/AI_LABELING.md | Duplicated information about AI labeling | Use single canonical source as noted in deprecation notice |

## High-Priority Documentation Fixes

1. **Layer2 Protocol Interface Documentation**: Update to match actual implementation
   - Fix method names and signatures
   - Complete missing method documentation
   - Ensure examples match current code

2. **Bitcoin Module Documentation**: Complete missing sections
   - Replace placeholder content
   - Ensure alignment with current implementation
   - Consolidate duplicate documentation

3. **API Documentation**: Resolve duplication and standardize
   - Remove duplicate files with different content
   - Update examples to match actual implementation
   - Standardize API reference format

## Next Steps

1. Address high-priority documentation issues first
2. Establish documentation standards and patterns
3. Update interface documentation to match implementation
4. Consolidate redundant documentation
5. Implement documentation review process for ongoing maintenance
