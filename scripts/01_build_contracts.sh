#!/bin/bash

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Load environment variables and constants
source .env
source scripts/00_load_constants.sh

# Create necessary directories
mkdir -p artifacts
mkdir -p scripts/state

# State file
STATE_FILE="scripts/state/01_build_contracts.state"
touch "$STATE_FILE"

# Function to check if a step is completed
check_step() {
    grep -q "^$1=done$" "$STATE_FILE"
    return $?
}

# Function to mark step as completed
mark_step_done() {
    echo "$1=done" >> "$STATE_FILE"
}

# Function to show step status
show_status() {
    echo -e "${BLUE}Build Status:${NC}"
    echo -e "1. Tile Contract: $(check_step "tile" && echo -e "${GREEN}✓${NC}" || echo -e "${RED}✗${NC}")"
    echo -e "2. Minter Contract: $(check_step "minter" && echo -e "${GREEN}✓${NC}" || echo -e "${RED}✗${NC}")"
    echo -e "3. Factory Contract: $(check_step "factory" && echo -e "${GREEN}✓${NC}" || echo -e "${RED}✗${NC}")"
    echo
}

# Parse command line arguments
START_STEP=1
FORCE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        --start-step)
            START_STEP="$2"
            shift 2
            ;;
        --force)
            FORCE=true
            shift
            ;;
        --help)
            echo "Usage: $0 [--start-step <1-3>] [--force]"
            echo "  --start-step: Start from specific step (1=tile, 2=minter, 3=factory)"
            echo "  --force: Force rebuild even if step was completed"
            exit 0
            ;;
        *)
            echo "Unknown option: $1"
            exit 1
            ;;
    esac
done

# Show initial status
show_status

# Build tile contract
if [[ $START_STEP -le 1 ]] && { $FORCE || ! check_step "tile"; }; then
    echo -e "${BLUE}1. Building Tile Contract...${NC}"
    if cargo build --release --target wasm32-unknown-unknown --locked; then
        # Optimize tile contract
        echo -e "${BLUE}Optimizing tile contract...${NC}"
        TILE_WASM="target/wasm32-unknown-unknown/release/tiles.wasm"
        if [ -f "$TILE_WASM" ]; then
            cp "$TILE_WASM" "artifacts/tiles.wasm"
            echo -e "${GREEN}✅ Tile contract built successfully${NC}"
            mark_step_done "tile"
        else
            echo -e "${RED}❌ Tile contract wasm not found${NC}"
            exit 1
        fi
    else
        echo -e "${RED}❌ Tile contract build failed${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}Skipping tile contract build (already completed)${NC}"
fi

# Build minter contract
if [[ $START_STEP -le 2 ]] && { $FORCE || ! check_step "minter"; }; then
    echo -e "${BLUE}2. Downloading Minter Contract...${NC}"
    
    # Create artifacts directory if it doesn't exist
    mkdir -p artifacts
    
    # Download the released wasm
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

# Build factory contract
if [[ $START_STEP -le 3 ]] && { $FORCE || ! check_step "factory"; }; then
    echo -e "${BLUE}3. Downloading Factory Contract...${NC}"
    
    # Create artifacts directory if it doesn't exist
    mkdir -p artifacts
    
    # Download the released wasm
    FACTORY_URL="https://github.com/public-awesome/launchpad/releases/download/v3.15.0/vending_factory.wasm"
    if curl -L -o artifacts/factory.wasm "$FACTORY_URL"; then
        echo -e "${GREEN}✅ Factory contract downloaded successfully${NC}"
        mark_step_done "factory"
    else
        echo -e "${RED}❌ Failed to download factory contract${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}Skipping factory contract download (already completed)${NC}"
fi

# Show final status
echo
show_status

if check_step "tile" && check_step "minter" && check_step "factory"; then
    echo -e "${GREEN}✅ All contracts built successfully!${NC}"
    echo -e "${BLUE}Artifacts:${NC}"
    ls -l artifacts/
else
    echo -e "${YELLOW}⚠️  Some steps are incomplete. Run again to complete remaining steps.${NC}"
fi 