## Cross-Repository Standards

1. **Dependency Policy**
   - Major versions must match across all repos
   - Security-critical dependencies pinned with SHA-256 hashes

2. **Security Requirements**

   ```toml
   [workspace.security]
   constant_time = true
   memory_isolation = "strict"
   rng_implementation = "hardware"
   ```

3. **Compliance Enforcement**

   ```bash
   # Validate all repos
   anya-audit repos --all --level strict
   ``` 
   