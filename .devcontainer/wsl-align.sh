#!/usr/bin/env bash
set -euo pipefail

echo "[wsl-align] Detecting WSL environment and suggesting configuration";

if grep -qi microsoft /proc/version 2>/dev/null; then
  echo "[wsl-align] WSL detected.";
else
  echo "[wsl-align] Not running under WSL; exiting."; exit 0; fi

PWD_REAL=$(pwd -P)
if [[ $PWD_REAL == /mnt/* ]]; then
  echo "[wsl-align][WARN] Workspace resides under Windows mounted path ($PWD_REAL). For heavy Rust builds, clone into /home for better FS performance, then mount only caches from /mnt/f if needed.";
fi

if [[ -d /mnt/f ]]; then
  df -h /mnt/f | awk 'NR==1 || NR==2'
else
  echo "[wsl-align] /mnt/f not present. If you need F: drive, ensure it is attached in Windows and WSL can see it (wsl --shutdown then relaunch).";
fi

echo "[wsl-align] Suggested ~/.wslconfig (edit on Windows in %UserProfile%):"
cat <<'EOF'
[wsl2]
memory=8GB        # Adjust
processors=4      # Adjust
swap=4GB          # Optional
localhostForwarding=true
EOF

echo "[wsl-align] To apply .wslconfig changes: wsl --shutdown (from Windows PowerShell) then reopen WSL.";
echo "[wsl-align] Done.";
