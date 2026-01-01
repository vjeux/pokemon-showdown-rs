#!/bin/bash

# Battle Comparison Test
#
# Compares JavaScript and Rust battle implementations by:
# 1. Generating teams independently in each language
# 2. Comparing team generation (should match exactly)
# 3. Running battles with those teams
# 4. Comparing battle state turn-by-turn
#
# Usage: ./tests/compare-battles.sh [seed_number]

SEED=${1:-1}

echo "======================================"
echo "Battle Comparison Test - Seed $SEED"
echo "======================================"
echo ""

# Step 1: Generate teams in both languages
echo "Step 1: Generating teams..."
echo ""

echo "  JavaScript team generation:"
node tests/generate-test-teams.js $SEED

echo ""
echo "  Rust team generation:"
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example generate_test_teams_rust $SEED 2>&1" | grep -E '(✓|P1|P2|File)'

# Copy Rust teams from container to host /tmp
docker cp pokemon-rust-dev:/tmp/teams-seed${SEED}-rust.json /tmp/teams-seed${SEED}-rust.json

# Step 2: Compare team generation
echo ""
echo "Step 2: Comparing team generation..."
echo ""

if diff -q /tmp/teams-seed${SEED}-js.json /tmp/teams-seed${SEED}-rust.json > /dev/null 2>&1; then
    echo "✅ PASS: Team generation matches!"
else
    echo "❌ FAIL: Team generation differs between JS and Rust!"
    echo ""
    echo "This indicates the random team generation logic is not synchronized."
    echo "Differences:"
    diff /tmp/teams-seed${SEED}-js.json /tmp/teams-seed${SEED}-rust.json | head -20
    echo ""
    echo "Stopping test. Fix team generation before testing battles."
    exit 1
fi

# Step 3: Run battles
echo ""
echo "Step 3: Running battles..."
echo ""

echo "  JavaScript battle:"
node tests/test-battle-js.js $SEED 2>&1 | grep '^#' > /tmp/js-battle-seed${SEED}.txt

echo "  Rust battle:"
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example test_battle_rust $SEED 2>&1 | grep '^#'" > /tmp/rust-battle-seed${SEED}.txt

# Step 4: Compare battle outputs
echo ""
echo "Step 4: Comparing battle outputs..."
echo ""

JS_LINES=$(cat /tmp/js-battle-seed${SEED}.txt)
RUST_LINES=$(cat /tmp/rust-battle-seed${SEED}.txt)

if [ "$JS_LINES" = "$RUST_LINES" ]; then
    echo "✅ PASS: Battle outputs match exactly!"
    echo ""
    echo "Sample output:"
    head -5 /tmp/js-battle-seed${SEED}.txt
    echo "..."
    tail -3 /tmp/js-battle-seed${SEED}.txt
    echo ""
    echo "======================================"
    echo "ALL TESTS PASSED FOR SEED $SEED"
    echo "======================================"
else
    echo "❌ FAIL: Battle outputs differ!"
    echo ""
    echo "First divergence:"
    echo ""
    echo "JavaScript:"
    head -10 /tmp/js-battle-seed${SEED}.txt
    echo ""
    echo "Rust:"
    head -10 /tmp/rust-battle-seed${SEED}.txt
    echo ""
    echo "Full diff:"
    diff /tmp/js-battle-seed${SEED}.txt /tmp/rust-battle-seed${SEED}.txt | head -40 || true
    echo ""
    echo "======================================"
    echo "BATTLE TEST FAILED FOR SEED $SEED"
    echo "======================================"
    exit 1
fi

echo ""
echo "Output files:"
echo "  JS teams:     /tmp/teams-seed${SEED}-js.json"
echo "  Rust teams:   /tmp/teams-seed${SEED}-rust.json"
echo "  JS battle:    /tmp/js-battle-seed${SEED}.txt"
echo "  Rust battle:  /tmp/rust-battle-seed${SEED}.txt"
