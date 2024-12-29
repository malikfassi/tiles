#!/bin/bash
set -e

# Load constants
source scripts/00_load_constants.sh

# Validate required constants
if [ -z "$DEPLOYER_ADDRESS" ]; then
    echo -e "\033[0;31mDEPLOYER_ADDRESS is not set\033[0m"
    exit 1
fi

# Create state directory
mkdir -p scripts/state

# State files
CURRENT_STATE_FILE="scripts/state/02_deploy_contracts.state"
PREV_STATE_FILE="scripts/state/01_build_contracts.state"
touch "$CURRENT_STATE_FILE"

# Load previous state
if [ ! -f "$PREV_STATE_FILE" ]; then
    echo -e "\033[0;31mPrevious state file not found: $PREV_STATE_FILE\033[0m"
    exit 1
fi

# Check if previous steps are completed
if ! grep -q "^tile=done$" "$PREV_STATE_FILE" || \
   ! grep -q "^minter=done$" "$PREV_STATE_FILE" || \
   ! grep -q "^factory=done$" "$PREV_STATE_FILE"; then
    echo -e "\033[0;31mPrevious steps not completed. Please run 01_build_contracts.sh first\033[0m"
    exit 1
fi

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

# Save code id and transaction hash
save_code_info() {
    local step=$1
    local code_id=$2
    local txhash=$3
    echo "${step}_code_id=$code_id" >> "$CURRENT_STATE_FILE"
    echo "${step}_txhash=$txhash" >> "$CURRENT_STATE_FILE"
}

# Save contract info
save_contract_info() {
    local step=$1
    local contract=$2
    local txhash=$3
    echo "${step}_contract=$contract" >> "$CURRENT_STATE_FILE"
    echo "${step}_txhash=$txhash" >> "$CURRENT_STATE_FILE"
}

# Store tile contract
if ! check_step "store_tile"; then
    echo -e "${BLUE}Storing Tile Contract...${NC}"
    
    TILE_TX=$(starsd tx wasm store artifacts/tiles.wasm \
        --from "$DEPLOYER_ADDRESS" \
        --keyring-backend test \
        --gas-prices "$GAS_PRICE"ustars \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --gas auto \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --broadcast-mode "$BROADCAST_MODE" \
        -y --output json)
    
    TILE_TXHASH=$(echo "$TILE_TX" | jq -r '.txhash')
    echo -e "${BLUE}Transaction hash: $TILE_TXHASH${NC}"
    
    echo -e "${BLUE}Waiting for transaction...${NC}"
    sleep 10
    
    TILE_TX_RESULT=$(starsd query tx "$TILE_TXHASH" --output json --node "$NODE_URL")
    TILE_CODE_ID=$(echo "$TILE_TX_RESULT" | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
    
    if [ -z "$TILE_CODE_ID" ]; then
        echo -e "${RED}❌ Tile contract store failed${NC}"
        exit 1
    fi
    
    save_code_info "tile" "$TILE_CODE_ID" "$TILE_TXHASH"
    mark_step_done "store_tile"
    echo -e "${GREEN}✅ Tile contract stored with code ID: $TILE_CODE_ID${NC}"
fi

# Store minter contract
if ! check_step "store_minter"; then
    echo -e "${BLUE}Storing Minter Contract...${NC}"
    
    MINTER_TX=$(starsd tx wasm store artifacts/vending_minter.wasm \
        --from "$DEPLOYER_ADDRESS" \
        --keyring-backend test \
        --gas-prices "$GAS_PRICE"ustars \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --gas auto \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --broadcast-mode "$BROADCAST_MODE" \
        -y --output json)
    
    MINTER_TXHASH=$(echo "$MINTER_TX" | jq -r '.txhash')
    echo -e "${BLUE}Transaction hash: $MINTER_TXHASH${NC}"
    
    echo -e "${BLUE}Waiting for transaction...${NC}"
    sleep 10
    
    MINTER_TX_RESULT=$(starsd query tx "$MINTER_TXHASH" --output json --node "$NODE_URL")
    MINTER_CODE_ID=$(echo "$MINTER_TX_RESULT" | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
    
    if [ -z "$MINTER_CODE_ID" ]; then
        echo -e "${RED}❌ Minter contract store failed${NC}"
        exit 1
    fi
    
    save_code_info "minter" "$MINTER_CODE_ID" "$MINTER_TXHASH"
    mark_step_done "store_minter"
    echo -e "${GREEN}✅ Minter contract stored with code ID: $MINTER_CODE_ID${NC}"
fi

# Store factory contract
if ! check_step "store_factory"; then
    echo -e "${BLUE}Storing Factory Contract...${NC}"
    
    FACTORY_TX=$(starsd tx wasm store artifacts/vending_factory.wasm \
        --from "$DEPLOYER_ADDRESS" \
        --keyring-backend test \
        --gas-prices "$GAS_PRICE"ustars \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --gas auto \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --broadcast-mode "$BROADCAST_MODE" \
        -y --output json)
    
    FACTORY_TXHASH=$(echo "$FACTORY_TX" | jq -r '.txhash')
    echo -e "${BLUE}Transaction hash: $FACTORY_TXHASH${NC}"
    
    echo -e "${BLUE}Waiting for transaction...${NC}"
    sleep 10
    
    FACTORY_TX_RESULT=$(starsd query tx "$FACTORY_TXHASH" --output json --node "$NODE_URL")
    FACTORY_CODE_ID=$(echo "$FACTORY_TX_RESULT" | jq -r '.logs[0].events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value')
    
    if [ -z "$FACTORY_CODE_ID" ]; then
        echo -e "${RED}❌ Factory contract store failed${NC}"
        exit 1
    fi
    
    save_code_info "factory" "$FACTORY_CODE_ID" "$FACTORY_TXHASH"
    mark_step_done "store_factory"
    echo -e "${GREEN}✅ Factory contract stored with code ID: $FACTORY_CODE_ID${NC}"
fi

# Load code IDs from state if not already set
if [ -z "$TILE_CODE_ID" ]; then
    TILE_CODE_ID=$(grep "^tile_code_id=" "$CURRENT_STATE_FILE" | cut -d'=' -f2)
fi
if [ -z "$MINTER_CODE_ID" ]; then
    MINTER_CODE_ID=$(grep "^minter_code_id=" "$CURRENT_STATE_FILE" | cut -d'=' -f2)
fi
if [ -z "$FACTORY_CODE_ID" ]; then
    FACTORY_CODE_ID=$(grep "^factory_code_id=" "$CURRENT_STATE_FILE" | cut -d'=' -f2)
fi

# Initialize factory
if ! check_step "init_factory"; then
    echo -e "${BLUE}Initializing Factory Contract...${NC}"
    
    MSG='{
      "params": {
        "code_id": '$MINTER_CODE_ID',
        "allowed_sg721_code_ids": ['$TILE_CODE_ID'],
        "frozen": false,
        "creation_fee": {"amount": "'$CREATION_FEE'", "denom": "'$TOKEN_DENOM'"},
        "min_mint_price": {"amount": "'$MIN_MINT_PRICE'", "denom": "'$TOKEN_DENOM'"},
        "mint_fee_bps": '$MINT_FEE_BPS',
        "max_trading_offset_secs": '$MAX_TRADING_OFFSET_SECS',
        "extension": {
            "max_token_limit": '$MAX_TOKEN_LIMIT',
            "max_per_address_limit": '$MAX_PER_ADDRESS_LIMIT',
            "airdrop_mint_price": { "denom": "'$TOKEN_DENOM'", "amount": "'$AIRDROP_MINT_PRICE'" },
            "airdrop_mint_fee_bps": '$AIRDROP_MINT_FEE_BPS',
            "shuffle_fee": { "amount": "'$SHUFFLE_FEE'", "denom": "'$TOKEN_DENOM'" }
        }
      }
    }'
    
    echo "Initialize message:"
    echo "$MSG" | jq .
    
    INIT_TX=$(starsd tx wasm instantiate "$FACTORY_CODE_ID" "$MSG" \
        --label "Tiles Factory" \
        --no-admin \
        --from "$DEPLOYER_ADDRESS" \
        --keyring-backend test \
        --gas-prices "$GAS_PRICE"ustars \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --gas auto \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --broadcast-mode "$BROADCAST_MODE" \
        -y --output json)
    
    INIT_TXHASH=$(echo "$INIT_TX" | jq -r '.txhash')
    echo -e "${BLUE}Transaction hash: $INIT_TXHASH${NC}"
    
    echo -e "${BLUE}Waiting for transaction...${NC}"
    sleep 10
    
    INIT_TX_RESULT=$(starsd query tx "$INIT_TXHASH" --output json --node "$NODE_URL")
    FACTORY_CONTRACT=$(echo "$INIT_TX_RESULT" | jq -r '.logs[0].events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value')
    
    if [ -z "$FACTORY_CONTRACT" ]; then
        echo -e "${RED}❌ Factory initialization failed${NC}"
        exit 1
    fi
    
    save_contract_info "factory" "$FACTORY_CONTRACT" "$INIT_TXHASH"
    mark_step_done "init_factory"
    echo -e "${GREEN}✅ Factory initialized at: $FACTORY_CONTRACT${NC}"
fi 