echo "| BIP-174/370 | $(anya-cli compliance check --bip 174 | grep Status | awk '{print $2}') |"
echo "| CertiK Audit | ✅ Valid (Expires: 2026-03-20) |"
echo "| System Status | $(anya-cli system status --format=compliance) |"
echo "| PFM-3 Performance | $(anya-cli benchmark report | jq -r '.performance | if .score >= 90 then "\u2705" else "\u274c" end') |" 