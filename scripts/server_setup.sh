#!/bin/bash
#
# Server Setup Script for Anya Core
# This script sets up the required services on the server

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting Anya Core server setup...${NC}"

# Check if running as root
if [ "$(id -u)" -ne 0 ]; then
    echo -e "${YELLOW}This script should be run as root. Trying with sudo...${NC}"
    sudo "$0" "$@"
    exit $?
fi

# Update package list
echo -e "${GREEN}Updating package list...${NC}"
apt-get update

# Install required packages
echo -e "${GREEN}Installing required packages...${NC}"
apt-get install -y \
    git \
    curl \
    wget \
    build-essential \
    python3 \
    python3-pip \
    nodejs \
    npm \
    docker.io \
    docker-compose

# Enable and start Docker
echo -e "${GREEN}Enabling and starting Docker...${NC}"
systemctl enable docker
systemctl start docker

# Install Bitcoin Core
echo -e "${GREEN}Setting up Bitcoin Core...${NC}"
mkdir -p /home/anya/bitcoin
cd /home/anya/bitcoin

# Download Bitcoin Core
if [ ! -f "bitcoin-24.0.1-x86_64-linux-gnu.tar.gz" ]; then
    echo -e "${GREEN}Downloading Bitcoin Core...${NC}"
    wget https://bitcoincore.org/bin/bitcoin-core-24.0.1/bitcoin-24.0.1-x86_64-linux-gnu.tar.gz
    tar -xzf bitcoin-24.0.1-x86_64-linux-gnu.tar.gz
    cd bitcoin-24.0.1
    install -m 0755 -o root -g root -t /usr/local/bin bin/*
fi

# Create Bitcoin configuration
mkdir -p /home/anya/.bitcoin
cat > /home/anya/.bitcoin/bitcoin.conf << EOF
server=1
rpcuser=anya
rpcpassword=Cr3@t10n
rpcallowip=127.0.0.1
txindex=1
testnet=1
EOF

# Set ownership
chown -R anya:anya /home/anya/.bitcoin

# Create systemd service for Bitcoin Core
cat > /etc/systemd/system/bitcoind.service << EOF
[Unit]
Description=Bitcoin daemon
After=network.target

[Service]
User=anya
Group=anya
Type=forking
ExecStart=/usr/local/bin/bitcoind -daemon
Restart=always
RestartSec=30
TimeoutStartSec=30
TimeoutStopSec=30

[Install]
WantedBy=multi-user.target
EOF

# Set up Web5 DWN node
echo -e "${GREEN}Setting up Web5 DWN node...${NC}"
mkdir -p /home/anya/web5
cd /home/anya/web5

# Clone Web5 DWN repository
if [ ! -d "dwn-server" ]; then
    echo -e "${GREEN}Cloning Web5 DWN repository...${NC}"
    git clone https://github.com/TBD54566975/dwn-server.git
    cd dwn-server
    npm install
fi

# Create Web5 configuration
cat > /home/anya/web5/dwn-server/.env << EOF
PORT=3000
HOST=0.0.0.0
STORAGE_PATH=/home/anya/web5/data
LOG_LEVEL=info
EOF

# Create systemd service for Web5 DWN
cat > /etc/systemd/system/web5-dwn.service << EOF
[Unit]
Description=Web5 DWN Server
After=network.target

[Service]
User=anya
Group=anya
Type=simple
WorkingDirectory=/home/anya/web5/dwn-server
ExecStart=/usr/bin/npm start
Restart=always
RestartSec=10
StandardOutput=syslog
StandardError=syslog
SyslogIdentifier=web5-dwn

[Install]
WantedBy=multi-user.target
EOF

# Create systemd service for Prometheus metrics
echo -e "${GREEN}Setting up Prometheus for metrics...${NC}"
mkdir -p /home/anya/prometheus
cd /home/anya/prometheus

# Download Prometheus
if [ ! -f "prometheus-2.45.0.linux-amd64.tar.gz" ]; then
    echo -e "${GREEN}Downloading Prometheus...${NC}"
    wget https://github.com/prometheus/prometheus/releases/download/v2.45.0/prometheus-2.45.0.linux-amd64.tar.gz
    tar -xzf prometheus-2.45.0.linux-amd64.tar.gz
    mv prometheus-2.45.0.linux-amd64 prometheus
fi

# Create Prometheus configuration
cat > /home/anya/prometheus/prometheus/prometheus.yml << EOF
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'anya-core'
    static_configs:
      - targets: ['localhost:9090']
  - job_name: 'node'
    static_configs:
      - targets: ['localhost:9100']
EOF

# Create systemd service for Prometheus
cat > /etc/systemd/system/prometheus.service << EOF
[Unit]
Description=Prometheus
After=network.target

[Service]
User=anya
Group=anya
Type=simple
WorkingDirectory=/home/anya/prometheus/prometheus
ExecStart=/home/anya/prometheus/prometheus/prometheus --config.file=/home/anya/prometheus/prometheus/prometheus.yml
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

# Set ownership
chown -R anya:anya /home/anya/web5
chown -R anya:anya /home/anya/prometheus

# Reload systemd
echo -e "${GREEN}Reloading systemd...${NC}"
systemctl daemon-reload

# Enable and start services
echo -e "${GREEN}Enabling and starting services...${NC}"
systemctl enable bitcoind
systemctl enable web5-dwn
systemctl enable prometheus

systemctl start bitcoind
systemctl start web5-dwn
systemctl start prometheus

# Configure firewall
echo -e "${GREEN}Configuring firewall...${NC}"
apt-get install -y ufw

# Allow SSH
ufw allow 22/tcp

# Allow Bitcoin ports
ufw allow 8332/tcp  # RPC
ufw allow 8333/tcp  # P2P
ufw allow 18332/tcp # Testnet RPC
ufw allow 18333/tcp # Testnet P2P

# Allow Web5 ports
ufw allow 3000/tcp

# Allow Prometheus ports
ufw allow 9090/tcp

# Enable firewall
echo "y" | ufw enable

echo -e "${GREEN}Server setup completed!${NC}"
echo -e "${YELLOW}Services and ports:${NC}"
echo -e "Bitcoin Core RPC: 8332 (mainnet), 18332 (testnet)"
echo -e "Web5 DWN: 3000"
echo -e "Prometheus: 9090"

echo -e "${GREEN}You can now push your code to this server.${NC}" 