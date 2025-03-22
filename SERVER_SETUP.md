# Server Setup Guide for Anya Core

This document provides instructions for setting up the Anya Core server environment and deploying the application.

## Server Requirements

- Ubuntu 20.04 LTS or higher
- Minimum 4GB RAM
- At least 50GB disk space
- Internet connection

## Setup Methods

There are three ways to set up the server:

### 1. Manual Setup Using the Script

1. SSH into your server:
   ```bash
   ssh anya@192.168.0.212
   ```

2. Clone the repository:
   ```bash
   git clone https://github.com/Anya-org/anya-core.git
   cd anya-core
   ```

3. Run the server setup script:
   ```bash
   chmod +x scripts/server_setup.sh
   sudo ./scripts/server_setup.sh
   ```

### 2. GitHub Deployment

The repository includes GitHub Actions workflows for automated deployment:

1. Add your server SSH key to the GitHub repository secrets with name `SSH_PRIVATE_KEY`.

2. Go to the "Actions" tab in your GitHub repository.

3. Select the "Deploy to Server" workflow and run it.

### 3. GitHub Tunnel Method

If direct SSH to the server is not possible, you can use the GitHub tunnel method:

1. Add your server SSH key to the GitHub repository secrets with name `SSH_PRIVATE_KEY`.

2. Go to the "Actions" tab in your GitHub repository.

3. Select the "Server Tunnel" workflow and run it.

4. Once the tunnel is established, the workflow logs will show connection instructions.

5. From the server, connect to the tunnel:
   ```bash
   ssh -p 9922 runner@localhost
   ```

6. After connecting, you can transfer files and execute commands on the GitHub runner.

## Server Configuration

The setup script configures the following services:

| Service | Port | Description |
|---------|------|-------------|
| Bitcoin Core | 8332 (mainnet), 18332 (testnet) | Bitcoin node with RPC |
| Web5 DWN | 3000 | Decentralized Web Node |
| Prometheus | 9090 | Metrics and monitoring |

## Manual Service Control

Each service can be controlled using systemd:

```bash
# Bitcoin Core
sudo systemctl start bitcoind
sudo systemctl stop bitcoind
sudo systemctl status bitcoind

# Web5 DWN 
sudo systemctl start web5-dwn
sudo systemctl stop web5-dwn
sudo systemctl status web5-dwn

# Prometheus
sudo systemctl start prometheus
sudo systemctl stop prometheus
sudo systemctl status prometheus
```

## Troubleshooting

### Service Not Starting

Check the service status and logs:

```bash
sudo systemctl status bitcoind
sudo journalctl -u bitcoind -n 50
```

### Connection Issues

Make sure the firewall allows the necessary ports:

```bash
sudo ufw status
```

### Tunnel Issues

If the GitHub tunnel method fails, check:
- The SSH key is correctly added to GitHub secrets
- The server allows outbound SSH connections
- The server's firewall allows incoming connections on port 9922 