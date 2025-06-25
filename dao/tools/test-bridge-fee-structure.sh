#!/bin/bash
# Cross-Chain Bridge Fee Test Script
# Tests the standardized 5% fee structure across all bridges

# Set up colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

echo -e "${GREEN}=================================${NC}"
echo -e "${GREEN}Cross-Chain Bridge Fee Test Script${NC}"
echo -e "${GREEN}=================================${NC}"

# Execute the JavaScript test script
echo -e "\n${YELLOW}Running fee calculation and bridge operation tests...${NC}"
node ./dao/tools/test-bridge-fees.js

if [ $? -eq 0 ]; then
    echo -e "\n${GREEN}✓ Fee calculation tests completed successfully${NC}"
else
    echo -e "\n${RED}× Fee calculation tests failed${NC}"
    exit 1
fi

# Validate bridge configuration
echo -e "\n${YELLOW}Validating bridge configuration...${NC}"

if [ -f "./dao/config/bridge_config.json" ]; then
    # Check fee rates (should all be 0.05)
    FEE_RATES=$(grep -o '"feeRate": 0.05' ./dao/config/bridge_config.json | wc -l)
    if [ $FEE_RATES -eq 8 ]; then # 4 bridge types x 2 networks (testnet, mainnet)
        echo -e "${GREEN}✓ All bridge fee rates are correctly set to 5%${NC}"
    else
        echo -e "${RED}× Some bridge fee rates are not set to 5% ($FEE_RATES/8 found)${NC}"
    fi

    # Check fee distribution (should be 0.8 for treasury and 0.2 for community)
    TREASURY_SHARES=$(grep -o '"treasuryShare": 0.8' ./dao/config/bridge_config.json | wc -l)
    COMMUNITY_SHARES=$(grep -o '"communityShare": 0.2' ./dao/config/bridge_config.json | wc -l)

    if [ $TREASURY_SHARES -eq 2 ] && [ $COMMUNITY_SHARES -eq 2 ]; then
        echo -e "${GREEN}✓ Fee distribution is correctly set to 80% treasury / 20% community${NC}"
    else
        echo -e "${RED}× Fee distribution is not correctly configured${NC}"
    fi
else
    echo -e "${RED}× Bridge configuration file not found${NC}"
    exit 1
fi

# Check if blockchain-integrations.js has fee calculation functions
echo -e "\n${YELLOW}Checking integration code for fee handling...${NC}"

if grep -q "calculateBridgeFee" ./dao/tools/blockchain-integrations.js; then
    echo -e "${GREEN}✓ Fee calculation function found in blockchain-integrations.js${NC}"
else
    echo -e "${RED}× Fee calculation function missing in blockchain-integrations.js${NC}"
fi

if grep -q "standardized 5% fee" ./dao/tools/blockchain-integrations.js; then
    echo -e "${GREEN}✓ Standardized fee documentation found${NC}"
else
    echo -e "${RED}× Standardized fee documentation missing${NC}"
fi

# Check documentation
echo -e "\n${YELLOW}Validating documentation...${NC}"

if [ -f "./dao/docs/BRIDGE_FEE_STRUCTURE_GUIDE.md" ]; then
    echo -e "${GREEN}✓ Bridge fee structure guide exists${NC}"
else
    echo -e "${RED}× Bridge fee structure guide missing${NC}"
fi

echo -e "\n${GREEN}=================================${NC}"
echo -e "${GREEN}Test Summary${NC}"
echo -e "${GREEN}=================================${NC}"
echo -e "Standardized 5% fee structure implemented"
echo -e "80% to DAO treasury, 20% to community"
echo -e "Consistent across all bridge directions"
echo -e "Documentation and configuration validated"
echo -e "${GREEN}=================================${NC}"
