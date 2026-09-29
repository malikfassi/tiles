#!/bin/bash
# Mints a tile on the deployed collection.
#
# There is no vending minter any more: the collection mints its own tiles, so this sends
# the standard CW721 `Mint` message. Only the configured minter may call it, which is the
# contract itself for tiles created through the collection's own flow; the deployment
# script keeps the deployer as admin so an operator can seed the first tile.
set -e

source scripts/00_load_constants.sh

STATE_FILE="scripts/state/02_deploy_contracts.state"
if [ ! -f "$STATE_FILE" ]; then
    echo -e "\033[0;31mNo deployment state found: run scripts/02_deploy_contracts.sh first\033[0m"
    exit 1
fi

TILE_CONTRACT=$(grep "^tile_contract=" "$STATE_FILE" | cut -d'=' -f2)
if [ -z "$TILE_CONTRACT" ]; then
    echo -e "\033[0;31mNo contract address in $STATE_FILE\033[0m"
    exit 1
fi

if [ -z "$TOKEN_ID" ]; then
    echo -e "\033[0;31mTOKEN_ID is not set (fill scripts/messages/constants.json)\033[0m"
    exit 1
fi

echo -e "\033[0;34mMinting tile $TOKEN_ID to $DEPLOYER_ADDRESS...${NC}"

# The tile starts as the 100-pixel blank canvas. `metadata` is not optional in the
# contract's extension, so the blank canvas is sent explicitly: the contract validates
# and normalises it on mint.
BLANK_METADATA=$(jq -n \
    '[range(100) | {id: ., color: "#FFFFFF", lease_expires_at: "0", leased_by: null, last_updated_at: "0"}] | {pixels: .}')

MSG=$(jq -n \
    --arg owner "$DEPLOYER_ADDRESS" \
    --arg token_id "$TOKEN_ID" \
    --argjson metadata "$BLANK_METADATA" \
    '{
        mint: {
            token_id: $token_id,
            owner: $owner,
            token_uri: null,
            extension: {
                tile_hash: "",
                metadata: $metadata
            }
        }
    }')

echo "$MSG" | jq .

TX_RESULT=$(gaiad tx wasm execute "$TILE_CONTRACT" "$MSG" \
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
echo -e "\033[0;34m   tx: $TX_HASH\033[0m"

mkdir -p scripts/state
echo "tx_hash=$TX_HASH" > "scripts/state/03_mint_token.state"
echo "token_id=$TOKEN_ID" >> "scripts/state/03_mint_token.state"
echo "contract=$TILE_CONTRACT" >> "scripts/state/03_mint_token.state"

echo -e "\033[0;32m✅ Mint broadcast. Verify with: scripts/query_token.sh\033[0m"
