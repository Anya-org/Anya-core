#!/bin/bash

# This script helps manage feature flags in the Cargo.toml file.

set -e

# Function to enable a feature
enable_feature() {
    sed -i "s/^#\s*\($1\s*=\s*\[.*\]\)/\1/" Cargo.toml
}

# Function to disable a feature
disable_feature() {
    sed -i "s/^\($1\s*=\s*\[.*\]\)/# \1/" Cargo.toml
}

# Check for the correct number of arguments
if [ "$#" -ne 2 ]; then
    echo "Usage: $0 <enable|disable> <feature>"
    exit 1
fi

# Get the action and feature from the command line
action=$1
feature=$2

# Perform the requested action
case "$action" in
    enable)
        enable_feature "$feature"
        echo "Feature '$feature' enabled."
        ;;
    disable)
        disable_feature "$feature"
        echo "Feature '$feature' disabled."
        ;;
    *)
        echo "Invalid action: $action"
        echo "Usage: $0 <enable|disable> <feature>"
        exit 1
        ;;
esac
