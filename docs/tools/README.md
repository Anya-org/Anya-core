---
title: "Tools Module"
description: "Utility tools for the Anya Core system"
status: "active"
last_updated: "2025-08-06"
---

# Tools Module [AIR-3][AIS-3][BPC-3][AIT-3]

This module provides various utility tools for the Anya Core system,
following official Bitcoin Improvement Proposals (BIPs) and canonical
Source of Truth Registry standards.

## Table of Contents

- [Overview](#overview)
- [Components](#components)
- [API](#api)
- [Testing](#testing)
- [See Also](#see-also)

## Overview

## Components

The Tools module contains utilities for maintaining code quality, documentation alignment,
and development workflows. These tools are designed to be used both programmatically
within the Anya Core system and via command-line interfaces.

### Documentation Tools

- **doc_duplication_scanner**: Detects duplicate content across documentation files
- **markdown**: Validates markdown files for consistency and proper structure
- **source_of_truth_registry**: Maintains canonical sources of truth for documentation

### Development Tools

- **commit_tracker**: Tracks commits and updates AI labeling information

## Components

- **commit_tracker.rs** - Tracks and validates commit information for AI labeling
- **doc_duplication_scanner.rs** - Detects and manages documentation duplication
- **doc_duplication_scanner_cli.rs** - Command-line interface for duplication scanning
- **markdown.rs** - Utilities for validating documentation structure and compliance
- **mod.rs** - Module definitions and exports
- **source_of_truth_registry.rs** - Canonical registry for documentation and code alignment

## API

API documentation is available:

```bash
cargo doc --open
```

## Testing

```bash
cargo test tools::
```

## See Also

- [Main Documentation](../README.md)
- [API Reference](../api/README.md)
- [Source Code](../../src/tools/)

*Last updated: 2025-08-06*
