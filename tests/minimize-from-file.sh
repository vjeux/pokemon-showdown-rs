#!/bin/bash

# Minimize all failing seeds from a file
# Usage: ./minimize-from-file.sh <seeds-file>

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MINIMIZED_DIR="$SCRIPT_DIR/minimized"
SEEDS_FILE="${1:-$SCRIPT_DIR/../failing-seeds.txt}"

if [ ! -f "$SEEDS_FILE" ]; then
    echo "Seeds file not found: $SEEDS_FILE"
    exit 1
fi

# Build Rust binary once upfront
echo "Building Rust binary (one-time)..."
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build --release --example test_battle_rust 2>&1" > /dev/null
if [ $? -ne 0 ]; then
    echo "Failed to build Rust binary"
    exit 1
fi
echo "Rust binary built successfully."
echo ""

RUST_BINARY="/home/builder/workspace/target/release/examples/test_battle_rust"

# Read seeds from file
FAILING_SEEDS=$(cat "$SEEDS_FILE" | sort -n | uniq)

total=$(echo "$FAILING_SEEDS" | wc -l | tr -d ' ')
current=0
skipped=0
still_pass=0
minimized=0

echo "Found $total failing seeds to check"
echo ""

for seed in $FAILING_SEEDS; do
    current=$((current + 1))

    # Skip if already minimized
    if [ -f "$MINIMIZED_DIR/seed${seed}.json" ]; then
        echo "[$current/$total] Seed $seed: Already minimized, skipping"
        skipped=$((skipped + 1))
        continue
    fi

    echo -n "[$current/$total] Seed $seed: Checking if still fails... "

    # Generate teams
    node "$SCRIPT_DIR/generate-test-teams.js" "$seed" > /dev/null 2>&1

    # Run quick comparison
    node "$SCRIPT_DIR/test-battle-js.js" "$seed" > /tmp/js-battle-seed${seed}.txt 2>&1
    docker exec pokemon-rust-dev bash -c "$RUST_BINARY $seed 2>/dev/null" > /tmp/rust-battle-seed${seed}.txt 2>&1

    # Compare just the battle lines
    js_lines=$(grep "^#[0-9]" /tmp/js-battle-seed${seed}.txt 2>/dev/null)
    rust_lines=$(grep "^#[0-9]" /tmp/rust-battle-seed${seed}.txt 2>/dev/null)

    if [ "$js_lines" = "$rust_lines" ]; then
        echo "Now passes, skipping"
        still_pass=$((still_pass + 1))
        continue
    fi

    echo "Still fails, minimizing..."

    # Run minimization, capture just the summary
    output=$(node "$SCRIPT_DIR/minimize-failure.js" "$seed" 2>&1)
    echo "$output" | grep -E "(Minimized team saved|Diverges at line)" | head -5

    minimized=$((minimized + 1))
    echo ""
done

echo ""
echo "======================================="
echo "Summary"
echo "======================================="
echo "Total seeds checked: $total"
echo "Already minimized: $skipped"
echo "Now pass (fixed): $still_pass"
echo "Newly minimized: $minimized"
echo ""
echo "Minimized files saved to: $MINIMIZED_DIR"
