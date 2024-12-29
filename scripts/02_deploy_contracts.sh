#!/bin/bash
set -e

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
mkdir -p scripts/cache
mkdir -p scripts/state

# State file
STATE_FILE="scripts/state/02_deploy_contracts.state"
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
    echo -e "${BLUE}Deploy Status:${NC}"
    echo -e "1. Store Factory: $(check_step "store_factory" && echo -e "${GREEN}✓${NC}" || echo -e "${RED}✗${NC}")"
    echo -e "2. Store Tile: $(check_step "store_tile" && echo -e "${GREEN}✓${NC}" || echo -e "${RED}✗${NC}")"
    echo -e "3. Store Minter: $(check_step "store_minter" && echo -e "${GREEN}✓${NC}" || echo -e "${RED}✗${NC}")"
    echo -e "4. Instantiate Factory: $(check_step "instantiate_factory" && echo -e "${GREEN}✓${NC}" || echo -e "${RED}✗${NC}")"
    echo
}

# Function to validate build artifacts
validate_artifacts() {
    local missing=false
    
    if [ ! -f "artifacts/factory.wasm" ]; then
        echo -e "${RED}❌ Factory wasm not found. Run build script first.${NC}"
        missing=true
    fi
    if [ ! -f "artifacts/tiles.wasm" ]; then
        echo -e "${RED}❌ Tile wasm not found. Run build script first.${NC}"
        missing=true
    fi
    if [ ! -f "artifacts/vending_minter.wasm" ]; then
        echo -e "${RED}❌ Minter wasm not found. Run build script first.${NC}"
        missing=true
    fi
    
    $missing && exit 1
    return 0
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
            echo "Usage: $0 [--start-step <1-4>] [--force]"
            echo "  --start-step: Start from specific step"
            echo "    1: Store Factory Contract"
            echo "    2: Store Tile Contract"
            echo "    3: Store Minter Contract"
            echo "    4: Instantiate Factory"
            echo "  --force: Force redeploy even if step was completed"
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

# Validate build artifacts
validate_artifacts

# Save transaction hash and code ID
save_tx_info() {
    local step=$1
    local txhash=$2
    local code_id=$3
    echo "${step}_txhash=$txhash" >> "$STATE_FILE"
    if [ ! -z "$code_id" ]; then
        echo "${step}_code_id=$code_id" >> "$STATE_FILE"
    fi
}

# Store factory contract
if [[ $START_STEP -le 1 ]] && { $FORCE || ! check_step "store_factory"; }; then
    echo -e "${BLUE}1. Storing Factory Contract...${NC}"
    FACTORY_TX=$(starsd tx wasm store artifacts/factory.wasm \
        --from "$DEPLOYER_ADDRESS" \
        --keyring-backend test \
        --gas-prices 0.025ustars \
        --gas-adjustment 1.7 \
        --gas auto \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --broadcast-mode sync \
        -y --output json)

    FACTORY_TXHASH=$(echo "$FACTORY_TX" | jq -r '.txhash')
    echo -e "${BLUE}Transaction hash: ${FACTORY_TXHASH}${NC}"

    echo -e "${BLUE}Waiting for transaction...${NC}"
    sleep 10

    FACTORY_TX_RESULT=$(starsd query tx "$FACTORY_TXHASH" --output json --node "$NODE_URL")
    FACTORY_CODE_ID=$(echo "$FACTORY_TX_RESULT" | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')

    if [ -z "$FACTORY_CODE_ID" ]; then
        echo -e "${RED}❌ Factory store failed${NC}"
        exit 1
    fi

    save_tx_info "store_factory" "$FACTORY_TXHASH" "$FACTORY_CODE_ID"
    mark_step_done "store_factory"
    echo -e "${GREEN}✅ Factory stored with code ID: ${FACTORY_CODE_ID}${NC}"
    echo "{\"code_id\": \"$FACTORY_CODE_ID\", \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"}" > scripts/cache/factory.json
else
    echo -e "${YELLOW}Skipping factory store (already completed)${NC}"
    FACTORY_CODE_ID=$(jq -r '.code_id' scripts/cache/factory.json)
fi

# Store tile contract
if [[ $START_STEP -le 2 ]] && { $FORCE || ! check_step "store_tile"; }; then
    echo -e "${BLUE}2. Storing Tile Contract...${NC}"
    TILE_TX=$(starsd tx wasm store artifacts/tiles.wasm \
        --from "$DEPLOYER_ADDRESS" \
        --keyring-backend test \
        --gas-prices 0.025ustars \
        --gas-adjustment 1.7 \
        --gas auto \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --broadcast-mode sync \
        -y --output json)

    TILE_TXHASH=$(echo "$TILE_TX" | jq -r '.txhash')
    echo -e "${BLUE}Transaction hash: ${TILE_TXHASH}${NC}"

    echo -e "${BLUE}Waiting for transaction...${NC}"
    sleep 10

    TILE_TX_RESULT=$(starsd query tx "$TILE_TXHASH" --output json --node "$NODE_URL")
    TILE_CODE_ID=$(echo "$TILE_TX_RESULT" | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')

    if [ -z "$TILE_CODE_ID" ]; then
        echo -e "${RED}❌ Tile store failed${NC}"
        exit 1
    fi

    save_tx_info "store_tile" "$TILE_TXHASH" "$TILE_CODE_ID"
    mark_step_done "store_tile"
    echo -e "${GREEN}✅ Tile stored with code ID: ${TILE_CODE_ID}${NC}"
    echo "{\"code_id\": \"$TILE_CODE_ID\", \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"}" > scripts/cache/tile.json
else
    echo -e "${YELLOW}Skipping tile store (already completed)${NC}"
    TILE_CODE_ID=$(jq -r '.code_id' scripts/cache/tile.json)
fi

# Store minter contract
if [[ $START_STEP -le 3 ]] && { $FORCE || ! check_step "store_minter"; }; then
    echo -e "${BLUE}3. Storing Minter Contract...${NC}"
    MINTER_TX=$(starsd tx wasm store artifacts/vending_minter.wasm \
        --from "$DEPLOYER_ADDRESS" \
        --keyring-backend test \
        --gas-prices 0.025ustars \
        --gas-adjustment 1.7 \
        --gas auto \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --broadcast-mode sync \
        -y --output json)

    MINTER_TXHASH=$(echo "$MINTER_TX" | jq -r '.txhash')
    echo -e "${BLUE}Transaction hash: ${MINTER_TXHASH}${NC}"

    echo -e "${BLUE}Waiting for transaction...${NC}"
    sleep 10

    MINTER_TX_RESULT=$(starsd query tx "$MINTER_TXHASH" --output json --node "$NODE_URL")
    MINTER_CODE_ID=$(echo "$MINTER_TX_RESULT" | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')

    if [ -z "$MINTER_CODE_ID" ]; then
        echo -e "${RED}❌ Minter store failed${NC}"
        exit 1
    fi

    save_tx_info "store_minter" "$MINTER_TXHASH" "$MINTER_CODE_ID"
    mark_step_done "store_minter"
    echo -e "${GREEN}✅ Minter stored with code ID: ${MINTER_CODE_ID}${NC}"
    echo "{\"code_id\": \"$MINTER_CODE_ID\", \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"}" > scripts/cache/minter.json
else
    echo -e "${YELLOW}Skipping minter store (already completed)${NC}"
    MINTER_CODE_ID=$(jq -r '.code_id' scripts/cache/minter.json)
fi

# Instantiate factory with code IDs
if [[ $START_STEP -le 4 ]] && { $FORCE || ! check_step "instantiate_factory"; }; then
    echo -e "${BLUE}4. Instantiating Factory Contract...${NC}"

    MSG='{
      "params": {
        "code_id": '$MINTER_CODE_ID',
        "allowed_sg721_code_ids": ['$TILE_CODE_ID'],
        "frozen": false,
        "creation_fee": {"amount": "1000000", "denom": "ustars"},
        "min_mint_price": {"amount": "0", "denom": "ustars"},
        "mint_fee_bps": 1000,
        "max_trading_offset_secs": 604800,
        "extension": {
            "max_token_limit": 9,
            "max_per_address_limit": 3,
            "airdrop_mint_price": { "denom": "ustars", "amount": "0" },
            "airdrop_mint_fee_bps": 0,
            "shuffle_fee": { "amount": "0", "denom": "ustars" }
        }
      }
    }'

    echo "Instantiate message:"
    echo "$MSG" | jq .

    INIT_TX=$(starsd tx wasm instantiate "$FACTORY_CODE_ID" "$MSG" \
        --label "Tiles Factory" \
        --no-admin \
        --from "$DEPLOYER_ADDRESS" \
        --keyring-backend test \
        --gas-prices 0.025ustars \
        --gas-adjustment 1.7 \
        --gas auto \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --broadcast-mode sync \
        -y --output json)

    INIT_TXHASH=$(echo "$INIT_TX" | jq -r '.txhash')
    echo -e "${BLUE}Transaction hash: ${INIT_TXHASH}${NC}"

    echo -e "${BLUE}Waiting for transaction...${NC}"
    sleep 10

    INIT_TX_RESULT=$(starsd query tx "$INIT_TXHASH" --output json --node "$NODE_URL")
    FACTORY_ADDRESS=$(echo "$INIT_TX_RESULT" | jq -r '.logs[0].events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')

    if [ -z "$FACTORY_ADDRESS" ]; then
        echo -e "${RED}❌ Factory instantiation failed${NC}"
        exit 1
    fi

    echo -e "${GREEN}✅ Factory instantiated at: ${FACTORY_ADDRESS}${NC}"
    echo "{\"address\": \"$FACTORY_ADDRESS\", \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"}" > scripts/cache/factory_contract.json
    mark_step_done "instantiate_factory"
else
    echo -e "${YELLOW}Skipping factory instantiation (already completed)${NC}"
    FACTORY_ADDRESS=$(jq -r '.address' scripts/cache/factory_contract.json)
fi

# Show final status
echo
show_status

if check_step "store_factory" && check_step "store_tile" && check_step "store_minter" && check_step "instantiate_factory"; then
    echo -e "${GREEN}✅ All contracts deployed successfully!${NC}"
    echo -e "${BLUE}Summary:${NC}"
    echo -e "Factory Code ID: ${FACTORY_CODE_ID}"
    echo -e "Tile Code ID: ${TILE_CODE_ID}"
    echo -e "Minter Code ID: ${MINTER_CODE_ID}"
    echo -e "Factory Address: ${FACTORY_ADDRESS}"
else
    echo -e "${YELLOW}⚠️  Some steps are incomplete. Run again to complete remaining steps.${NC}"
fi 