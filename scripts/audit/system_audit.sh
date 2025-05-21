#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]
# System Audit Tool

# Security Checks
if [ -f /etc/anya.conf ]; then
  grep 'HARDENING_LEVEL' /etc/anya.conf
else
  echo "No anya.conf found"
fi
ss -tulpn | grep ':8334'

# Configuration Validation
if command -v anya-core &>/dev/null; then
  anya-core --verify-config
else
  echo "anya-core not installed"
fi

# Permission Verification
check_permissions() {
  find /etc/anya-core -type d ! -perm 0755 -exec chmod 0755 {} \;
  find /etc/anya-core -type f ! -perm 0644 -exec chmod 0644 {} \;
}

# Repair Functions
repair_permissions() {
  find /etc/anya -type d -exec chmod 0755 {} \;
  find /etc/anya -type f -exec chmod 0644 {} \;
}
