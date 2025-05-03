# AI Labeling Tools [AIR-3][AIS-3][BPC-3]

## Overview

This directory contains tools for managing and validating AI labeling across the Anya Core codebase. All tools follow the standardized AI labeling system defined in [docs/standards/AI_LABELING.md](../docs/standards/AI_LABELING.md).

## Available Tools

### 1. `validate_ai_labels.ps1`

This script validates that AI labels in code and documentation follow the standardized format.

```powershell
./scripts/validate_ai_labels.ps1 [-file <path>] [-fix] [-verbose]
```

#### Parameters

- `-file`: (Optional) Specific file to validate
- `-fix`: (Optional) Automatically fix legacy label formats
- `-verbose`: (Optional) Show detailed information about each file

#### Examples

```powershell
# Validate all files in the codebase
./scripts/validate_ai_labels.ps1

# Validate and fix a specific file
./scripts/validate_ai_labels.ps1 -file src/bitcoin/validation.rs -fix

# Validate all files with detailed output
./scripts/validate_ai_labels.ps1 -verbose
```

### 2. `update_ai_labeling.ps1`

This script finds and updates deprecated AI labeling formats across Markdown files in the codebase.

```powershell
./scripts/update_ai_labeling.ps1
```

This will automatically:

1. Scan all Markdown files
2. Replace legacy formats (AIR-001, AIR-012, etc.) with standardized formats ([AIR-3], etc.)
3. Report summary statistics

## Standard AI Label Format

The standardized AI label format is `[XXX-N]` where:

- `XXX` is a 3-letter category code (AIR, AIS, BPC, etc.)
- `N` is a numeric level (0-3, with 3 being highest)

Example: `[AIR-3][AIS-3][BPC-3]`

## Common Label Combinations

The most common label combinations for production code:

```
[AIR-3][AIS-3][BPC-3]              # Standard Bitcoin code
[AIR-3][AIS-3][BPC-3][AIT-3]       # Well-tested Bitcoin code
[AIR-3][AIS-3][BPC-3][RES-3]       # Resilient Bitcoin code
[AIR-3][AIS-3][BPC-3][AIT-3][RES-3] # Comprehensive Bitcoin code
```

## Migration from Legacy Formats

Legacy formats should be updated to the standardized format:

| Legacy Format | New Format |
|---------------|------------|
| AIR-001, AIR-002 | [AIR-1] |
| AIR-003, AIR-004 | [AIR-2] |
| AIR-005+ | [AIR-3] |
| (AIR-012) | [AIR-3] |

## Integration with CI/CD

These scripts can be integrated into CI/CD pipelines to enforce standardized labeling:

```yaml
- name: Validate AI Labels
  run: |
    ./scripts/validate_ai_labels.ps1
```

For more details, see the [AI Labeling System documentation](../docs/standards/AI_LABELING.md). 
