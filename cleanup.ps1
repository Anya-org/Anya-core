# Bitcoin Core cleanup script

# Bitcoin installation files
Remove-Item -Force -ErrorAction SilentlyContinue scripts/install_bitcoin_core.sh
Remove-Item -Force -ErrorAction SilentlyContinue docker/bitcoin-core/Dockerfile
Remove-Item -Force -ErrorAction SilentlyContinue docker-compose.bitcoin.yml

# Test files
Remove-Item -Force -ErrorAction SilentlyContinue src/test/bitcoin_local.rs
Remove-Item -Force -ErrorAction SilentlyContinue src/test/bitcoin_core_installation.rs
Remove-Item -Force -ErrorAction SilentlyContinue src/bitcoin/installation.rs

# RPC files
Remove-Item -Force -ErrorAction SilentlyContinue src/bitcoin/rpc/local_node.rs

Write-Host "Cleanup completed successfully" -ForegroundColor Green
