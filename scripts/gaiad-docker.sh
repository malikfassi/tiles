#!/bin/bash
# `gaiad` shim that runs the official Gaia container.
#
# Why this exists: on Apple Silicon there is no arm64 Gaia binary after v27.3.0, and the
# darwin-amd64 binaries abort on macOS 27 with `__DATA_CONST segment missing SG_READ_ONLY
# flag`. Running the linux/amd64 image through Docker avoids both problems, so the
# project keeps a single `gaiad` command in front of every script.
#
# The keyring lives on the host under $GAIAD_HOME (default ~/.gaia), mounted into the
# container, so keys created here are visible to every later call. Queries are read-only
# and need no home; transactions do.
#
# Usage: identical to `gaiad`, e.g.
#   scripts/gaiad-docker.sh version
#   scripts/gaiad-docker.sh query bank balances <addr> --node <rpc>
set -euo pipefail

GAIA_IMAGE=${GAIA_IMAGE:-ghcr.io/cosmos/gaia:v28.0.0}
GAIAD_HOME=${GAIAD_HOME:-$HOME/.gaia}

mkdir -p "$GAIAD_HOME"

# `-i` keeps stdin closed (the CLI never prompts here) and `-t` is deliberately omitted:
# a TTY would make the keyring prompt, which cannot be answered from a script.
exec docker run --rm \
    --platform linux/amd64 \
    -v "$GAIAD_HOME:/root/.gaia" \
    -i \
    "$GAIA_IMAGE" \
    /usr/local/bin/gaiad "$@"
