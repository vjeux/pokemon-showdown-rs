#!/bin/bash

# Battle Comparison Test
#
# Compares JavaScript and Rust battle implementations by:
# 1. Generating teams independently in each language (or using minimized seed files)
# 2. Comparing team generation (should match exactly)
# 3. Running battles with those teams
# 4. Comparing battle state turn-by-turn
#
# Usage: ./tests/compare-battles.sh [seed_number]
# Example: ./tests/compare-battles.sh 100

SEED=${1:-1}

# Get the directory where this script is located
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_DIR="$( cd "$SCRIPT_DIR/.." && pwd )"

# Check if a minimized seed file exists
MINIMIZED_FILE="$PROJECT_DIR/tests/minimized/seed${SEED}.json"

echo "======================================"
echo "Battle Comparison Test - Seed $SEED"
echo "======================================"
echo ""

if [ -f "$MINIMIZED_FILE" ]; then
    # Use minimized seed file
    echo "Using minimized seed file: $MINIMIZED_FILE"
    echo ""

    # Copy minimized file to both JS and Rust team locations
    cp "$MINIMIZED_FILE" /tmp/teams-seed${SEED}-js.json
    cp "$MINIMIZED_FILE" /tmp/teams-seed${SEED}-rust.json

    # Also copy to Docker container
    docker cp "$MINIMIZED_FILE" pokemon-rust-dev:/tmp/teams-seed${SEED}-rust.json

    echo "✅ Teams loaded from minimized seed file"
    echo ""
else
    # Step 1: Generate teams in both languages
    echo "Step 1: Generating teams..."
    echo ""

    echo "  JavaScript team generation:"
    node $PROJECT_DIR/tests/generate-test-teams.js $SEED

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
fi

# Step 3: Run battles
echo ""
echo "Step 3: Running battles..."
echo ""

echo "  Running JavaScript battle..."
# Run JavaScript battle and save both full output and summary lines (suppress console output)
node $PROJECT_DIR/tests/test-battle-js.js $SEED > /tmp/js-battle-seed${SEED}-stdout.txt 2> /tmp/js-battle-seed${SEED}-stderr.txt
cat /tmp/js-battle-seed${SEED}-stdout.txt /tmp/js-battle-seed${SEED}-stderr.txt > /tmp/js-battle-seed${SEED}-full.txt
grep '^#[0-9]' /tmp/js-battle-seed${SEED}-full.txt > /tmp/js-battle-seed${SEED}.txt

echo "  Running Rust battle..."
# Run Rust battle and save both full output and summary lines (suppress console output)
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example test_battle_rust $SEED 2>&1" > /tmp/rust-battle-seed${SEED}-full.txt 2>&1
grep '^#[0-9]' /tmp/rust-battle-seed${SEED}-full.txt > /tmp/rust-battle-seed${SEED}.txt

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
    echo "Generated Files"
    echo "======================================"
    echo ""
    echo "Team Files:"
    ls -lh /tmp/teams-seed${SEED}-js.json 2>/dev/null | awk '{print "  "$9" ("$5")"}'
    ls -lh /tmp/teams-seed${SEED}-rust.json 2>/dev/null | awk '{print "  "$9" ("$5")"}'
    echo ""
    echo "Battle Summary (used for comparison):"
    ls -lh /tmp/js-battle-seed${SEED}.txt 2>/dev/null | awk '{print "  "$9" ("$5")"}'
    ls -lh /tmp/rust-battle-seed${SEED}.txt 2>/dev/null | awk '{print "  "$9" ("$5")"}'
    echo ""
    echo "JavaScript Detailed Logs:"
    ls -lh /tmp/js-battle-seed${SEED}-stdout.txt 2>/dev/null | awk '{print "  "$9" ("$5")"}'
    ls -lh /tmp/js-battle-seed${SEED}-stderr.txt 2>/dev/null | awk '{print "  "$9" ("$5")"}'
    ls -lh /tmp/js-battle-seed${SEED}-full.txt 2>/dev/null | awk '{print "  "$9" ("$5")"}'
    echo ""
    echo "Rust Detailed Logs:"
    ls -lh /tmp/rust-battle-seed${SEED}-full.txt 2>/dev/null | awk '{print "  "$9" ("$5")"}'
    echo ""
    echo "To debug divergence:"
    echo "  1. Check which turn diverges above"
    echo "  2. View detailed logs at that turn:"
    echo "     cat /tmp/js-battle-seed${SEED}-full.txt | less"
    echo "     cat /tmp/rust-battle-seed${SEED}-full.txt | less"
    echo "  3. Compare side-by-side:"
    echo "     diff /tmp/js-battle-seed${SEED}-full.txt /tmp/rust-battle-seed${SEED}-full.txt | less"
    echo ""
    echo "======================================"
    echo "BATTLE TEST FAILED FOR SEED $SEED"
    echo "======================================"
    exit 1
fi

echo ""
echo "======================================"
echo "Generated Files"
echo "======================================"
echo ""
echo "Team Files:"
ls -lh /tmp/teams-seed${SEED}-js.json 2>/dev/null | awk '{print "  "$9" ("$5")"}'
ls -lh /tmp/teams-seed${SEED}-rust.json 2>/dev/null | awk '{print "  "$9" ("$5")"}'
echo ""
echo "Battle Summary (used for comparison):"
ls -lh /tmp/js-battle-seed${SEED}.txt 2>/dev/null | awk '{print "  "$9" ("$5")"}'
ls -lh /tmp/rust-battle-seed${SEED}.txt 2>/dev/null | awk '{print "  "$9" ("$5")"}'
echo ""
echo "JavaScript Detailed Logs:"
ls -lh /tmp/js-battle-seed${SEED}-stdout.txt 2>/dev/null | awk '{print "  "$9" ("$5")"}'
ls -lh /tmp/js-battle-seed${SEED}-stderr.txt 2>/dev/null | awk '{print "  "$9" ("$5")"}'
ls -lh /tmp/js-battle-seed${SEED}-full.txt 2>/dev/null | awk '{print "  "$9" ("$5")"}'
echo ""
echo "Rust Detailed Logs:"
ls -lh /tmp/rust-battle-seed${SEED}-full.txt 2>/dev/null | awk '{print "  "$9" ("$5")"}'
echo ""
echo "To view detailed logs:"
echo "  cat /tmp/js-battle-seed${SEED}-full.txt"
echo "  cat /tmp/rust-battle-seed${SEED}-full.txt"
echo ""
echo "To compare logs side-by-side:"
echo "  diff /tmp/js-battle-seed${SEED}-full.txt /tmp/rust-battle-seed${SEED}-full.txt | less"
