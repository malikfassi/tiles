#!/bin/bash
# Loads the generated constants into the environment.
#
# Values are emitted as a NUL-separated `key=value` stream and read back with `read -d ''`,
# so spaces and quotes inside a constant survive. The previous `eval` of `export K=V` broke
# on any value containing a space: `COLLECTION_DESCRIPTION=A collaborative pixel art canvas`
# silently exported `A` and tried to run `collaborative` as a command.
set -e

CONSTANTS_FILE=${CONSTANTS_FILE:-"scripts/messages/constants.json"}
if [ ! -f "$CONSTANTS_FILE" ]; then
    echo "Constants file not found: $CONSTANTS_FILE (run 'cargo build' first to generate the default one)" >&2
    exit 1
fi

while IFS= read -r -d '' entry; do
    export "$entry"
done < <(jq -j 'to_entries[] | "\(.key)=\(.value)\u0000"' "$CONSTANTS_FILE")
 