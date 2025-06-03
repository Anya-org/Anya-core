# Extension Publishing Guide

[AIR-3][AIS-3][AIT-3][RES-3] **Complete guide for publishing Anya Core extensions to official, community, and enterprise registries.**

*Last updated: May 30, 2025*

## Table of Contents

- [Publishing Overview](#publishing-overview)
- [Registry Types](#registry-types)
- [Pre-Publishing Checklist](#pre-publishing-checklist)
- [Extension Packaging](#extension-packaging)
- [Publishing Process](#publishing-process)
- [Review and Approval](#review-and-approval)
- [Post-Publishing Management](#post-publishing-management)
- [Best Practices](#best-practices)
- [Troubleshooting](#troubleshooting)

## Publishing Overview

Publishing Anya Core extensions involves several steps to ensure quality, security, and compatibility:

### Publishing Lifecycle

1. **Development**: Build and test your extension
2. **Packaging**: Create distributable package
3. **Validation**: Automated and manual validation
4. **Submission**: Submit to appropriate registry
5. **Review**: Security and quality review process
6. **Publication**: Extension becomes available
7. **Maintenance**: Ongoing updates and support

### Extension Types

- **Core Extensions**: Official extensions maintained by Anya Core team
- **Community Extensions**: Open-source community contributions
- **Enterprise Extensions**: Commercial extensions with premium features
- **Private Extensions**: Internal organizational extensions

## Registry Types

### Official Registry

**URL**: `https://extensions.anya.org`

**Characteristics:**

- ✅ Curated and thoroughly tested
- ✅ Security audited by Anya Core team
- ✅ Long-term support guarantee
- ✅ Highest quality standards
- ✅ Production-ready stability

**Requirements:**

- Exceptional code quality and documentation
- Comprehensive test coverage (>95%)
- Security audit compliance
- Maintenance commitment (minimum 2 years)
- Community adoption evidence

**Publishing Process:**

```bash
# Submit proposal first
anya ext propose --registry official --proposal proposal.md

# After approval, submit extension
anya ext publish --registry official --license official
```

### Community Registry

**URL**: `https://community.anya.org/extensions`

**Characteristics:**

- 🌍 Open to all developers
- 🌍 Community-driven quality assurance
- 🌍 Rapid innovation and experimentation
- 🌍 Peer review process
- 🌍 Collaborative development

**Requirements:**

- Open source license (MIT, Apache 2.0, GPL)
- Basic documentation and examples
- Test coverage (minimum 80%)
- Community contribution guidelines
- Issue tracking and support

**Publishing Process:**

```bash
# Standard community publishing
anya ext publish --registry community

# With peer review request
anya ext publish --registry community --request-review
```

### Enterprise Registry

**URL**: `https://enterprise.anya.org/extensions`

**Characteristics:**

- 🏢 Commercial and premium extensions
- 🏢 Professional support and SLA
- 🏢 Enterprise-grade security and compliance
- 🏢 Advanced features and integrations
- 🏢 Dedicated customer success

**Requirements:**

- Valid enterprise license
- Commercial support agreement
- Security and compliance certification
- Professional documentation
- Customer success metrics

**Publishing Process:**

```bash
# Requires enterprise license
anya ext publish --registry enterprise --license enterprise.license

# With support tier specification
anya ext publish --registry enterprise --support-tier premium
```

### Private Registry

**Self-hosted or organizational registries**

**Characteristics:**

- 🔒 Internal organizational use
- 🔒 Custom security requirements
- 🔒 Proprietary features and data
- 🔒 Full control over distribution
- 🔒 Custom approval workflows

**Setup:**

```bash
# Configure private registry
anya registry add private https://registry.company.com --auth-token $TOKEN

# Publish to private registry
anya ext publish --registry private
```

## Pre-Publishing Checklist

### Code Quality

- [ ] **Code Coverage**: Minimum 80% test coverage (95% for official registry)
- [ ] **Documentation**: Complete API documentation and usage examples
- [ ] **Linting**: Passes all linting rules (`cargo clippy`)
- [ ] **Formatting**: Consistent code formatting (`cargo fmt`)
- [ ] **Dependencies**: Up-to-date and secure dependencies
- [ ] **Security**: No known vulnerabilities (`cargo audit`)

### Extension Metadata

- [ ] **Extension.toml**: Complete and accurate metadata
- [ ] **Version**: Semantic versioning compliance
- [ ] **License**: Clear and appropriate license
- [ ] **Authors**: Contact information provided
- [ ] **Description**: Clear and informative description
- [ ] **Keywords**: Relevant search keywords

### Compatibility

- [ ] **Anya Version**: Compatible with current Anya Core version
- [ ] **Platform Support**: Tested on supported platforms
- [ ] **Dependencies**: Compatible dependency versions
- [ ] **Features**: Optional features clearly documented
- [ ] **Configuration**: Schema validation and defaults

### Testing

- [ ] **Unit Tests**: Comprehensive unit test suite
- [ ] **Integration Tests**: End-to-end integration testing
- [ ] **Performance Tests**: Performance benchmarks included
- [ ] **Platform Tests**: Multi-platform testing
- [ ] **Compatibility Tests**: Version compatibility testing

### Documentation

- [ ] **README**: Comprehensive README with examples
- [ ] **API Docs**: Complete API documentation
- [ ] **Changelog**: Version history and changes
- [ ] **Contributing**: Contribution guidelines
- [ ] **License**: License file included
- [ ] **Examples**: Working usage examples

### Security

- [ ] **Audit**: Security audit completed
- [ ] **Vulnerabilities**: No known security issues
- [ ] **Permissions**: Minimal required permissions
- [ ] **Secrets**: No hardcoded secrets or credentials
- [ ] **Cryptography**: Proper cryptographic practices

## Extension Packaging

### Package Structure

```
my-extension-1.0.0/
├── extension.toml          # Extension metadata
├── Cargo.toml             # Rust dependencies
├── README.md              # Main documentation
├── LICENSE                # License file
├── CHANGELOG.md           # Version history
├── src/                   # Source code
├── tests/                 # Test suite
├── docs/                  # Documentation
├── examples/              # Usage examples
├── benches/              # Benchmarks
└── assets/               # Static assets
```

### Extension Metadata (extension.toml)

```toml
[extension]
name = "my-awesome-extension"
version = "1.0.0"
description = "An awesome extension for Anya Core"
authors = ["John Doe <john@example.com>"]
license = "MIT"
repository = "https://github.com/user/my-awesome-extension"
homepage = "https://my-awesome-extension.com"
documentation = "https://docs.my-awesome-extension.com"
keywords = ["bitcoin", "web5", "ml", "awesome"]
categories = ["cryptocurrency", "web-technologies", "science"]

[extension.compatibility]
min_anya_version = "2.5.0"
max_anya_version = "3.0.0"
platforms = ["linux", "macos", "windows"]
architectures = ["x86_64", "aarch64"]

[extension.dependencies]
anya-core = "2.5.0"
bitcoin = { version = "0.30", optional = true }
web5 = { version = "0.8", optional = true }
tokio = "1.0"
serde = { version = "1.0", features = ["derive"] }

[extension.features]
default = ["bitcoin"]
bitcoin = ["dep:bitcoin"]
web5 = ["dep:web5"]
ml = ["dep:candle-core"]
enterprise = ["bitcoin", "web5", "ml"]

[extension.configuration]
schema = "config-schema.json"
default_config = "default-config.toml"
required_config = ["api_key"]

[extension.permissions]
required = ["network.http", "storage.read", "storage.write"]
optional = ["hardware.gpu", "network.bitcoin"]

[extension.resources]
min_memory = "512MB"
min_disk = "100MB"
cpu_intensive = false
gpu_accelerated = true

[extension.support]
documentation = "https://docs.my-awesome-extension.com"
issues = "https://github.com/user/my-awesome-extension/issues"
discussions = "https://github.com/user/my-awesome-extension/discussions"
security = "security@my-awesome-extension.com"

[extension.publishing]
registry = "community"
category = "tools"
maturity = "stable"  # experimental, beta, stable, mature
maintenance = "active"  # active, maintenance, deprecated
```

### Configuration Schema

```json
{
  "$schema": "http://json-schema.org/draft-07/schema#",
  "title": "My Awesome Extension Configuration",
  "type": "object",
  "properties": {
    "api_key": {
      "type": "string",
      "description": "API key for external service",
      "minLength": 32
    },
    "endpoint": {
      "type": "string",
      "format": "uri",
      "default": "https://api.example.com",
      "description": "API endpoint URL"
    },
    "timeout": {
      "type": "integer",
      "minimum": 1,
      "maximum": 300,
      "default": 30,
      "description": "Request timeout in seconds"
    },
    "features": {
      "type": "object",
      "properties": {
        "bitcoin_integration": {
          "type": "boolean",
          "default": true
        },
        "web5_integration": {
          "type": "boolean",
          "default": false
        }
      }
    }
  },
  "required": ["api_key"],
  "additionalProperties": false
}
```

### Package Creation

```bash
# Create package
anya ext package

# Package with specific features
anya ext package --features bitcoin,web5

# Package for specific platform
anya ext package --target x86_64-unknown-linux-gnu

# Package with assets
anya ext package --include-assets

# Validate package
anya ext validate my-extension-1.0.0.tar.gz
```

### Package Validation

```bash
# Comprehensive validation
anya ext validate my-extension-1.0.0.tar.gz --comprehensive

# Security scan
anya ext validate my-extension-1.0.0.tar.gz --security-scan

# Compatibility check
anya ext validate my-extension-1.0.0.tar.gz --compatibility-check

# Performance analysis
anya ext validate my-extension-1.0.0.tar.gz --performance-analysis
```

## Publishing Process

### Step 1: Pre-Publication Testing

```bash
# Local testing
cargo test --all-features
cargo bench
cargo clippy -- -D warnings
cargo audit

# Integration testing
anya test extension ./my-extension

# Performance testing
anya benchmark extension ./my-extension

# Security testing
anya security-scan ./my-extension
```

### Step 2: Package Creation

```bash
# Create final package
anya ext package --release

# Sign package (for official/enterprise)
anya ext sign my-extension-1.0.0.tar.gz --key signing.key

# Verify signature
anya ext verify my-extension-1.0.0.tar.gz.sig
```

### Step 3: Registry Submission

```bash
# Community registry submission
anya ext publish --registry community

# With additional metadata
anya ext publish \
  --registry community \
  --category tools \
  --maturity stable \
  --description "Enhanced description for registry"

# Enterprise registry submission
anya ext publish \
  --registry enterprise \
  --license enterprise.license \
  --support-tier premium \
  --sla support-sla.pdf
```

### Step 4: Publication Verification

```bash
# Check publication status
anya ext status my-extension --registry community

# Verify published extension
anya ext info my-extension --registry community

# Test installation from registry
anya ext install my-extension --registry community --dry-run
```

## Review and Approval

### Community Review Process

1. **Automated Checks**: Security, compatibility, and quality metrics
2. **Peer Review**: Community member code review
3. **Testing**: Automated testing on multiple platforms
4. **Documentation Review**: Documentation quality assessment
5. **Final Approval**: Community maintainer approval

### Official Registry Review

1. **Initial Screening**: Automated quality and security checks
2. **Security Audit**: Professional security assessment
3. **Code Review**: Anya Core team code review
4. **Documentation Review**: Technical writing team review
5. **Testing**: Comprehensive testing across environments
6. **Legal Review**: License and legal compliance check
7. **Final Approval**: Core team approval

### Enterprise Registry Review

1. **Commercial Review**: Business model and pricing assessment
2. **Security Certification**: Enterprise security requirements
3. **Compliance Check**: Regulatory compliance verification
4. **Support Assessment**: Support capabilities evaluation
5. **SLA Review**: Service level agreement validation
6. **Customer References**: Customer adoption verification
7. **Final Approval**: Enterprise team approval

### Review Criteria

#### Code Quality (Weight: 25%)

- Code structure and organization
- Error handling and edge cases
- Performance optimization
- Memory safety and security

#### Documentation (Weight: 20%)

- Completeness and accuracy
- Code examples and tutorials
- API reference quality
- User experience

#### Testing (Weight: 20%)

- Test coverage and quality
- Integration test completeness
- Performance benchmarks
- Platform compatibility

#### Security (Weight: 20%)

- Vulnerability assessment
- Cryptographic practices
- Permission model
- Data handling

#### Community Value (Weight: 15%)

- Innovation and uniqueness
- Community demand
- Ecosystem integration
- Long-term viability

## Post-Publishing Management

### Version Management

```bash
# Publish new version
anya ext publish --version 1.1.0

# Deprecate old version
anya ext deprecate my-extension@1.0.0 --reason "Security vulnerability"

# Yanking broken version
anya ext yank my-extension@1.0.5 --reason "Critical bug"

# Update metadata
anya ext update-metadata my-extension --description "Updated description"
```

### Analytics and Metrics

```bash
# Download statistics
anya ext stats my-extension --downloads

# Usage analytics
anya ext stats my-extension --usage

# User feedback
anya ext feedback my-extension

# Performance metrics
anya ext metrics my-extension
```

### Support and Maintenance

```bash
# Monitor issues
anya ext issues my-extension

# Security alerts
anya ext security-alerts my-extension

# Update dependencies
anya ext update-deps my-extension

# Compatibility monitoring
anya ext compatibility my-extension
```

## Best Practices

### Development Best Practices

1. **Follow Rust Best Practices**: Use idiomatic Rust code
2. **Comprehensive Testing**: High test coverage and quality
3. **Clear Documentation**: Write for your future self
4. **Semantic Versioning**: Follow SemVer strictly
5. **Security First**: Security by design principles

### Publishing Best Practices

1. **Start with Community**: Build reputation in community registry
2. **Gradual Feature Addition**: Evolve features based on feedback
3. **Responsive Maintenance**: Address issues promptly
4. **Community Engagement**: Participate in discussions
5. **Quality Over Quantity**: Focus on doing one thing well

### Marketing and Adoption

1. **Clear Value Proposition**: Explain benefits clearly
2. **Good Documentation**: Excellent docs drive adoption
3. **Active Community**: Engage with users and contributors
4. **Regular Updates**: Show active maintenance
5. **Use Cases**: Provide real-world examples

### Support and Maintenance

1. **Issue Tracking**: Use GitHub issues effectively
2. **Release Notes**: Detailed changelog for each release
3. **Security Updates**: Prompt security patch releases
4. **Community Guidelines**: Clear contribution guidelines
5. **Backward Compatibility**: Maintain compatibility when possible

## Troubleshooting

### Common Publishing Issues

#### Package Validation Failures

```bash
# Check package contents
tar -tzf my-extension-1.0.0.tar.gz

# Validate metadata
anya ext validate-metadata extension.toml

# Check dependencies
cargo tree
```

#### Registry Authentication Issues

```bash
# Check authentication
anya auth status --registry community

# Re-authenticate
anya auth login --registry community

# Update token
anya auth token --registry community --update
```

#### Publication Failures

```bash
# Check network connectivity
anya registry ping community

# Verify package integrity
anya ext verify my-extension-1.0.0.tar.gz

# Check registry status
anya registry status community
```

#### Review Process Issues

```bash
# Check review status
anya ext review-status my-extension

# Request review expedite
anya ext request-review my-extension --expedite

# Contact reviewers
anya ext contact-reviewers my-extension
```

### Error Resolution

#### "Extension Already Exists"

```bash
# Check existing versions
anya ext versions my-extension --registry community

# Use different name or increment version
anya ext rename my-extension my-extension-v2
```

#### "Incompatible Anya Version"

```bash
# Update compatibility in extension.toml
[extension.compatibility]
min_anya_version = "2.5.0"

# Test with multiple Anya versions
anya test-compatibility my-extension
```

#### "Security Scan Failed"

```bash
# Run local security scan
anya security-scan ./my-extension --detailed

# Fix identified issues
cargo audit fix

# Update dependencies
cargo update
```

#### "Insufficient Documentation"

```bash
# Generate documentation template
anya ext doc-template my-extension

# Check documentation coverage
anya ext doc-coverage my-extension

# Validate examples
anya ext validate-examples my-extension
```

### Performance Optimization

#### Package Size Optimization

```bash
# Minimize package size
cargo clean
anya ext package --optimize-size

# Exclude unnecessary files
echo "target/" >> .packageignore
echo "*.tmp" >> .packageignore
```

#### Build Time Optimization

```bash
# Parallel builds
cargo build --release -j8

# Cache dependencies
anya ext cache-deps my-extension

# Incremental builds
cargo build --release --incremental
```

## Related Documentation

- **[Review Process](./review-process.md)**: Detailed review process documentation
- **[Publishing Guidelines](./guidelines.md)**: Extension publishing guidelines
- **[Distribution](./distribution.md)**: Extension distribution mechanisms
- **[Extension Development](../development/README.md)**: Extension development guide
- **[Security Guidelines](../integration/security-guidelines.md)**: Security best practices

For additional support with publishing, visit the [Anya Core Documentation](https://docs.anya.org) or join the [Community Discord](https://discord.gg/anya).
