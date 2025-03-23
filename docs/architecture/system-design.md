<!-- markdownlint-disable MD013 line-length -->

# System Design

Documentation for System Design

*Last updated: 2024-12-07*

## Installation Architecture

![Installation Hexagonal Architecture](https://mermaid.ink/svg/eyJjb2RlIjoiZ3JhcGggVERcbiAgICBBW0JpdGNvaW4gQ29yZV0gLS0-IEJbQml0Y29pbiBBZGFwdGVyXVxuICAgIEIgLS0-IENTaGVsbCBJbnN0YWxsZXJdXG4gICAgQyAtLT4gRFtBdXRvLUNvbmZpZ11cbiAgICBEIC0tPiBFW0JpcC0zNDEgVmFsaWRhdG9yXVxuICAgIEMgLS0-IEZbQmlwLTE3NCBWYWxpZGF0b3JdXG4gICAgQyAtLT4gR1tTZWN1cml0eSBDaGVja3NdXG4gICAgQyAtLT4gSFtSdXN0IENvcmVdXG4gICAgQyAtLT4gSVtXZWI1IEludGVncmF0aW9uXSIsIm1lcm1haWQiOnsidGhlbWUiOiJkZWZhdWx0In0sInVwZGF0ZUVkaXRvciI6ZmFsc2V9)

### Compliance Status
| Component              | BIP-341 | BIP-174 | AIS-3 | RES-3 | Audit |
|------------------------|---------|---------|-------|-------|-------|
| Shell Installer        | Partial | Full    | Yes   | Yes   | 2025  |
| Rust Installer         | Full    | Full    | Yes   | Yes   | 2025  |
| Python Installer       | No      | Partial | No    | Yes   | -     |
| PowerShell Installer   | No      | No      | No    | Yes   | -     |
