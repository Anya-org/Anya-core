#!/bin/bash
#
# Server Setup Script for Anya Core
# This script sets up the required services on the server
# Following the Bitcoin Development Framework principles v2.5
# Implementing Hexagonal Architecture for clean separation of concerns

set -e

echo "==== Anya Core Server Setup ===="
echo "Version: 1.0.0"
echo "Following Bitcoin Development Framework v2.5"
echo "Implementing Hexagonal Architecture"
echo

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print success messages
success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

# Function to print warning messages
warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

# Function to print error messages
error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

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
    docker-compose \
    gnupg \
    apt-transport-https \
    ca-certificates \
    software-properties-common \
    ufw \
    jq

# Enable and start Docker
echo -e "${GREEN}Enabling and starting Docker...${NC}"
systemctl enable docker
systemctl start docker

# Create Bitcoin RPC configuration for public endpoint
echo -e "${GREEN}Setting up Bitcoin RPC configuration for public endpoint...${NC}"
mkdir -p /home/anya/.bitcoin
cat > /home/anya/.bitcoin/bitcoin.conf << EOF
# Bitcoin RPC configuration
# Using public Bitcoin RPC endpoint
rpcconnect=testnet.getblock.io
rpcport=3333
rpcuser=mainnet_8731
rpcpassword=c8f7af8a-c33c-4f49-954c-a997a50b9a22
testnet=1

# BIP Support
# Taproot (BIP 341/342) is enabled on the public node
EOF

# Create Bitcoin connection check script
echo -e "${GREEN}Creating Bitcoin connection check script...${NC}"
cat > /home/anya/check_bitcoin_connection.sh << EOF
#!/bin/bash
# Test connection to public Bitcoin RPC
curl -s --user mainnet_8731:c8f7af8a-c33c-4f49-954c-a997a50b9a22 \
     -H 'Content-Type: application/json' \
     -d '{"jsonrpc":"1.0","id":"anya-test","method":"getblockchaininfo","params":[]}' \
     https://testnet.getblock.io/3333/ | jq .
EOF
chmod +x /home/anya/check_bitcoin_connection.sh

# Test the connection
echo -e "${GREEN}Testing connection to public Bitcoin RPC...${NC}"
sudo -u anya /home/anya/check_bitcoin_connection.sh
success "Connection to public Bitcoin RPC successful"

# Set ownership
chown -R anya:anya /home/anya/.bitcoin

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
BITCOIN_RPC_URL=https://testnet.getblock.io/3333/
BITCOIN_RPC_USER=mainnet_8731
BITCOIN_RPC_PASSWORD=c8f7af8a-c33c-4f49-954c-a997a50b9a22
BITCOIN_NETWORK=testnet
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
  - job_name: 'web5-dwn'
    static_configs:
      - targets: ['localhost:3000']
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
systemctl enable web5-dwn
systemctl enable prometheus

systemctl start web5-dwn
systemctl start prometheus

# Configure firewall
echo -e "${GREEN}Configuring firewall...${NC}"
ufw allow ssh
ufw allow 3000/tcp  # Web5 DWN
ufw allow 9090/tcp  # Prometheus
ufw --force enable

# Create Anya Core directory structure
echo -e "${GREEN}Creating Anya Core directory structure...${NC}"
mkdir -p /home/anya/projectanya/{src,config,data,logs}
mkdir -p /home/anya/projectanya/src/{core,adapters,ports}
chown -R anya:anya /home/anya/projectanya

echo -e "${GREEN}Server setup completed!${NC}"
echo -e "${YELLOW}Services and ports:${NC}"
echo -e "Web5 DWN: 3000"
echo -e "Prometheus: 9090"
echo -e "Using public Bitcoin RPC at testnet.getblock.io:3333"

echo
echo "==== Anya Core Server Setup Complete ===="
echo
echo "Web5 DWN: $(systemctl is-active web5-dwn)"
echo "Prometheus: $(systemctl is-active prometheus)"
echo
echo "Next steps:"
echo "1. Clone Anya Core repository to /home/anya/projectanya"
echo "2. Install Anya Core dependencies"
echo "3. Configure Anya Core"
echo
success "Server setup completed successfully" 