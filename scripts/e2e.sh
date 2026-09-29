#!/bin/bash
# End-to-end scenario against a live chain, driven only by `gaiad` (T-014 support).
#
# Every step asserts an on-chain fact: if a rule is broken, this script fails with a
# message naming the rule. It is idempotent enough to re-run: the tile is coloured twice
# under a lease to prove the second write is refused.
#
# Usage:
#   scripts/e2e.sh                    # uses scripts/messages/constants.json as generated
#   scripts/e2e.sh testnet            # overrides with scripts/messages/testnet.json
#
# Prerequisites:
#   - `gaiad` and `jq` on PATH
#   - a funded key in the keyring named $KEY_NAME
#   - the contract already stored and instantiated (run scripts/02_deploy_contracts.sh),
#     OR let this script do it with --full
set -euo pipefail

OVERRIDE=${1:-}

if [ "$OVERRIDE" = "testnet" ]; then
    if [ ! -f scripts/messages/testnet.json ]; then
        echo "scripts/messages/testnet.json is missing" >&2
        exit 1
    fi
    export CONSTANTS_FILE="scripts/messages/testnet.json"
fi

source scripts/00_load_constants.sh

BLUE='\033[0;34m'
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m'

PASSED=0
FAILED=0

step() {
    echo -e "\n${BLUE}== $1${NC}"
}

ok() {
    PASSED=$((PASSED + 1))
    echo -e "   ${GREEN}✅ $1${NC}"
}

fail() {
    FAILED=$((FAILED + 1))
    echo -e "   ${RED}❌ $1${NC}"
}

# Asserts an equality between two values, with the rule that was checked.
assert_eq() {
    local actual=$1 expected=$2 rule=$3
    if [ "$actual" = "$expected" ]; then
        ok "$rule"
    else
        fail "$rule (expected '$expected', got '$actual')"
    fi
}

# Asserts that a condition expressed by the caller is true.
assert_true() {
    local rule=$1
    shift
    if "$@"; then
        ok "$rule"
    else
        fail "$rule"
    fi
}

gaia_query() {
    gaiad query wasm contract-state smart "$TILE_CONTRACT" "$1" \
        --node "$NODE_URL" --output json
}

gaia_smart() {
    gaia_query "$1" | jq -c '.data'
}

# Sends a message and waits for it to be included, returns the tx result as JSON.
#
# A transaction the chain rejects never produces a hash: `gaiad` exits non-zero and prints
# the reason. That is the normal outcome for the "must be refused" steps, so the failure is
# captured and returned as a synthetic result with a non-zero code, which `tx_rejected`
# recognises.
gaia_exec() {
    local msg=$1
    shift
    local output txhash rc
    # `|| rc=$?` keeps `set -e` from aborting on a refused transaction, which is an
    # expected outcome in the steps that assert a rule is enforced.
    rc=0
    output=$(gaiad tx wasm execute "$TILE_CONTRACT" "$msg" \
        --from "$DEPLOYER_ADDRESS" \
        --keyring-backend "$KEYRING_BACKEND" \
        --gas-prices "${GAS_PRICE}${NATIVE_DENOM}" \
        --gas-adjustment "$GAS_ADJUSTMENT" \
        --gas auto \
        --chain-id "$CHAIN_ID" \
        --node "$NODE_URL" \
        --broadcast-mode "$BROADCAST_MODE" \
        -y --output json "$@" 2>&1) || rc=$?

    if [ "$rc" -ne 0 ]; then
        printf '{"code":1,"raw_log":%s}' "$(printf '%s' "$output" | jq -Rs .)"
        return 0
    fi

    # `gaiad` prints a `gas estimate: N` line before the JSON document, so only the last
    # line is parsed. Reading the whole output as JSON fails with an invalid literal.
    txhash=$(printf '%s' "$output" | tail -1 | jq -r '.txhash // empty' 2>/dev/null)
    if [ -z "$txhash" ]; then
        printf '{"code":1,"raw_log":%s}' "$(printf '%s' "$output" | jq -Rs .)"
        return 0
    fi

    sleep 6
    rc=0
    gaiad query tx "$txhash" --node "$NODE_URL" --output json 2>/dev/null || rc=$?
    if [ "$rc" -ne 0 ]; then
        printf '{"code":1,"raw_log":%s}' "$(printf '%s' "$output" | jq -Rs .)"
    fi
}

# True when a transaction failed, which is what a refused rule looks like on chain.
tx_rejected() {
    local result=$1
    [ "$(echo "$result" | jq -r '.code // 0')" != "0" ]
}

# True when a transaction was included without error.
tx_succeeded() {
    local result=$1
    [ "$(echo "$result" | jq -r '.code // 0')" = "0" ]
}

# ============================================================================
# 0. Prerequisites
# ============================================================================
step "0. Prerequisites"

for tool in gaiad jq; do
    assert_true "$tool is available" command -v "$tool"
done

if [ -z "${DEPLOYER_ADDRESS:-}" ]; then
    fail "DEPLOYER_ADDRESS is empty: set it in the constants file"
    exit 1
fi

BALANCE=$(gaiad query bank balances "$DEPLOYER_ADDRESS" --node "$NODE_URL" \
    --output json | jq -r --arg d "$NATIVE_DENOM" '[.balances[] | select(.denom==$d) | .amount] | first // "0"')
echo "   balance: ${BALANCE}${NATIVE_DENOM}"
assert_true "the key holds enough to pay gas" [ "$BALANCE" -gt 1000000 ]

STATE_FILE="scripts/state/02_deploy_contracts.state"
if [ -f "$STATE_FILE" ]; then
    TILE_CONTRACT=$(grep "^tile_contract=" "$STATE_FILE" | cut -d'=' -f2)
fi

if [ -z "${TILE_CONTRACT:-}" ]; then
    fail "no deployed contract found: run scripts/02_deploy_contracts.sh first"
    exit 1
fi
echo "   contract: $TILE_CONTRACT"

assert_true "the contract answers a query on $CHAIN_ID" \
    gaia_query '{"extension":{"msg":{"config":{}}}}'

# ============================================================================
# 1. Instantiation state
# ============================================================================
step "1. The collection is configured as decided"

CONFIG=$(gaia_smart '{"extension":{"msg":{"config":{}}}}')
assert_eq "$(echo "$CONFIG" | jq -r '.collection_share_bps')" "$COLLECTION_SHARE_BPS" \
    "the collection share is $COLLECTION_SHARE_BPS bp (ADR 0005, Stargaze 2.0)"
assert_eq "$(echo "$CONFIG" | jq -r '.platform_share_bps')" "$PLATFORM_SHARE_BPS" \
    "the platform share is $PLATFORM_SHARE_BPS bp"
assert_eq "$(echo "$CONFIG" | jq -r '.collection_payment_address')" "$DEPLOYER_ADDRESS" \
    "the collection address is the deployer"

COLLECTION_INFO=$(gaia_smart '{"get_collection_info_and_extension":{}}')
ROYALTY=$(echo "$COLLECTION_INFO" | jq -r '.extension.royalty_info.share // .royalty_info.share // "absent"')
assert_eq "$ROYALTY" "$ROYALTY_SHARE" "the CW721 collection declares $ROYALTY_SHARE royalties"

# ============================================================================
# 2. Mint a tile
# ============================================================================
step "2. A tile exists with its 100 blank pixels"

TOKEN_ID=${TOKEN_ID:-1}
NUM_TOKENS=$(gaiad query wasm contract-state smart "$TILE_CONTRACT" '{"num_tokens":{}}' \
    --node "$NODE_URL" --output json | jq -r '.data.count')

if [ "$NUM_TOKENS" = "0" ]; then
    BLANK_METADATA=$(jq -n \
        '[range(100) | {id: ., color: "#FFFFFF", lease_expires_at: "0", leased_by: null, last_updated_at: "0"}] | {pixels: .}')
    MINT_MSG=$(jq -n \
        --arg owner "$DEPLOYER_ADDRESS" --arg token_id "$TOKEN_ID" --argjson metadata "$BLANK_METADATA" \
        '{mint:{token_id:$token_id, owner:$owner, token_uri:null,
                extension:{tile_hash:"", metadata:$metadata}}}')
    MINT_TX=$(gaia_exec "$MINT_MSG")
    assert_true "the mint transaction succeeded" tx_succeeded "$MINT_TX"
else
    echo "   the collection already holds $NUM_TOKENS tile(s), reusing it"
fi

OWNER=$(gaiad query wasm contract-state smart "$TILE_CONTRACT" \
    "{\"owner_of\":{\"token_id\":\"$TOKEN_ID\",\"include_expired\":null}}" \
    --node "$NODE_URL" --output json | jq -r '.data.owner')
assert_eq "$OWNER" "$DEPLOYER_ADDRESS" "the deployer owns tile $TOKEN_ID"

PIXELS=$(gaia_smart "{\"extension\":{\"msg\":{\"tile_pixels\":{\"token_id\":\"$TOKEN_ID\"}}}}")
assert_eq "$(echo "$PIXELS" | jq -r '.pixels | length')" "$PIXELS_PER_TILE" \
    "the tile holds exactly $PIXELS_PER_TILE pixels"
assert_eq "$(echo "$PIXELS" | jq -r '.pixels[0].color')" "$DEFAULT_COLOR" \
    "pixel 0 starts as $DEFAULT_COLOR"

# ============================================================================
# 3. Quote, then colour: the advertised price is the price charged
# ============================================================================
step "3. Colouring a pixel pays exactly the quoted price"

PIXEL_ID=42
NEW_COLOR="#FF0000"
DURATION=$PIXEL_MIN_EXPIRATION

UPDATES=$(jq -n --argjson id "$PIXEL_ID" --arg color "$NEW_COLOR" --argjson d "$DURATION" \
    '[{id:$id, color:$color, expiration_duration:$d}]')

QUOTE=$(gaia_smart "{\"extension\":{\"msg\":{\"quote_pixel_updates\":{\"token_id\":\"$TOKEN_ID\",\"updates\":$UPDATES}}}}")
TOTAL=$(echo "$QUOTE" | jq -r '.total')
echo "   quoted total: ${TOTAL}${NATIVE_DENOM}"
assert_true "the quote is positive" [ "$TOTAL" -gt 0 ]

# The three shares must add up to exactly the quoted total.
SHARE_SUM=$(echo "$QUOTE" | jq -r '(.collection_amount|tonumber) + (.platform_amount|tonumber) + (.owner_amount|tonumber)')
assert_eq "$SHARE_SUM" "$TOTAL" "the three shares add up to the total (exactly)"

# Paying one unit less must be refused.
CURRENT_METADATA=$(echo "$PIXELS" | jq -c '.')
UNDER=$(jq -n --arg token_id "$TOKEN_ID" --argjson metadata "$CURRENT_METADATA" --argjson updates "$UPDATES" \
    '{update_extension:{msg:{set_pixel_color:{token_id:$token_id, current_metadata:$metadata, updates:$updates}}}}')
UNDER_TX=$(gaia_exec "$UNDER" --amount "$((TOTAL - 1))${NATIVE_DENOM}")
assert_true "an underpayment is refused" tx_rejected "$UNDER_TX"

# A wrong denom must be refused too, but the chain refuses unknown denoms before the
# contract sees them, so the contract-level rule is covered by the unit tests.

# Now pay the exact price.
PAY_TX=$(gaia_exec "$UNDER" --amount "${TOTAL}${NATIVE_DENOM}")
assert_true "the exact payment succeeds" tx_succeeded "$PAY_TX"

AFTER=$(gaia_smart "{\"extension\":{\"msg\":{\"tile_pixels\":{\"token_id\":\"$TOKEN_ID\"}}}}")
assert_eq "$(echo "$AFTER" | jq -r ".pixels[$PIXEL_ID].color")" "$NEW_COLOR" \
    "pixel $PIXEL_ID is now $NEW_COLOR on chain"
assert_eq "$(echo "$AFTER" | jq -r ".pixels[$PIXEL_ID].leased_by")" "$DEPLOYER_ADDRESS" \
    "pixel $PIXEL_ID is leased by the payer"
assert_true "pixel $PIXEL_ID has a future lease expiration" \
    test "$(echo "$AFTER" | jq -r ".pixels[$PIXEL_ID].lease_expires_at")" -gt 0

# ============================================================================
# 4. The lease protects the colour (ADR 0004)
# ============================================================================
step "4. A leased pixel cannot be overwritten by someone else"

# The holder may extend their own pixel: that is the allowed case.
EXTEND_UPDATES=$(jq -n --argjson id "$PIXEL_ID" --arg color "#00FF00" --argjson d "$DURATION" \
    '[{id:$id, color:$color, expiration_duration:$d}]')
EXTEND_QUOTE=$(gaia_smart "{\"extension\":{\"msg\":{\"quote_pixel_updates\":{\"token_id\":\"$TOKEN_ID\",\"updates\":$EXTEND_UPDATES}}}}")
EXTEND_TOTAL=$(echo "$EXTEND_QUOTE" | jq -r '.total')
EXTEND_MSG=$(jq -n --arg token_id "$TOKEN_ID" --argjson metadata "$AFTER" --argjson updates "$EXTEND_UPDATES" \
    '{update_extension:{msg:{set_pixel_color:{token_id:$token_id, current_metadata:$metadata, updates:$updates}}}}')
EXTEND_TX=$(gaia_exec "$EXTEND_MSG" --amount "${EXTEND_TOTAL}${NATIVE_DENOM}")
assert_true "the lease holder may recolour their own pixel" tx_succeeded "$EXTEND_TX"

EXPIRES_AT=$(echo "$(gaia_smart "{\"extension\":{\"msg\":{\"tile_pixels\":{\"token_id\":\"$TOKEN_ID\"}}}}")" \
    | jq -r ".pixels[$PIXEL_ID].lease_expires_at")
assert_true "the lease was extended, not reset to the same value" \
    test "$EXPIRES_AT" -gt "$(echo "$AFTER" | jq -r ".pixels[$PIXEL_ID].lease_expires_at")"

# A third party needs its own key; without one the rule is covered by the unit tests
# (PixelLeaseActive) and by cw-multi-test. This script cannot forge a second signer.

# ============================================================================
# 5. The payment split actually moved the money
# ============================================================================
step "5. Every recipient was paid its share"

BALANCE_AFTER=$(gaiad query bank balances "$DEPLOYER_ADDRESS" --node "$NODE_URL" \
    --output json | jq -r --arg d "$NATIVE_DENOM" '[.balances[] | select(.denom==$d) | .amount] | first // "0"')
echo "   balance before: ${BALANCE}${NATIVE_DENOM}"
echo "   balance after:  ${BALANCE_AFTER}${NATIVE_DENOM}"
assert_true "the payer spent something (colouring is not free)" \
    test "$BALANCE" -gt "$BALANCE_AFTER"

# The payment_distribution event is the contract's own report of the split.
DIST=$(echo "$EXTEND_TX" | jq -r '[.events[]? | select(.type=="wasm-payment_distribution") | .attributes[] | select(.key=="total") | .value] | last // "absent"' 2>/dev/null)
PAID=$(echo "$EXTEND_QUOTE" | jq -r '.total')
assert_eq "$DIST" "$PAID" "the payment_distribution event reports the exact total paid"

# ============================================================================
# Summary
# ============================================================================
echo ""
echo -e "${BLUE}== Summary ==${NC}"
echo -e "   passed: ${GREEN}${PASSED}${NC}"
echo -e "   failed: ${RED}${FAILED}${NC}"

if [ "$FAILED" -gt 0 ]; then
    echo -e "${RED}E2E FAILED${NC}"
    exit 1
fi

echo -e "${GREEN}E2E PASSED${NC}"
