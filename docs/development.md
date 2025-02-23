# Anya Development Guide

## Project Structure

```
anya/
├── src/
│   ├── core/           # Core Bitcoin implementation
│   ├── mobile/         # Mobile interface
│   └── bitcoin/        # Bitcoin protocol implementation
├── packages/
│   └── dash33/         # Dashboard application
├── docs/               # Documentation
└── scripts/            # Development scripts
```

## Getting Started

1. **Setup Development Environment**
   ```powershell
   ./scripts/dev-setup.ps1
   ```

2. **Build Components**
   ```bash
   # Build core
   cargo build --workspace
   
   # Build mobile
   cd src/mobile && yarn build
   
   # Build dashboard
   cd packages/dash33 && yarn build
   ```

## Development Workflow

### Core Development
- Use `cargo watch` for live reloading
- Follow Bitcoin Core coding standards
- Implement comprehensive tests
- Document all public APIs

### Mobile Development
- Use React Native best practices
- Test on both iOS and Android
- Follow mobile-specific security guidelines
- Maintain offline-first approach

### Dashboard Development
- Follow React best practices
- Implement responsive design
- Use TypeScript for type safety
- Follow accessibility guidelines

## Testing

```bash
# Run all tests
cargo test --workspace

# Run specific tests
cargo test -p anya-core
cargo test -p anya-mobile
```

## Documentation

- API documentation: `cargo doc --no-deps --open`
- Mobile documentation: `cd src/mobile && yarn docs`
- Dashboard documentation: `cd packages/dash33 && yarn docs`

## Deployment

1. **Staging**
   - Automated via GitHub Actions
   - Requires passing tests
   - Includes smoke tests

2. **Production**
   - Manual approval required
   - Full test suite must pass
   - Security audit required

## Contributing

1. Fork the repository
2. Create feature branch
3. Commit changes
4. Run tests
5. Submit pull request

## Security

- Follow security best practices
- Use secure dependencies
- Regular security audits
- Vulnerability reporting process
