#!/bin/bash
# Local approximation of critical CI workflows (unified-ci + secret-scan + docs)
set -euo pipefail

echo "[local-ci] Starting local CI approximation"

have() { command -v "$1" >/dev/null 2>&1; }

phase() { echo; echo "==== $* ===="; }

phase "Cargo check (all features)"; cargo check --all-features || { echo "cargo check failed"; exit 1; }

if have cargo-deny; then
  phase "cargo deny"; cargo deny check || { echo "cargo deny failed"; exit 1; }
else
  echo "[local-ci] cargo-deny not installed (skip). Install: cargo install cargo-deny";
fi

if [ -f scripts/f_drive_offload.sh ]; then
  phase "Light secret scan"; bash scripts/f_drive_offload.sh scan-secrets || true
  phase "Deep secret scan"; bash scripts/f_drive_offload.sh scan-secrets-deep || true
fi

if have gitleaks; then
  phase "Gitleaks scan"; gitleaks detect --no-git --redact || { echo "Gitleaks found potential secrets"; exit 1; }
else
  echo "[local-ci] gitleaks not installed (skip). Install: curl -sSfL https://github.com/gitleaks/gitleaks/releases/latest/download/gitleaks_$(uname -s)_$(uname -m).tar.gz | tar -xz -C /usr/local/bin gitleaks";
fi

if have markdownlint; then
  phase "Markdownlint docs"; markdownlint docs/**/*.md --ignore docs/template.md || true
fi

echo "[local-ci] Completed"
