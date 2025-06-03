#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# Build for Linux
cross build --target x86_64-unknown-linux-gnu --release

# Optional: Build for multiple architectures
cross build --target aarch64-unknown-linux-gnu --release

