#!/bin/bash

# PRNG Stack Trace Helper
#
# Shows stack traces for specific PRNG calls to help debug divergences
#
# Usage: ./tests/trace-prng.sh <seed> <trace_spec> [js|rust|both]
#
# Examples:
#   ./tests/trace-prng.sh 100 1        # Trace PRNG call #1 in both JS and Rust
#   ./tests/trace-prng.sh 100 1-5      # Trace PRNG calls #1 through #5
#   ./tests/trace-prng.sh 100 1,5,10   # Trace PRNG calls #1, #5, and #10
#   ./tests/trace-prng.sh 100 1-5 js   # Trace only JavaScript

SEED=${1:-1}
TRACE_PRNG=${2:-1}
LANG=${3:-both}

if [ "$LANG" = "js" ] || [ "$LANG" = "both" ]; then
    echo "======================================"
    echo "JavaScript PRNG Trace - Seed $SEED"
    echo "Tracing calls: $TRACE_PRNG"
    echo "======================================"
    echo ""
    TRACE_PRNG=$TRACE_PRNG node tests/test-battle-js.js $SEED 2>&1 | head -100
    echo ""
fi

if [ "$LANG" = "rust" ] || [ "$LANG" = "both" ]; then
    echo "======================================"
    echo "Rust PRNG Trace - Seed $SEED"
    echo "Tracing calls: $TRACE_PRNG"
    echo "======================================"
    echo ""
    docker exec -e TRACE_PRNG="$TRACE_PRNG" pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example test_battle_rust $SEED 2>&1" | head -100
    echo ""
fi
