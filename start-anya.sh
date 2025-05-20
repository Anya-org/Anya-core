#!/bin/bash
# [AIR-3][AIS-3][BPC-3][RES-3]

# Stop any running containers
echo "Stopping any running Anya Core services..."
sudo docker-compose -f docker-compose.min.yml down

# Clean up resources
echo "Cleaning up system resources..."
sudo sync && echo 3 | sudo tee /proc/sys/vm/drop_caches > /dev/null

# Start services with resource limits
echo "Starting Anya Core services..."
sudo docker-compose -f docker-compose.min.yml up -d

# Wait for services to initialize
echo "Waiting for services to start..."
sleep 10

# Verify services are running
echo "\nService Status:"
echo "--------------"

# Check Bitcoin
if curl -s http://localhost:8332 >/dev/null; then
    echo "✅ Bitcoin RPC is running"
else
    echo "❌ Bitcoin RPC is not accessible"
fi

# Check DWN
if curl -s http://localhost:3000 >/dev/null; then
    echo "✅ DWN is running"
else
    echo "❌ DWN is not accessible"
fi

# Check Anya Core
if curl -s http://localhost:8080/health >/dev/null; then
    echo "✅ Anya Core is running"
else
    echo "❌ Anya Core is not accessible"
fi

echo "\nResource Usage:"
echo "--------------"
sudo docker stats --no-stream

echo "\nTo view logs: docker-compose -f docker-compose.min.yml logs -f"
echo "To stop services: docker-compose -f docker-compose.min.yml down"
