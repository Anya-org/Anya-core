# Cross-platform Makefile for Anya Core

# Detect OS
ifeq ($(OS),Windows_NT)
	DETECTED_OS := Windows
else
	UNAME_S := $(shell uname -s)
	ifeq ($(UNAME_S),Linux)
		DETECTED_OS := Linux
	endif
	ifeq ($(UNAME_S),Darwin)
		DETECTED_OS := macOS
	endif
endif

# Platform-specific commands
ifeq ($(DETECTED_OS),Windows)
	FIX_EDITION := powershell -File ./fix_cargo_edition.ps1
	FIX_REFS := powershell -File ./fix_core_package_refs.ps1
	RM := rmdir /s /q
	MKDIR := mkdir
	RUST_FIX_PATH := target\\release\\anya-fix-scripts.exe
else
	FIX_EDITION := ./fix_cargo_edition.sh
	FIX_REFS := ./fix_core_package_refs.sh
	RM := rm -rf
	MKDIR := mkdir -p
	RUST_FIX_PATH := ./target/release/anya-fix-scripts
endif

.PHONY: all check build test clean fix fix-edition fix-refs rust-fix-tool docs clippy fmt install uninstall pwsh-install

# Main targets
all: fix build test

check:
	cargo check

build:
	cargo build --release

test:
	cargo test --all

clean:
	cargo clean
	$(RM) target 2>/dev/null || true

# Fix targets
fix: fix-edition fix-refs

fix-edition:
	@echo "Fixing edition inheritance issues..."
ifeq ($(DETECTED_OS),Windows)
	$(FIX_EDITION)
else
	chmod +x ./fix_cargo_edition.sh
	$(FIX_EDITION)
endif

fix-refs:
	@echo "Fixing package references..."
ifeq ($(DETECTED_OS),Windows)
	$(FIX_REFS)
else
	chmod +x ./fix_core_package_refs.sh
	$(FIX_REFS)
endif

# Build the Rust fix tool
rust-fix-tool:
	@echo "Building Rust fix tool..."
	cd rust-fix-scripts && cargo build --release
	@echo "Rust fix tool built at rust-fix-scripts/$(RUST_FIX_PATH)"

docs:
	cargo doc --no-deps --document-private-items --open

clippy:
	cargo clippy -- -D warnings

fmt:
	cargo fmt --all

# Installation targets
install: build
	@echo "Installing Anya Core..."
ifeq ($(DETECTED_OS),Windows)
	@echo "You may need administrator privileges for installation"
	cargo run --bin anya-installer -- install
else
	@echo "You may need to use sudo for installation"
	cargo run --bin anya-installer -- install
endif

uninstall:
	@echo "Uninstalling Anya Core..."
ifeq ($(DETECTED_OS),Windows)
	@echo "You may need administrator privileges for uninstallation"
	cargo run --bin anya-installer -- uninstall
else
	@echo "You may need to use sudo for uninstallation"
	cargo run --bin anya-installer -- uninstall
endif

# PowerShell Core targets (cross-platform)
pwsh-install:
	@echo "Running PowerShell installer (cross-platform)..."
ifeq ($(DETECTED_OS),Windows)
	powershell -File install/Install-AnyaCore.ps1
else
	pwsh -File install/Install-AnyaCore.ps1
endif

# Help target
help:
	@echo "Cross-platform Makefile for Anya Core"
	@echo ""
	@echo "Available targets:"
	@echo "  all          - Fix issues, build, and test (default)"
	@echo "  check        - Check compilation without building"
	@echo "  build        - Build in release mode"
	@echo "  test         - Run all tests"
	@echo "  clean        - Remove build artifacts"
	@echo "  fix          - Run all fix scripts"
	@echo "  fix-edition  - Fix edition inheritance issues"
	@echo "  fix-refs     - Fix package references"
	@echo "  rust-fix-tool - Build the Rust-based fix tool"
	@echo "  docs         - Generate and open documentation"
	@echo "  clippy       - Run clippy lints"
	@echo "  fmt          - Format code"
	@echo "  install      - Install Anya Core"
	@echo "  uninstall    - Uninstall Anya Core"
	@echo "  pwsh-install - Run the PowerShell installer (cross-platform)"
	@echo "  help         - Show this help message"
	@echo ""
	@echo "Detected OS: $(DETECTED_OS)" 