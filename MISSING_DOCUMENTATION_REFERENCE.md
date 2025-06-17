# Missing Documentation Reference Guide

*Date: June 17, 2025*

This document tracks documentation files that are referenced in the codebase but do not exist. These files need to be created as part of the documentation cleanup campaign.

## Priority 1: Critical Files (Complete by June 19, 2025)

These files are referenced in multiple places and are essential for user understanding:

1. **docs/installation/troubleshooting.md**
   - Referenced in: `docs/installation/README.md`
   - Content: Specific installation troubleshooting guidance
   - Status: ✅ Complete
   - Assignee: Documentation Team

2. **docs/installation/related1.md** and **docs/installation/related2.md**
   - Referenced in: `docs/installation/README.md`
   - Content: Should reference existing `INSTALLATION.md` and `INSTALLATION_REVIEW.md`
   - Status: ✅ Complete
   - Assignee: Documentation Team

3. **docs/DOCUMENTATION_QA_COMPLETE.md**
   - Referenced in: Multiple files
   - Content: Documentation QA process and status
   - Status: ✅ Complete
   - Assignee: Documentation Team

## Priority 2: Important Files (Complete by June 25, 2025)

These files enhance documentation but are not critical blockers:

1. **docs/standards/MARKDOWN_STYLE_GUIDE.md**
   - Referenced in: Documentation processes
   - Content: Markdown style and formatting guidelines
   - Status: 📝 Planned
   - Assignee: TBD

2. **docs/API_REFERENCE.md**
   - Referenced in: Various API documentation files
   - Content: Complete API reference
   - Status: 📝 Planned
   - Assignee: TBD

3. **docs/HEXAGONAL.md**
   - Referenced in: Architecture documentation
   - Content: Hexagonal architecture explanation
   - Status: 📝 Planned
   - Assignee: TBD

## Priority 3: Nice-to-Have Files (Complete by June 30, 2025)

These files would improve documentation but can be deferred:

1. **docs/web5/TAPROOT_INTEGRATION.md**
   - Referenced in: `DAO_MULTILAYER_UPGRADE.md`
   - Content: Integration details for Taproot with Web5
   - Status: 📝 Planned
   - Assignee: TBD

2. **docs/bitcoin/taproot.md**
   - Referenced in: Multiple locations
   - Content: Taproot implementation details
   - Status: 📝 Planned
   - Assignee: TBD

## Consolidated Files

These are redundant files that should be consolidated:

1. **architecture.md** → **ARCHITECTURE.md**
   - Multiple versions of the same content
   - Action: Remove duplicate and update references

2. **contributing.md** → **CONTRIBUTING.md**
   - Multiple versions of the same content
   - Action: Remove duplicate and update references

## Tracking Process

1. When starting work on a file, update its status to "⏳ In Progress"
2. When complete, update to "✅ Complete" with completion date
3. Each file should be reviewed by at least one other team member
4. Run link validation after each file creation to ensure no broken links

## Template for New Files

```markdown
---
title: "Document Title"
description: "Brief description"
last_updated: 2025-06-17
---

# Document Title

## Overview

Brief overview of the document's purpose.

[AIR-3][AIS-3][BPC-3][RES-3]

## Content Section 1

Content here...

## Content Section 2

Content here...

## See Also

- [Documentation Cleanup Plan](DOCUMENTATION_CLEANUP_PLAN.md)
- [Link Campaign Summary](DOCUMENTATION_LINK_CAMPAIGN_SUMMARY.md)
```
