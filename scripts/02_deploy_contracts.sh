#!/bin/bash
# Stores and instantiates the tiles collection on the Cosmos Hub.
#
# Target: Stargaze 2.0 on the Cosmos Hub (`gaiad`, gas in uatom). Override CHAIN_ID,
# NODE_URL and GAS_PRICE in scripts/messages/constants.json for a testnet run.
#
# There is no factory and no minter contract to wire up: the collection is instantiated
# directly and mints its own tiles. That is the CW721 shape Stargaze 2.0 documents.
set -e

source scripts/00_load_constants.sh

if [ -z "$DEPLOYER_ADDRESS" ]; then
    echo -e "\033[0;31mDEPLOYER_ADDRESS is not set (fill scripts/messages/constants.json)\033[0m"
    exit 1
fi

if [ ! -f artifacts/tiles.wasm ]; then
    echo -e "\033[0;31martifacts/tiles.wasm is missing: run scripts/01_build_contracts.sh first\033[0m"
    exit 1
fi

mkdir -p scripts/state
CURRENT_STATE_FILE="scripts/state/02_deploy_contracts.state"
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

# Waits for a broadcast transaction and prints its raw log.
wait_for_tx() {
    local txhash=$1
    echo -e "${BLUE}   tx: $txhash${NC}"
    sleep 8
    gaia_wait_tx "$txhash"
}

# `gaiad` flags shared by every transaction. Gas is paid in uatom: this is the gas
# denom of the Cosmos Hub and it is a separate flow from the pixel payment (ADR 0005).
gaia_tx() {
    gaiad tx "$@" \
        --from "$DEPLOYER_ADDRESS" \
        --keyring-backend "$KEYRING_BACKEND" \
        --gas-prices "${GAS_PRICE}${NATIVE_DENOM}" \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --gas auto \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --broadcast-mode "$BROADCAST_MODE" \
        -y --output json
}

# ============================================================================
# 1. Store the contract code
# ============================================================================
if ! check_step "store_tile"; then
    echo -e "${BLUE}1. Storing the tiles contract...${NC}"

    STORE_TX=$(gaia_tx wasm store artifacts/tiles.wasm)
    STORE_TXHASH=$(echo "$STORE_TX" | jq -r '.txhash')
    echo -e "${BLUE}   tx: $STORE_TXHASH${NC}"

    sleep 10
    STORE_RESULT=$(gaiad query tx "$STORE_TXHASH" --output json --node "$NODE_URL")
    TILE_CODE_ID=$(echo "$STORE_RESULT" | jq -r '[.events[] | select(.type=="store_code") | .attributes[] | select(.key=="code_id") | .value] | first')

    if [ -z "$TILE_CODE_ID" ] || [ "$TILE_CODE_ID" = "null" ]; then
        echo -e "${RED}❌ Store failed${NC}"
        echo "$STORE_RESULT" | jq .
        exit 1
    fi

    echo "tile_code_id=$TILE_CODE_ID" >> "$CURRENT_STATE_FILE"
    echo "tile_store_txhash=$STORE_TXHASH" >> "$CURRENT_STATE_FILE"
    mark_step_done "store_tile"
    echo -e "${GREEN}✅ Stored with code id $TILE_CODE_ID${NC}"
fi

if [ -z "$TILE_CODE_ID" ]; then
    TILE_CODE_ID=$(grep "^tile_code_id=" "$CURRENT_STATE_FILE" | cut -d'=' -f2)
fi

# ============================================================================
# 2. Instantiate the collection
# ============================================================================
if ! check_step "instantiate_tile"; then
    echo -e "${BLUE}2. Instantiating the collection...${NC}"

    # Built with jq so the message follows the CW721 0.22 collection extension shape:
    # royalties live in `collection_info_extension.royalty_info`, capped at 10 %.
    # `share` is a `Decimal`, which is string-encoded on the wire, so it is passed as
    # `--arg` (a JSON string) and not `--argjson` (a JSON number).
    MSG=$(jq -n \
        --arg creator "$DEPLOYER_ADDRESS" \
        --arg name "$COLLECTION_NAME" \
        --arg symbol "$COLLECTION_SYMBOL" \
        --arg description "$COLLECTION_DESCRIPTION" \
        --arg image "$COLLECTION_URI" \
        --arg share "$ROYALTY_SHARE" \
        '{
            name: $name,
            symbol: $symbol,
            minter: null,
            creator: $creator,
            withdraw_address: null,
            collection_info_extension: {
                description: $description,
                image: (if $image == "" then null else $image end),
                external_link: null,
                royalty_info: {
                    payment_address: $creator,
                    share: $share
                }
            }
        }')

    echo "   instantiate message:"
    echo "$MSG" | jq .

    INIT_TX=$(gaia_tx wasm instantiate "$TILE_CODE_ID" "$MSG" \
        --label "$COLLECTION_NAME" \
        --admin "$DEPLOYER_ADDRESS")
    INIT_TXHASH=$(echo "$INIT_TX" | jq -r '.txhash')
    echo -e "${BLUE}   tx: $INIT_TXHASH${NC}"

    sleep 10
    INIT_RESULT=$(gaiad query tx "$INIT_TXHASH" --output json --node "$NODE_URL")
    TILE_CONTRACT=$(echo "$INIT_RESULT" | jq -r '[.events[] | select(.type=="instantiate") | .attributes[] | select(.key=="_contract_address") | .value] | first')

    if [ -z "$TILE_CONTRACT" ] || [ "$TILE_CONTRACT" = "null" ]; then
        echo -e "${RED}❌ Instantiate failed${NC}"
        echo "$INIT_RESULT" | jq .
        exit 1
    fi

    echo "tile_contract=$TILE_CONTRACT" >> "$CURRENT_STATE_FILE"
    echo "tile_instantiate_txhash=$INIT_TXHASH" >> "$CURRENT_STATE_FILE"
    mark_step_done "instantiate_tile"
    echo -e "${GREEN}✅ Collection live at $TILE_CONTRACT${NC}"
fi
