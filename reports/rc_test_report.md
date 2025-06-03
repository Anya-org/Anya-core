# Anya Core RC Test Report
Generated: Thu May 15 12:21:04 PM UTC 2025

## Version
0.2.0-rc1

## Branch Status
```
* develop
  main
  remotes/origin/HEAD -> origin/main
  remotes/origin/develop
  remotes/origin/main
```

## Dependencies
```
anya-core v0.2.0 (/home/anya/anyachainlabs/projects/anya-core)
anyhow v1.0.98
argon2 v0.5.3
base64ct v1.7.3
blake2 v0.10.6
digest v0.10.7
block-buffer v0.10.4
generic-array v0.14.7
typenum v1.18.0
zeroize v1.8.1
zeroize_derive v1.4.2 (proc-macro)
proc-macro2 v1.0.95
unicode-ident v1.0.18
quote v1.0.40
proc-macro2 v1.0.95 (*)
syn v2.0.101
proc-macro2 v1.0.95 (*)
quote v1.0.40 (*)
unicode-ident v1.0.18
version_check v0.9.5
... (truncated)
```

## Known Issues
- HSM module requires additional dependencies
- Core module has dependency resolution issues
- Some Result type signatures need to be fixed

