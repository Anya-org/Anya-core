# Anya Core Documentation Refactoring Plan

**Status**: 🔄 In Progress
**Date**: August 6, 2025
**Scope**: Complete documentation alignment with source code truth

## 📊 Current State Analysis

### Documentation Statistics

- **Markdown Files**: 429 docs across `/docs` directory
- **Generated HTML**: 194 files in `/site` directory
- **Rust Source Files**: ~1,132 .rs files with public APIs to document
- **Main Library Modules**: 20+ top-level modules in `src/lib.rs`

### Key Issues Identified

#### 1. **Documentation Fragmentation**

- 429 markdown files scattered across multiple directories
- Inconsistent structure between `/docs` and `/site`
- Multiple index files and navigation systems
- Redundant content in different locations

#### 2. **Source Code Misalignment**

- Documentation references non-existent modules
- API documentation doesn't match current implementation
- Broken internal links throughout documentation
- Outdated architectural diagrams

#### 3. **Structural Problems**

- TODOs and placeholders in critical documentation
- Mixed concerns (PRD, technical docs, tutorials)
- Missing documentation for implemented features
- Archive content mixed with current documentation

## 🎯 Refactoring Objectives

### Primary Goals

1. **Single Source of Truth**: Align all documentation with actual codebase
2. **Code-First Documentation**: Generate API docs from source code
3. **Clear Information Architecture**: Logical, navigable structure
4. **Maintainable System**: Automated documentation updates

### Success Criteria

- [ ] All internal links functional
- [ ] API documentation matches current implementation
- [ ] Clear separation of user/developer/operator documentation
- [ ] Automated documentation generation pipeline
- [ ] < 100 total documentation files (75% reduction)

## 🏗️ Implementation Strategy

### Phase 1: Code Analysis & Truth Mapping

**Duration**: 1 day
**Owner**: Technical Lead

#### Tasks

1. **Module Inventory**: Map all public APIs in source code

   ```bash
   # Generate comprehensive API inventory
   find src/ -name "*.rs" -exec grep -l "pub fn\|pub struct\|pub enum\|pub mod\|pub trait" {} \; | \
   xargs cargo doc --no-deps --document-private-items
   ```

2. **Feature Analysis**: Document actual feature implementation
   - Bitcoin module capabilities
   - Layer2 protocol implementations
   - ML/AI agent functionality
   - Web5 integration points
   - DAO governance features

3. **API Surface Documentation**: Generate from source

   ```bash
   cargo doc --all-features --open
   ```

### Phase 2: Information Architecture Redesign

**Duration**: 1 day
**Owner**: Documentation Team

#### New Documentation Structure

```
docs/
├── README.md                     # Single entry point
├── getting-started/
│   ├── installation.md           # Consolidated installation guide
│   ├── quickstart.md            # Basic usage examples
│   └── concepts.md              # Core concepts explained
├── api/
│   ├── README.md                # API overview
│   ├── rest-api.md              # REST endpoints
│   ├── rust-api.md              # Rust library API (auto-generated)
│   └── websocket-api.md         # WebSocket interface
├── architecture/
│   ├── README.md                # System overview
│   ├── core-components.md       # Core module architecture
│   ├── security-model.md        # Security design
│   └── data-flow.md             # Data flow diagrams
├── guides/
│   ├── bitcoin-integration.md   # Bitcoin usage guide
│   ├── layer2-protocols.md      # Layer2 implementation guide
│   ├── ml-agents.md             # ML/AI agent usage
│   ├── web5-integration.md      # Web5 protocol usage
│   └── dao-governance.md        # DAO functionality guide
├── operations/
│   ├── deployment.md            # Production deployment
│   ├── monitoring.md            # Observability setup
│   ├── security.md              # Security operations
│   └── troubleshooting.md       # Common issues & solutions
├── development/
│   ├── contributing.md          # How to contribute
│   ├── testing.md               # Testing strategies
│   ├── building.md              # Build system guide
│   └── extending.md             # Extension development
└── reference/
    ├── configuration.md         # Configuration options
    ├── cli-commands.md          # CLI reference
    └── changelog.md             # Version history
```

### Phase 3: Content Migration & Cleanup

**Duration**: 2 days
**Owner**: Engineering Team

#### Migration Process

1. **Content Audit**: Review existing 429 files
   - Categorize by relevance and accuracy
   - Identify content for migration vs. archival
   - Mark duplicate content for consolidation

2. **Truth Reconciliation**: Align with source code
   - Update API documentation to match current implementation
   - Verify all code examples compile and run
   - Update architectural diagrams with current system design

3. **Link Remediation**: Fix broken references
   - Scan all markdown files for broken links
   - Update internal references to new structure
   - Verify external links are still valid

### Phase 4: Automation & Generation

**Duration**: 1 day
**Owner**: DevOps/Infrastructure Team

#### Automated Documentation Pipeline

1. **API Documentation Generation**:

   ```toml
   # Cargo.toml additions
   [package.metadata.docs.rs]
   all-features = true
   rustdoc-args = ["--cfg", "docsrs"]
   ```

2. **CI/CD Integration**:

   ```yaml
   # .github/workflows/docs.yml
   name: Documentation
   on:
     push:
       branches: [main]
     pull_request:
       branches: [main]

   jobs:
     docs:
       runs-on: ubuntu-latest
       steps:
         - uses: actions/checkout@v3
         - name: Generate API docs
           run: cargo doc --all-features --no-deps
         - name: Build user docs
           run: mdbook build docs/
         - name: Check links
           run: linkchecker docs/book/
   ```

3. **Link Checking Automation**:

   ```bash
   # Add to CI pipeline
   npm install -g markdown-link-check
   find docs/ -name "*.md" -exec markdown-link-check {} \;
   ```

### Phase 5: Quality Assurance & Testing

**Duration**: 1 day
**Owner**: QA Team

#### Validation Tasks

1. **Documentation Testing**:
   - All code examples compile and execute
   - All installation instructions work on clean systems
   - All API examples produce expected results

2. **User Experience Testing**:
   - New user can complete quickstart guide
   - Developer can build and extend system
   - Operator can deploy and monitor system

3. **Maintenance Verification**:
   - Documentation updates automatically on code changes
   - Broken links detected in CI
   - Performance impact of documentation generation acceptable

## 📋 Detailed Implementation Plan

### Week 1: Foundation & Analysis

#### Day 1: Code Inventory & Current State Analysis

- **Morning**: Complete source code API mapping

  ```bash
  # Generate comprehensive API inventory
  cargo doc --all-features --no-deps --open
  rg "pub (fn|struct|enum|mod|trait)" src/ --type rust > api_inventory.txt
  ```

- **Afternoon**: Document actual implementation vs. claimed features
  - Bitcoin module: Check BIP compliance implementations
  - Layer2: Verify Lightning, RGB, DLC implementations
  - ML: Document actual AI agent capabilities
  - Web5: Map DID/DWN implementation status

#### Day 2: Information Architecture Design

- **Morning**: Design new documentation structure (above)
- **Afternoon**: Create migration mapping from old to new structure

  ```bash
  # Create migration plan
  find docs/ -name "*.md" | while read file; do
    echo "$file -> $(determine_new_location "$file")" >> migration_plan.txt
  done
  ```

### Week 2: Migration & Content Creation

#### Day 3-4: Content Migration

- **Consolidate Installation Docs**: Merge scattered installation guides
  - Current: `docs/INSTALLATION.md`, `docs/getting-started/installation.md`, etc.
  - Target: Single `docs/getting-started/installation.md`

- **API Documentation**: Generate from source + manual curation
  - Auto-generate: `cargo doc --all-features --no-deps`
  - Manual: REST API endpoints from `src/api/routes.rs`
  - Manual: WebSocket API from `src/api/websocket.rs`

- **Architecture Documentation**: Update with current system
  - Source: `src/lib.rs` module structure
  - Source: `src/api/mod.rs` API architecture
  - Source: `src/bitcoin/mod.rs` Bitcoin integration
  - Source: `src/layer2/mod.rs` Layer2 protocols

#### Day 5: Automation Implementation

- **CI/CD Pipeline**: Implement automated documentation

  ```yaml
  # .github/workflows/docs.yml
  name: Documentation Pipeline
  on: [push, pull_request]
  jobs:
    docs:
      steps:
        - name: Generate API docs
          run: cargo doc --all-features
        - name: Build user guides
          run: mdbook build
        - name: Validate links
          run: linkchecker docs/book/
        - name: Deploy to GitHub Pages
          uses: peaceiris/actions-gh-pages@v3
  ```

### Week 3: Quality Assurance & Launch

#### Day 6-7: Testing & Validation

- **Documentation Testing**: All examples work
- **User Testing**: Fresh developer experience
- **Performance Testing**: Documentation build times
- **Link Validation**: All references functional

## 📊 Metrics & Success Tracking

### Quantitative Metrics

- **File Count**: 429 → <100 files (75% reduction)
- **Broken Links**: Current unknown → 0 broken links
- **Build Time**: Establish baseline → <2 minutes
- **Coverage**: APIs documented: Target 95%

### Qualitative Metrics

- **Developer Experience**: Time to first contribution
- **User Onboarding**: Time to running system
- **Maintainability**: Time to update docs when code changes

## 🔧 Tools & Technologies

### Documentation Generation

- **API Docs**: `rustdoc` with `cargo doc`
- **User Guides**: `mdBook` or `Sphinx`
- **Diagrams**: `mermaid` for architecture diagrams
- **Testing**: `rust-analyzer` for code example validation

### Quality Assurance

- **Link Checking**: `markdown-link-check` or `linkchecker`
- **Spell Check**: `aspell` or `grammarly-cli`
- **Style Guide**: `markdownlint` for consistency
- **CI/CD**: GitHub Actions for automation

### Collaboration

- **Review Process**: GitHub PR reviews for all changes
- **Issue Tracking**: GitHub issues for documentation bugs
- **Discussions**: GitHub discussions for larger changes

## 🎯 Next Steps

### Immediate Actions (Today)

1. **Start Code Inventory**: Begin mapping actual API surface
2. **Create Working Branch**: `docs/refactor-alignment`
3. **Setup Tooling**: Install documentation generation tools
4. **Begin Migration**: Start with highest-priority documents

### This Week

1. **Complete Phase 1**: Code analysis and truth mapping
2. **Begin Phase 2**: Information architecture implementation
3. **Setup Automation**: Basic CI/CD pipeline for docs

### Success Criteria

- [ ] Documentation matches implemented functionality
- [ ] Clear navigation and information architecture
- [ ] Automated updates when code changes
- [ ] <2 minute documentation build time
- [ ] Zero broken internal links
- [ ] New developer can contribute within 1 day

---

This refactoring plan transforms the current fragmented documentation landscape into a maintainable, accurate, and user-friendly system aligned with the actual source code implementation.
