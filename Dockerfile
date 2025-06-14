# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/anya-core

# Copy the current directory contents into the container
COPY . .

# Copy the entrypoint script and make it executable
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

COPY docker/verify_bitcoin_metrics.sh /usr/local/bin/verify_bitcoin_metrics.sh
RUN chmod +x /usr/local/bin/verify_bitcoin_metrics.sh

# Install system dependencies (removed PostgreSQL for decentralized architecture)
RUN apt-get update && apt-get install -y \
    libssl-dev pkg-config && \
    rm -rf /var/lib/apt/lists/*

# Build the project
RUN cargo build --release

# Set up environment variables for decentralized operation
ENV ANYA_MODE=decentralized \
    ANYA_DATA_DIR=/usr/src/anya-core/data \
    ANYA_LOG_DIR=/usr/src/anya-core/logs

# Expose the application's port (Source: src/main.rs line 36)
EXPOSE 8080

# Set the entrypoint
ENTRYPOINT ["entrypoint.sh"]

# Run the application with correct binary name
CMD ["./target/release/anya"]