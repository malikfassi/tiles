#!/bin/bash
set -e

# Load constants
source scripts/00_load_constants.sh

# Create state directory
mkdir -p scripts/state

# State file
CURRENT_STATE_FILE="scripts/state/01_build_contracts.state"
touch "$CURRENT_STATE_FILE"

# Colors
BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

# Function to check if a step is completed
check_step() {
    grep -q "^$1=done$" "$CURRENT_STATE_FILE"
    return $?
}

# Function to mark step as completed
mark_step_done() {
    echo "$1=done" >> "$CURRENT_STATE_FILE"
}

# Create artifacts directory if it doesn't exist
mkdir -p artifacts

# Build tile contract
if ! check_step "tile"; then
    echo -e "${BLUE}1. Building Tile Contract...${NC}"
    
    if cargo build --release --target wasm32-unknown-unknown; then
        cp target/wasm32-unknown-unknown/release/tiles.wasm artifacts/
        echo -e "${GREEN}✅ Tile contract built successfully${NC}"
        mark_step_done "tile"
    else
        echo -e "${RED}❌ Failed to build tile contract${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}Skipping tile contract build (already completed)${NC}"
fi

# Download minter contract
if ! check_step "minter"; then
    echo -e "${BLUE}2. Downloading Minter Contract...${NC}"
    
    MINTER_URL="https://github.com/public-awesome/launchpad/releases/download/v3.15.0/vending_minter.wasm"
    if curl -L -o artifacts/vending_minter.wasm "$MINTER_URL"; then
        echo -e "${GREEN}✅ Minter contract downloaded successfully${NC}"
        mark_step_done "minter"
    else
        echo -e "${RED}❌ Failed to download minter contract${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}Skipping minter contract download (already completed)${NC}"
fi

# Download factory contract
if ! check_step "factory"; then
    echo -e "${BLUE}3. Downloading Factory Contract...${NC}"
    
    FACTORY_URL="https://github.com/public-awesome/launchpad/releases/download/v3.15.0/vending_factory.wasm"
    if curl -L -o artifacts/vending_factory.wasm "$FACTORY_URL"; then
        echo -e "${GREEN}✅ Factory contract downloaded successfully${NC}"
        mark_step_done "factory"
    else
        echo -e "${RED}❌ Failed to download factory contract${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}Skipping factory contract download (already completed)${NC}"
fi 