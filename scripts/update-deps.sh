#!/bin/bash

# This script updates all dependencies in the workspace to the latest versions.

set -e

echo "Updating all dependencies..."
cargo update

echo "Dependencies updated successfully."
