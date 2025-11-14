#!/usr/bin/env bash
set -euo pipefail

# Automated pruning when free space below threshold.
# Thresholds: PRUNE_MIN_FREE_GB, PRUNE_TARGET_MAX_GB, PRUNE_VERBOSE

MIN_FREE_GB=${PRUNE_MIN_FREE_GB:-6}
TARGET_MAX_GB=${PRUNE_TARGET_MAX_GB:-15}
VERBOSE=${PRUNE_VERBOSE:-1}

log() { echo "[prune-caches] $*"; }
warn() { echo "[prune-caches][WARN] $*" >&2; }

TARGET_DIR=${CARGO_TARGET_DIR:-"$(pwd)/target"}
SCCACHE_DIR=${SCCACHE_DIR:-"/var/cache/sccache"}
CARGO_HOME=${CARGO_HOME:-"/usr/local/cargo"}

fs_free_gb() { df -Pk "$1" | awk 'NR==2 {print int($4/1024/1024)}'; }
dir_size_gb() { du -sk "$1" 2>/dev/null | awk '{printf "%d\n", $1/1024/1024}' || echo 0; }

BASE_PATH=$(dirname "$TARGET_DIR")
FREE=$(fs_free_gb "$BASE_PATH")
[[ -z "$FREE" ]] && FREE=0
log "Free space on filesystem: ${FREE}GB (min required ${MIN_FREE_GB}GB)"

TARGET_SIZE=$(dir_size_gb "$TARGET_DIR")
SCCACHE_SIZE=$(dir_size_gb "$SCCACHE_DIR")
REGISTRY_SIZE=$(dir_size_gb "$CARGO_HOME/registry")
GIT_SIZE=$(dir_size_gb "$CARGO_HOME/git")

log "Current sizes (GB): target=${TARGET_SIZE} sccache=${SCCACHE_SIZE} registry=${REGISTRY_SIZE} cargo-git=${GIT_SIZE}"

PRUNED=0

if (( FREE < MIN_FREE_GB )); then
  warn "Free space below threshold; initiating pruning sequence."

  if (( TARGET_SIZE > TARGET_MAX_GB )); then
    log "Pruning cargo target (size ${TARGET_SIZE}GB > ${TARGET_MAX_GB}GB)"
    rm -rf "${TARGET_DIR}" || warn "Failed to remove target"
    PRUNED=1
  fi

  if command -v sccache >/dev/null 2>&1; then
    log "Resetting sccache stats (no size shrink unless configured limit exceeded)"
    sccache --zero-stats || true
  fi

  if [[ -d "${CARGO_HOME}/registry/cache" ]]; then
    find "${CARGO_HOME}/registry/cache" -type f -name '*.crate' -mtime +30 -print -delete 2>/dev/null || true
  fi
else
  log "No pruning required."
fi

if (( PRUNED == 1 )); then
  NEW_FREE=$(fs_free_gb "$BASE_PATH")
  log "New free space: ${NEW_FREE}GB"
fi

log "Done."
