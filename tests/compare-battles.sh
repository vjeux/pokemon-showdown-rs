#!/bin/bash

# Battle Comparison Test
#
# Compares JavaScript and Rust battle implementations by running the same
# seed and comparing turn-by-turn state (PRNG calls and Pokemon HP).
#
# Usage: ./tests/compare-battles.sh [seed_number]

SEED=${1:-1}

echo "======================================"
echo "Battle Comparison Test - Seed $SEED"
echo "======================================"
echo ""

# Check if team file exists
if [ ! -f "tests/teams-seed${SEED}.json" ]; then
    echo "Generating test teams for seed $SEED..."
    node tests/generate-test-teams.js $SEED
    echo ""
fi

echo "Running JavaScript battle..."
node tests/test-battle-js.js $SEED 2>&1 | grep '^#' > /tmp/js-battle-seed${SEED}.txt

echo "Running Rust battle..."
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example test_battle_rust $SEED 2>&1 | grep '^#'" > /tmp/rust-battle-seed${SEED}.txt

echo ""
echo "Comparing outputs..."
echo ""

# Compare line by line
JS_LINES=$(cat /tmp/js-battle-seed${SEED}.txt)
RUST_LINES=$(cat /tmp/rust-battle-seed${SEED}.txt)

if [ "$JS_LINES" = "$RUST_LINES" ]; then
    echo "✅ PASS: JavaScript and Rust outputs match exactly!"
    echo ""
    echo "Sample output:"
    head -5 /tmp/js-battle-seed${SEED}.txt
    echo "..."
    tail -3 /tmp/js-battle-seed${SEED}.txt
else
    echo "❌ FAIL: Outputs differ!"
    echo ""
    echo "First 10 lines comparison:"
    echo ""
    echo "JavaScript:"
    head -10 /tmp/js-battle-seed${SEED}.txt
    echo ""
    echo "Rust:"
    head -10 /tmp/rust-battle-seed${SEED}.txt
    echo ""
    echo "Full diff:"
    diff /tmp/js-battle-seed${SEED}.txt /tmp/rust-battle-seed${SEED}.txt || true
fi

echo ""
echo "Full outputs saved to:"
echo "  JavaScript: /tmp/js-battle-seed${SEED}.txt"
echo "  Rust:       /tmp/rust-battle-seed${SEED}.txt"
