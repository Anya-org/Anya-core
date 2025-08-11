#!/bin/bash
# F Drive Offload Helper (WSL)
# Safely experiment with putting selected Rust build artifacts on /mnt/f.
# WARNING: Windows /mnt/f (drvfs/9p) is ~10-20x slower for many small file ops.
# Benchmark from this machine (64MB sync write):
#  - $HOME (ext4): ~39 MB/s
#  - /mnt/f (drvfs): ~2.6 MB/s
# Strategy: Keep hot compilation (source + incremental) on ext4. Optionally move
# cold artifacts (release builds, old target dirs, compressed caches) to F.
# NEVER offload active incremental debug builds unless absolutely required for space.
#
# Quick micro-benchmark (compare ext4 vs drvfs):
#   dd if=/dev/zero of=/tmp/ext4_test.bin bs=1M count=64 oflag=dsync && rm /tmp/ext4_test.bin
#   dd if=/dev/zero of=/mnt/f/drvfs_test.bin bs=1M count=64 oflag=dsync && rm /mnt/f/drvfs_test.bin
# (Expect drvfs to be significantly slower; keep frequent rebuild paths off /mnt/f.)

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
  plan              Show recommended phased offload plan
  archive-old       Tar + zstd old target artifacts (debug+release) to F (space win)
  move-release      Move only target/release (keep fast incremental locally)
  enable-target     Set CARGO_TARGET_DIR to F (NOT recommended; slowdown)
  disable-target    Remove CARGO_TARGET_DIR line from ~/.bashrc
  move-registry     Copy ~/.cargo/registry to F, symlink back (adds latency)
  reclaim           Run cargo cache prune (if cargo-cache) + cargo clean
  verify-signing    Show last commit SSH signing status & config
  scan-secrets      Lightweight scan for accidental private key material (no network)
  scan-secrets-deep Extended heuristic scan (.env style tokens, AWS keys, generic secrets)
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

verify_signing() {
  COLOR "Git SSH signing status (last commit):";
  git log -1 --show-signature 2>/dev/null || echo "No commits yet or unable to show signature";
  COLOR "Configured signing key:";
  git config user.signingkey 2>/dev/null || echo "(none)";
  COLOR "Allowed signers file:";
  git config --global gpg.ssh.allowedSignersFile 2>/dev/null || echo "(not set)";
  echo "If unsigned: ensure ssh-agent has key: ssh-add ~/.ssh/id_ed25519_anya";
}

scan_secrets() {
  COLOR "Scanning workspace for private key patterns (local only)...";
  # Patterns (kept intentionally tight). We do NOT read binary files (-I) and skip common large/irrelevant dirs.
  local EXCLUDES='(./.git|./target|./site|./node_modules|./docs/audit|./archives)'
  local PATTERNS='-----BEGIN (OPENSSH|RSA|EC|DSA) PRIVATE KEY-----'
  local hits
  hits=$(grep -ErIl --exclude-dir={.git,target,site,node_modules} --exclude=*.tar.zst --exclude=*.gz --exclude=*.zip -e "$PATTERNS" . || true)
  if [ -z "$hits" ]; then
    COLOR "No raw private key blocks found.";
  else
    ERR "Potential sensitive files detected:";
    printf '%s\n' "$hits"
    echo "Review and remove/redact before committing."
  fi
  echo "Note: This is a heuristic scan. For deeper scanning run: gitleaks detect --no-git";
}

scan_secrets_deep() {
  COLOR "Deep scan: private keys + common token patterns (.env, AWS, generic hex/api keys)";
  local PATTERNS=(
    '-----BEGIN (OPENSSH|RSA|EC|DSA) PRIVATE KEY-----'
    'AWS_ACCESS_KEY_ID=AKIA[0-9A-Z]{16}'
    'AKIA[0-9A-Z]{16}'
    'AWS_SECRET_ACCESS_KEY=[A-Za-z0-9/+=]{40}'
    '[A-Za-z0-9_]*API[_-]?KEY=[A-Za-z0-9]{24,}'
    '[A-Za-z0-9_]*SECRET[_-]?KEY=[A-Za-z0-9/+=]{24,}'
    'PRIVATE_KEY="?-----BEGIN'
    'ghp_[A-Za-z0-9]{36,}'
    'github_pat_[A-Za-z0-9_]{20,}'
    '[A-F0-9]{64}'
  )
  local EXCLUDES='--exclude-dir=.git --exclude-dir=target --exclude-dir=site --exclude-dir=node_modules'
  local findings=0
  for p in "${PATTERNS[@]}"; do
    local res
    res=$(grep -ErIn $EXCLUDES -e "$p" . 2>/dev/null || true)
    if [ -n "$res" ]; then
      findings=1
      ERR "Pattern match: $p";
      echo "$res" | head -n 40
    fi
  done
  if [ $findings -eq 0 ]; then
    COLOR "No suspicious patterns detected.";
  else
    ERR "Review above potential secrets before committing.";
  fi
  echo "Tip: Run 'gitleaks detect --redact' for authoritative scan.";
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
  verify-signing) verify_signing ;;
  scan-secrets) scan_secrets ;;
  scan-secrets-deep) scan_secrets_deep ;;
  help|*) usage ;;
esac
