#!/bin/bash
# F Drive Offload Helper (WSL)
# Safely experiment with putting selected Rust build artifacts on /mnt/f.
# WARNING: Windows /mnt/f (drvfs/9p) is ~10-20x slower for many small file ops.
# Benchmark from this machine (64MB sync write):
#  - $HOME (ext4): ~39 MB/s
#  - /mnt/f (drvfs): ~2.6 MB/s
# Strategy: Keep hot compilation (source + incremental) on ext4. Optionally move
# cold artifacts (release builds, old target dirs, compressed caches) to F.

set -euo pipefail

COLOR() { printf "\033[1;34m%s\033[0m\n" "$*"; }
ERR() { printf "\033[1;31m%s\033[0m\n" "$*" >&2; }

SCRIPT_DIR=$(cd "$(dirname "$0")" && pwd)
F_MOUNT=/mnt/f
F_BASE="$F_MOUNT/anya-offload"
TARGET_OFFLOAD_DIR="$F_BASE/target"
REGISTRY_ARCHIVE_DIR="$F_BASE/registry-archives"

usage() {
  cat <<EOF
F Drive Offload Helper
Usage: $0 <command>

Commands:
  assess            Show current sizes & performance warning
  plan              Show recommended offload plan (safe defaults)
  archive-old       Tar + gzip old target artifacts (debug/release) to F and delete originals
  move-release      Move only target/release (keeping incremental debug builds local)
  enable-target     Set CARGO_TARGET_DIR to F (NOT recommended for perf) via ~/.bashrc
  disable-target    Remove CARGO_TARGET_DIR line from ~/.bashrc (no deletion of data)
  move-registry     Copy ~/.cargo/registry to F, symlink back (can slow builds; optional)
  reclaim           Run cargo cache trim & clean after archiving
  help              Show this help
EOF
}

require_mount() {
  if ! mount | grep -q " on $F_MOUNT "; then
    ERR "$F_MOUNT not mounted. Mount with: sudo mount -t drvfs F: /mnt/f -o metadata"; exit 1;
  fi
}

size_line() { du -sh "$1" 2>/dev/null || true; }

assess() {
  require_mount
  COLOR "Sizes:"; 
  size_line "$HOME/.cargo"; 
  size_line "$HOME/.rustup"; 
  size_line "$PWD/target"; 
  [ -d "$TARGET_OFFLOAD_DIR" ] && size_line "$TARGET_OFFLOAD_DIR";
  echo ""; COLOR "Filesystem performance note:"; echo "Ext4 home build path is much faster than drvfs (/mnt/f). Keep active compilation on ext4 for speed.";
}

plan() {
  cat <<'EOP'
Recommended Phased Plan:
1. archive-old     - Compress stale target artifacts to free space immediately.
2. reclaim         - Trim cargo cache (preserves needed indices) and cargo clean.
3. move-release    - If release builds are infrequent/large, move only target/release to F.
4. (optional) move-registry - Only if disk pressure remains high and you accept slower registry access.
5. Avoid enable-target unless disk exhaustion forces full offload; expect slower builds.
Rollback: Reverse symlinks or move directories back; remove added CARGO_TARGET_DIR lines.
EOP
}

archive_old() {
  require_mount
  mkdir -p "$F_BASE/archives"
  ts=$(date +%Y%m%d_%H%M%S)
  if [ -d target ]; then
    COLOR "Archiving target (incremental + release) -> $F_BASE/archives/target_$ts.tar.zst";
    tar -I 'zstd -19 -T0' -cf "$F_BASE/archives/target_$ts.tar.zst" target
  else
    ERR "No target dir present"; return 0;
  fi
}

move_release() {
  require_mount
  if [ ! -d target/release ]; then ERR "No target/release yet"; return 0; fi
  mkdir -p "$TARGET_OFFLOAD_DIR"
  if [ -d "$TARGET_OFFLOAD_DIR/release" ]; then ERR "Release already moved"; return 1; fi
  COLOR "Moving target/release -> $TARGET_OFFLOAD_DIR/release";
  rsync -a --remove-source-files target/release/ "$TARGET_OFFLOAD_DIR/release/" || true
  find target/release -type f -empty -delete || true
  rmdir target/release 2>/dev/null || true
  ln -s "$TARGET_OFFLOAD_DIR/release" target/release
  COLOR "Done. Symlink created.";
}

enable_target() {
  require_mount
  mkdir -p "$TARGET_OFFLOAD_DIR/full"
  if ! grep -q 'CARGO_TARGET_DIR=' "$HOME/.bashrc"; then
    echo "export CARGO_TARGET_DIR=$TARGET_OFFLOAD_DIR/full" >> "$HOME/.bashrc"
    COLOR "Added CARGO_TARGET_DIR to ~/.bashrc (effective for new shells)."
  else
    ERR "CARGO_TARGET_DIR already set in ~/.bashrc; adjust manually if needed.";
  fi
}

disable_target() {
  if grep -q 'CARGO_TARGET_DIR=' "$HOME/.bashrc"; then
    sed -i.bak '/CARGO_TARGET_DIR=/d' "$HOME/.bashrc"
    COLOR "Removed CARGO_TARGET_DIR from ~/.bashrc (backup at ~/.bashrc.bak)."
  else
    ERR "No CARGO_TARGET_DIR entry found in ~/.bashrc";
  fi
}

move_registry() {
  require_mount
  local SRC="$HOME/.cargo/registry"
  [ -d "$SRC" ] || { ERR "No cargo registry at $SRC"; return 0; }
  local DEST="$F_BASE/registry"
  if [ -L "$SRC" ]; then ERR "Registry already symlinked"; return 0; fi
  COLOR "Copying registry to $DEST (may take time)";
  mkdir -p "$DEST"
  rsync -a "$SRC/" "$DEST/"
  COLOR "Creating backup + symlink";
  mv "$SRC" "$SRC.backup.$(date +%s)"
  ln -s "$DEST" "$SRC"
  COLOR "Registry offload complete.";
}

reclaim() {
  COLOR "Cargo cache trim";
  cargo cache -a prune 2>/dev/null || echo "Install cargo-cache for deeper prune: cargo install cargo-cache";
  COLOR "Running cargo clean (debug artifacts removed)";
  cargo clean || true
}

cmd=${1:-help}
case $cmd in
  assess) assess ;;
  plan) plan ;;
  archive-old) archive_old ;;
  move-release) move_release ;;
  enable-target) enable_target ;;
  disable-target) disable_target ;;
  move-registry) move_registry ;;
  reclaim) reclaim ;;
  help|*) usage ;;
esac
