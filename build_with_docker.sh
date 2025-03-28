#!/bin/bash
set -e

echo "Building with Docker using latest Rust version..."

# Create a Dockerfile
cat > Dockerfile << 'DOCKERFILE'
FROM rust:latest

WORKDIR /usr/src/anya-core
COPY . .
RUN cargo build --release
DOCKERFILE

# Build the Docker image
docker build -t anya-core-builder .

# Extract the built binaries
docker create --name anya-core-container anya-core-builder
mkdir -p target/docker-release
docker cp anya-core-container:/usr/src/anya-core/target/release/anya-core target/docker-release/
docker cp anya-core-container:/usr/src/anya-core/target/release/unified_installer target/docker-release/
docker rm -f anya-core-container

echo "Build complete. Binaries are available in target/docker-release/"
ls -la target/docker-release/
