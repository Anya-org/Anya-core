# Use the official Rust image as the base image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/anya-core

# Copy the current directory contents into the container
COPY . .

# Copy the entrypoint script and make it executable
COPY entrypoint.sh /usr/local/bin/entrypoint.sh
RUN chmod +x /usr/local/bin/entrypoint.sh

# Install system dependencies
RUN apt-get update && apt-get install -y \
    postgresql postgresql-contrib \
    libssl-dev pkg-config

# Build the project
RUN cargo build --release

# Set up environment variables
ENV DATABASE_URL=postgres://postgres:anya_core_password@db/anya_core

# Expose the application's port
EXPOSE 8000 # Assuming anya-core runs on 8000, adjust if different

# Set the entrypoint
ENTRYPOINT ["entrypoint.sh"]

# Run the application (this will be passed to entrypoint.sh)
CMD ["cargo", "run", "--release"]