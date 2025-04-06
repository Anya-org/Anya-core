# Anya Core Deployment Guide

This document provides instructions for deploying Anya Core to a remote server.

## Files Created/Modified

1. **Server Setup Scripts**:
   - `scripts/server_setup.sh`: Main server setup script that installs and configures all required components
   - `scripts/verify_installation.sh`: Script to verify the installation and check for compliance

2. **GitHub Workflows**:
   - `.github/workflows/deploy.yml`: Workflow for automated deployment
   - `.github/workflows/tunnel.yml`: Workflow for creating an SSH tunnel to the server

3. **Documentation**:
   - `SERVER_SETUP.md`: Detailed server setup guide
   - `DEPLOYMENT.md`: This deployment guide

4. **Utility Scripts**:
   - `Test-AnyaCore.ps1`: PowerShell script to test the setup files
   - `push_to_server.ps1`: PowerShell script to push files to the remote server

## Deployment Options

### Option 1: Manual Deployment

1. Run the test script to verify the setup files:
   ```powershell
   .\Test-AnyaCore.ps1
   ```

2. Push the files to the remote server:
   ```powershell
   .\push_to_server.ps1 -ServerHost "192.168.0.212" -ServerUser "anya"
   ```

3. SSH to the server and run the setup script:
   ```bash
   ssh anya@192.168.0.212
   cd ~/projectanya
   sudo ./scripts/server_setup.sh
   ```

4. Verify the installation:
   ```bash
   ./scripts/verify_installation.sh
   ```

### Option 2: Automated Deployment via PowerShell

1. Run the test script to verify the setup files:
   ```bash
   ./Test-AnyaCore.sh
   ```

2. Push the files to the remote server and run the setup:
   ```bash
   ./push_to_server.sh -ServerHost "192.168.0.212" -ServerUser "anya" -RunSetup
   ```

### Option 3: GitHub Actions Deployment

1. Add the following secrets to your GitHub repository:
   - `SERVER_HOST`: The hostname or IP address of your server (e.g., "192.168.0.212")
   - `SERVER_USER`: The username to use for SSH (e.g., "anya")
   - `SSH_PRIVATE_KEY`: The private SSH key for authentication

2. Go to the "Actions" tab in your GitHub repository.

3. Select the "Deploy Anya Core" workflow and run it.

## Post-Deployment

After deployment, you can:

1. Check the status of the services:
   ```bash
   systemctl status bitcoind
   systemctl status web5-dwn
   systemctl status prometheus
   ```

2. Access the Web5 DWN API at `http://your-server-ip:3000`

3. Access Prometheus metrics at `http://your-server-ip:9090`

## Troubleshooting

If you encounter issues during deployment:

1. Check the logs:
   ```bash
   journalctl -u bitcoind -n 50
   journalctl -u web5-dwn -n 50
   journalctl -u prometheus -n 50
   ```

2. Verify the firewall settings:
   ```bash
   sudo ufw status
   ```

3. Check if the required ports are open:
   ```bash
   nc -zv localhost 8332
   nc -zv localhost 18332
   nc -zv localhost 3000
   nc -zv localhost 9090
   ```

4. Run the verification script again:
   ```bash
   ./scripts/verify_installation.sh
   ```

## Security Considerations

- The setup script generates a random password for the Bitcoin RPC interface
- All configuration files have appropriate permissions
- The firewall is configured to allow only necessary ports
- Services run as non-root users

## Next Steps

After successful deployment:

1. Set up monitoring and alerts
2. Configure regular backups
3. Implement additional security measures
4. Deploy your application code to interact with the infrastructure