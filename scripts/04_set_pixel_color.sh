#!/bin/bash
# Colours pixels on a tile, paying the advertised price (ADR 0004).
#
# The quote comes from the contract itself (`QuotePixelUpdates`), so this script never
# invents a price: it asks, then pays exactly that. Paying anything else is refused.
#
# Usage:
#   scripts/04_set_pixel_color.sh <token_id> <pixel_id> <#RRGGBB> <duration_seconds>
#
# Example:
#   scripts/04_set_pixel_color.sh 1 42 "#FF0000" 3600
set -e

source scripts/00_load_constants.sh

if [ $# -lt 4 ]; then
    echo "Usage: $0 <token_id> <pixel_id> <#RRGGBB> <duration_seconds>"
    exit 1
fi

TOKEN_ID_ARG=$1
PIXEL_ID=$2
COLOR=$3
DURATION=$4

STATE_FILE="scripts/state/02_deploy_contracts.state"
if [ ! -f "$STATE_FILE" ]; then
    echo -e "\033[0;31mNo deployment state found: run scripts/02_deploy_contracts.sh first\033[0m"
    exit 1
fi

TILE_CONTRACT=$(grep "^tile_contract=" "$STATE_FILE" | cut -d'=' -f2)

# 1. Read the current metadata: the contract uses it as an optimistic lock, so a stale
#    copy is refused rather than silently overwriting someone else's pixel.
echo -e "\033[0;34m1. Reading the current tile state...${NC}"
CURRENT_METADATA=$(gaiad query wasm contract-state smart "$TILE_CONTRACT" \
    "{\"extension\":{\"msg\":{\"tile_pixels\":{\"token_id\":\"$TOKEN_ID_ARG\"}}}}" \
    --node "$NODE_URL" --output json | jq -c '.data')

if [ -z "$CURRENT_METADATA" ] || [ "$CURRENT_METADATA" = "null" ]; then
    echo -e "\033[0;31mCould not read the tile metadata\033[0m"
    exit 1
fi

# 2. Ask the contract what the write costs. The contract is the only source of truth
#    for the price, and the same computation runs again when the message is executed.
echo -e "\033[0;34m2. Quoting the price...${NC}"
UPDATES=$(jq -n \
    --argjson id "$PIXEL_ID" \
    --arg color "$COLOR" \
    --argjson duration "$DURATION" \
    '[{id: $id, color: $color, expiration_duration: $duration}]')

QUOTE=$(gaiad query wasm contract-state smart "$TILE_CONTRACT" \
    "{\"extension\":{\"msg\":{\"quote_pixel_updates\":{\"token_id\":\"$TOKEN_ID_ARG\",\"updates\":$UPDATES}}}}" \
    --node "$NODE_URL" --output json | jq -c '.data')

TOTAL=$(echo "$QUOTE" | jq -r '.total')
echo "   quote:"
echo "$QUOTE" | jq .
echo -e "   total: ${TOTAL}${NATIVE_DENOM}"

# 3. Pay exactly that, in the payment denom decided in ADR 0005.
echo -e "\033[0;34m3. Colouring pixel $PIXEL_ID as $COLOR for ${DURATION}s...${NC}"
MSG=$(jq -n \
    --arg token_id "$TOKEN_ID_ARG" \
    --argjson metadata "$CURRENT_METADATA" \
    --argjson updates "$UPDATES" \
    '{
        update_extension: {
            msg: {
                set_pixel_color: {
                    token_id: $token_id,
                    current_metadata: $metadata,
                    updates: $updates
                }
            }
        }
    }')

TX_RESULT=$(gaiad tx wasm execute "$TILE_CONTRACT" "$MSG" \
    --amount "${TOTAL}${NATIVE_DENOM}" \
    --from "$DEPLOYER_ADDRESS" \
    --keyring-backend "$KEYRING_BACKEND" \
    --gas-prices "${GAS_PRICE}${NATIVE_DENOM}" \
    --gas-adjustment "$GAS_ADJUSTMENT" \
    --gas auto \
    --chain-id "$CHAIN_ID" \
    --node "$NODE_URL" \
    --broadcast-mode "$BROADCAST_MODE" \
    -y --output json)

TX_HASH=$(echo "$TX_RESULT" | jq -r '.txhash')
echo -e "\033[0;32m✅ Broadcast: $TX_HASH\033[0m"
echo -e "   Verify with: scripts/query_tx.sh $TX_HASH"
