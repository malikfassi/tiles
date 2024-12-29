#!/bin/bash
set -e

# Load constants
source scripts/00_load_constants.sh

# Create directories
mkdir -p scripts/cache
mkdir -p scripts/state

# State file
STATE_FILE="scripts/state/03_create_minter.state"
touch "$STATE_FILE"

# Save transaction hash and contract addresses
save_tx_info() {
    local step=$1
    local txhash=$2
    local contract_addr=$3
    echo "${step}_txhash=$txhash" >> "$STATE_FILE"
    if [ ! -z "$contract_addr" ]; then
        echo "${step}_contract=$contract_addr" >> "$STATE_FILE"
    fi
}

# Set current time plus 24 hours for start time
TIME=$(date +%s)
TIME=$((TIME + 86400))

# Create minter
if ! check_step "create_minter"; then
    echo -e "\033[0;34mCreating minter...\033[0m"
    
    MINTER_TX=$(starsd tx wasm instantiate "$MINTER_CODE_ID" \
        "$(cat scripts/messages/create_minter.json | jq --arg time "$TIME" '.start_time = ($time + "000000000")' -c)" \
        --label "Tiles Minter" \
        --admin "$DEPLOYER_ADDRESS" \
        --from "$DEPLOYER_ADDRESS" \
        --keyring-backend test \
        --gas-prices "$GAS_PRICE"ustars \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --gas auto \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --broadcast-mode "$BROADCAST_MODE" \
        -y \
        --output json)
    
    MINTER_TXHASH=$(echo "$MINTER_TX" | jq -r .txhash)
    echo -e "\033[0;34mTransaction hash: $MINTER_TXHASH\033[0m"
    
    echo -e "\033[0;34mWaiting for transaction...\033[0m"
    sleep 10
    
    MINTER_TX_RESULT=$(starsd query tx "$MINTER_TXHASH" --output json --node "$NODE_URL")
    MINTER_ADDR=$(echo "$MINTER_TX_RESULT" | jq -r '.logs[0].events[] | select(.type=="wasm") | .attributes[] | select(.key=="_contract_address") | .value')
    
    if [ -z "$MINTER_ADDR" ]; then
        echo -e "\033[0;31m❌ Minter creation failed\033[0m"
        exit 1
    fi
    
    save_tx_info "create_minter" "$MINTER_TXHASH" "$MINTER_ADDR"
    mark_step_done "create_minter"
    
    # Save minter address
    echo "{\"address\": \"$MINTER_ADDR\", \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"}" > scripts/cache/minter_contract.json
    echo -e "\033[0;32m✅ Minter created at: $MINTER_ADDR\033[0m"
    
    # Query and save sg721 address
    sleep 5
    SG721_ADDR=$(starsd query wasm contract-state smart "$MINTER_ADDR" '{"collection_info":{}}' --node "$NODE_URL" --output json | jq -r .sg721_address)
    if [ ! -z "$SG721_ADDR" ]; then
        echo "{\"address\": \"$SG721_ADDR\", \"timestamp\": \"$(date -u +"%Y-%m-%dT%H:%M:%SZ")\"}" > scripts/cache/sg721_contract.json
        save_tx_info "sg721_contract" "$MINTER_TXHASH" "$SG721_ADDR"
        echo -e "\033[0;32m✅ SG721 contract at: $SG721_ADDR\033[0m"
    fi
fi 