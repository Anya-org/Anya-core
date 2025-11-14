#!/usr/bin/env bash
set -euo pipefail

echo "[cache-maintenance] Starting";

TARGET_DIR=${CARGO_TARGET_DIR:-"$(pwd)/target"}
SCCACHE_DIR=${SCCACHE_DIR:-"/var/cache/sccache"}
CARGO_HOME=${CARGO_HOME:-"/usr/local/cargo"}

HUMAN_DF=$(df -h "$TARGET_DIR" | awk 'NR==2')

size_or_zero() { du -sh "$1" 2>/dev/null | awk '{print $1}' || echo 0; }

TARGET_SIZE=$(size_or_zero "$TARGET_DIR")
SCCACHE_SIZE=$(size_or_zero "$SCCACHE_DIR")
REGISTRY_SIZE=$(size_or_zero "$CARGO_HOME/registry")
GIT_SIZE=$(size_or_zero "$CARGO_HOME/git")

echo "Filesystem usage: $HUMAN_DF"

echo "Target:   $TARGET_DIR => $TARGET_SIZE"
echo "sccache:  $SCCACHE_DIR => $SCCACHE_SIZE"
echo "registry: $CARGO_HOME/registry => $REGISTRY_SIZE"
echo "cargo git: $CARGO_HOME/git => $GIT_SIZE"

if command -v sccache >/dev/null 2>&1; then
  echo "sccache stats:"; sccache --show-stats || true
fi

echo "[cache-maintenance] Done.";
