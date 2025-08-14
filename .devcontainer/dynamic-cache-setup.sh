#!/usr/bin/env bash
set -euo pipefail

log() { echo "[dynamic-cache] $*"; }
warn() { echo "[dynamic-cache][WARN] $*" >&2; }

MIN_FREE_GB=${DYNAMIC_CACHE_MIN_FREE_GB:-8}
FORCE_ROOT=${DYNAMIC_CACHE_FORCE_ROOT:-}
VERBOSE=${DYNAMIC_CACHE_VERBOSE:-0}
WORKDIR="${CONTAINER_WORKSPACE_FOLDER:-$PWD}"
DEFAULT_TARGET="$WORKDIR/target"
DEFAULT_SCCACHE="/var/cache/sccache"

# Candidate paths in order of preference (if they exist & have space)
CANDIDATES=()
# If a Windows F: style mount is present inside container (e.g., /workspace/F or /mnt/f)
for p in /workspace/F /mnt/f /f /F; do
  [[ -d "$p" ]] && CANDIDATES+=("$p/anya-build")
done
# Fallback to workspace local storage
CANDIDATES+=("$WORKDIR/.cache-local")

# Ensure candidate base directories
for c in "${CANDIDATES[@]}"; do
  mkdir -p "$c" || true
  [[ $VERBOSE == 1 ]] && log "Candidate prepared: $c"
done

choose_path() {
  local chosen=""
  for base in "${CANDIDATES[@]}"; do
    mkdir -p "$base" || continue
    # Get free GB on filesystem hosting base
    # Use df -Pk to ensure consistent block size
    local free=$(df -Pk "$base" | awk 'NR==2 {print int($4/1024/1024)}')
    [[ -z "$free" ]] && continue
    [[ $VERBOSE == 1 ]] && log "Free space at $base: ${free}GB"
    if (( free >= MIN_FREE_GB )); then
      chosen="$base"
      break
    fi
  done
  if [[ -z "$chosen" ]]; then
    warn "No candidate with >= ${MIN_FREE_GB}GB free. Using last candidate ${CANDIDATES[-1]} anyway."
    chosen="${CANDIDATES[-1]}"
  fi
  echo "$chosen"
}

if [[ -n "$FORCE_ROOT" ]]; then
  BASE="$FORCE_ROOT"
  log "FORCE_ROOT override: $BASE"
else
  BASE=$(choose_path)
fi

mkdir -p "$BASE/target" "$BASE/sccache" "$BASE/cargo-registry" || true

# Export to profile for subsequent shells
PROFILE_SNIPPET="# >>> dynamic cache paths >>>\nexport CARGO_TARGET_DIR=\"$BASE/target\"\nexport SCCACHE_DIR=\"$BASE/sccache\"\nexport RUSTC_WRAPPER=\"sccache\"\n# <<< dynamic cache paths <<<"

if ! grep -q 'dynamic cache paths' /home/vscode/.bashrc 2>/dev/null; then
  echo -e "${PROFILE_SNIPPET}" >> /home/vscode/.bashrc
fi

# Apply to current session for postCreateCommand tail
export CARGO_TARGET_DIR="$BASE/target"
export SCCACHE_DIR="$BASE/sccache"
export RUSTC_WRAPPER="sccache"

log "Selected base: $BASE"
log "CARGO_TARGET_DIR=$CARGO_TARGET_DIR"
log "SCCACHE_DIR=$SCCACHE_DIR"

# Optional: configure cargo sparse (idempotent)
mkdir -p /home/vscode/.cargo
if ! grep -q 'protocol' /home/vscode/.cargo/config.toml 2>/dev/null; then
  cat >> /home/vscode/.cargo/config.toml <<'EOF'
[source.crates-io]
protocol = "sparse"
EOF
fi

# Size hint
free_final=$(df -Pk "$BASE" | awk 'NR==2 {print int($4/1024/1024)}')
log "Free space on selected FS after setup: ${free_final}GB"

# Optional auto-prune if enabled and below threshold
AUTO_PRUNE=${AUTO_PRUNE_ON_SETUP:-1}
PRUNE_TRIGGER_GB=${PRUNE_TRIGGER_GB:-6}
if (( AUTO_PRUNE == 1 )) && (( free_final < PRUNE_TRIGGER_GB )); then
  if [[ -x "/workspaces/Anya-core/.devcontainer/prune-caches.sh" ]]; then
    log "Free space ${free_final}GB < ${PRUNE_TRIGGER_GB}GB; invoking prune-caches.sh"
    PRUNE_MIN_FREE_GB=${PRUNE_TRIGGER_GB} bash /workspaces/Anya-core/.devcontainer/prune-caches.sh || warn "Prune script encountered issues"
  else
    warn "Prune script not found or not executable"
  fi
fi

exit 0
