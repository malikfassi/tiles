#!/bin/bash
set -e

# Load constants
source scripts/00_load_constants.sh

# Create state directory
mkdir -p scripts/state

# State files
CURRENT_STATE_FILE="scripts/state/03_create_minter.state"
PREV_STATE_FILE="scripts/state/02_deploy_contracts.state"
touch "$CURRENT_STATE_FILE"

# Load previous state
if [ ! -f "$PREV_STATE_FILE" ]; then
    echo -e "\033[0;31mPrevious state file not found: $PREV_STATE_FILE\033[0m"
    exit 1
fi

# Load required values from previous state
FACTORY_CODE_ID=$(grep "^factory_code_id=" "$PREV_STATE_FILE" | cut -d'=' -f2)
FACTORY_CONTRACT=$(grep "^factory_contract=" "$PREV_STATE_FILE" | cut -d'=' -f2)
MINTER_CODE_ID=$(grep "^minter_code_id=" "$PREV_STATE_FILE" | cut -d'=' -f2)
TILE_CODE_ID=$(grep "^tile_code_id=" "$PREV_STATE_FILE" | cut -d'=' -f2)

if [ -z "$FACTORY_CODE_ID" ] || [ -z "$FACTORY_CONTRACT" ] || [ -z "$MINTER_CODE_ID" ] || [ -z "$TILE_CODE_ID" ]; then
    echo -e "\033[0;31mMissing required values from previous state\033[0m"
    exit 1
fi

# Function to check if a step is completed
check_step() {
    grep -q "^$1=done$" "$CURRENT_STATE_FILE"
    return $?
}

# Function to mark step as completed
mark_step_done() {
    echo "$1=done" >> "$CURRENT_STATE_FILE"
}

# Save transaction hash and contract addresses
save_tx_info() {
    local step=$1
    local txhash=$2
    local contract_addr=$3
    echo "${step}_txhash=$txhash" >> "$CURRENT_STATE_FILE"
    if [ ! -z "$contract_addr" ]; then
        echo "${step}_contract=$contract_addr" >> "$CURRENT_STATE_FILE"
    fi
}

# Set start time to next minute
TIME=$(date +%s)
TIME=$((TIME + 60))

# Create minter
if ! check_step "create_minter"; then
    echo -e "\033[0;34mCreating minter...\033[0m"
    
    MSG='{
      "create_minter": {
        "init_msg": {
          "base_token_uri": "'$BASE_TOKEN_URI'",
          "mint_price": {
            "amount": "'$MINT_PRICE'",
            "denom": "'$TOKEN_DENOM'"
          },
          "num_tokens": '$MAX_TOKEN_LIMIT',
          "payment_address": "'$DEPLOYER_ADDRESS'",
          "per_address_limit": '$MAX_PER_ADDRESS_LIMIT',
          "start_time": "'$TIME'000000000",
          "whitelist": null
        },
        "collection_params": {
          "code_id": '$TILE_CODE_ID',
          "name": "'$COLLECTION_NAME'",
          "symbol": "'$COLLECTION_SYMBOL'",
          "info": {
            "creator": "'$DEPLOYER_ADDRESS'",
            "description": "'"$COLLECTION_DESCRIPTION"'",
            "image": "'$COLLECTION_URI'",
            "external_link": null,
            "explicit_content": false,
            "royalty_info": {
              "payment_address": "'$DEPLOYER_ADDRESS'",
              "share": "0.'$DEFAULT_ROYALTY_SHARE'"
            }
          }
        }
      }
    }'

    echo "Execute message:"
    echo "$MSG" | jq .

    MINTER_TX=$(starsd tx wasm execute "$FACTORY_CONTRACT" "$MSG" \
        --amount "${CREATION_FEE}${TOKEN_DENOM}" \
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
    
    # Save all info to state
    save_tx_info "create_minter" "$MINTER_TXHASH" "$MINTER_ADDR"
    mark_step_done "create_minter"
    
    echo -e "\033[0;32m✅ Minter created at: $MINTER_ADDR\033[0m"
    
    # Query and save sg721 address
    sleep 5
    SG721_ADDR=$(starsd query wasm contract-state smart "$MINTER_ADDR" '{"collection_info":{}}' --node "$NODE_URL" --output json | jq -r .sg721_address)
    if [ ! -z "$SG721_ADDR" ]; then
        save_tx_info "sg721_contract" "$MINTER_TXHASH" "$SG721_ADDR"
        echo -e "\033[0;32m✅ SG721 contract at: $SG721_ADDR\033[0m"
    fi
fi 