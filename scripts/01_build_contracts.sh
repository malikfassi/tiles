#!/bin/bash
# Builds the tiles wasm artifact.
#
# The contract is self-contained: there is no vending factory and no vending minter to
# fetch any more. A Stargaze 2.0 collection on the Cosmos Hub is a plain CW721 contract,
# and this one mints its own tiles (ADR 0002, ADR 0003).
set -e

source scripts/00_load_constants.sh

mkdir -p scripts/state
CURRENT_STATE_FILE="scripts/state/01_build_contracts.state"
touch "$CURRENT_STATE_FILE"

BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m'

check_step() {
    grep -q "^$1=done$" "$CURRENT_STATE_FILE"
}

mark_step_done() {
    echo "$1=done" >> "$CURRENT_STATE_FILE"
}

mkdir -p artifacts

if ! check_step "tile"; then
    echo -e "${BLUE}1. Building the tiles contract...${NC}"

    if cargo build --release --target wasm32-unknown-unknown; then
        cp target/wasm32-unknown-unknown/release/tiles.wasm artifacts/
        echo -e "${GREEN}✅ Contract built${NC}"
        mark_step_done "tile"
    else
        echo -e "${RED}❌ Build failed${NC}"
        exit 1
    fi
else
    echo -e "${YELLOW}Skipping build (already done)${NC}"
fi

if ! check_step "artifact"; then
    echo -e "${BLUE}2. Checking artifacts/tiles.wasm...${NC}"

    if [ ! -f artifacts/tiles.wasm ]; then
        echo -e "${RED}❌ artifacts/tiles.wasm is missing${NC}"
        exit 1
    fi

    SIZE=$(stat -f%z artifacts/tiles.wasm 2>/dev/null || stat -c%s artifacts/tiles.wasm)
    echo -e "   raw wasm size: ${SIZE} bytes"
    echo -e "${YELLOW}   Optimise before uploading to a live chain:${NC}"
    echo -e "   docker run --rm -v \"\$(pwd)\":/code \\"
    echo -e "     --mount type=volume,source=tiles_cache,target=/code/target \\"
    echo -e "     --mount type=volume,source=registry_cache,target=/usr/local/cargo/registry \\"
    echo -e "     cosmwasm/rust-optimizer:0.16.1"

    mark_step_done "artifact"
fi
