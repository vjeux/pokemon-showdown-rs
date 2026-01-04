#!/bin/bash
# Compare PRNG traces between JavaScript and Rust for a specific seed

SEED=${1:-1}

echo "Running JavaScript with PRNG tracing..."
TRACE_PRNG=113-119 node tests/test-battle-js.js $SEED 2>&1 | grep "PRNG #" > /tmp/js-prng-trace-$SEED.txt

echo "Running Rust with PRNG tracing..."
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && TRACE_PRNG=113-119 cargo run --example test_battle_rust $SEED 2>&1" | grep "PRNG #" > /tmp/rust-prng-trace-$SEED.txt

echo ""
echo "JavaScript PRNG calls:"
cat /tmp/js-prng-trace-$SEED.txt

echo ""
echo "Rust PRNG calls:"
cat /tmp/rust-prng-trace-$SEED.txt

echo ""
echo "Diff (< JS, > Rust):"
diff /tmp/js-prng-trace-$SEED.txt /tmp/rust-prng-trace-$SEED.txt
