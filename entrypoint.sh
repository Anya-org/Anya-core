#!/bin/sh
set -e

# Placeholder for pre-start setup tasks
# Example: Run database migrations, check configurations, etc.
# echo "Running pre-start setup tasks..."
# your_setup_command_here

# Execute the main command (passed as arguments to this script)
echo "Starting Anya Core application..."
exec "$@"
