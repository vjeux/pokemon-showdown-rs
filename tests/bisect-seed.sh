#!/bin/bash

# Bisect script for a single seed
# Usage: ./tests/bisect-seed.sh <seed>
# Returns: 0 if seed passes, 1 if seed fails, 125 if untestable (skip)

seed=$1

if [ -z "$seed" ]; then
    echo "Usage: $0 <seed>"
    exit 125
fi

# Build the project first
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build --release --example test_unified 2>&1" > /tmp/build-output.txt
if [ $? -ne 0 ]; then
    echo "Build failed - skipping this commit"
    exit 125
fi

# Run JS test for this seed
node tests/test-unified-parallel.js $seed $seed > /tmp/js-bisect.txt 2>&1 &
JS_PID=$!

# Run Rust test for this seed
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && ./target/release/examples/test_unified $seed $seed 2>/dev/null" > /tmp/rust-bisect.txt &
RUST_PID=$!

wait $JS_PID
wait $RUST_PID

# Compare outputs
js_line=$(cat /tmp/js-bisect.txt)
rust_line=$(cat /tmp/rust-bisect.txt)

if [ "$js_line" = "$rust_line" ]; then
    echo "PASS: Seed $seed matches"
    exit 0
else
    echo "FAIL: Seed $seed diverges"
    echo "  JS:   $js_line"
    echo "  Rust: $rust_line"
    exit 1
fi
