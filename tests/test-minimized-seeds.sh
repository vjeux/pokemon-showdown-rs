#!/bin/bash

# Test all minimized seed cases
# These are minimal reproduction cases for known bugs
# Usage: ./test-minimized-seeds.sh

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MINIMIZED_DIR="$SCRIPT_DIR/minimized"
RESULTS_FILE="minimized-seeds-results.txt"

# Start fresh output file
> "$RESULTS_FILE"

# Function to output to both console and file
log() {
    echo "$@" | tee -a "$RESULTS_FILE"
}

# Build Rust binary once upfront
log "Building Rust binary..."
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build --release --example test_battle_rust 2>&1" > /dev/null
RUST_BINARY="/home/builder/workspace/target/release/examples/test_battle_rust"

# Get all seed files
SEED_FILES=$(ls "$MINIMIZED_DIR"/seed*.json 2>/dev/null | sort -t'd' -k2 -n)
total_seeds=$(echo "$SEED_FILES" | wc -l | tr -d ' ')

log "======================================="
log "Testing Minimized Seeds ($total_seeds tests)"
log "======================================="
log ""

passed_seeds=0
failed_seeds=0

for seed_file in $SEED_FILES; do
    # Extract seed number from filename
    seed=$(basename "$seed_file" | sed 's/seed\([0-9]*\).json/\1/')

    # Copy team file to expected locations
    cp "$seed_file" "/tmp/teams-seed${seed}-js.json"
    cp "$seed_file" "/tmp/teams-seed${seed}-rust.json"
    docker cp "/tmp/teams-seed${seed}-rust.json" "pokemon-rust-dev:/tmp/teams-seed${seed}-rust.json" 2>/dev/null

    # Run JS battle
    node "$SCRIPT_DIR/test-battle-js.js" "$seed" > "/tmp/js-battle-seed${seed}.txt" 2>&1

    # Run Rust battle
    docker exec pokemon-rust-dev bash -c "$RUST_BINARY $seed 2>/dev/null" > "/tmp/rust-battle-seed${seed}.txt" 2>&1

    # Extract battle lines (lines like "#1: turn=2, prng=...")
    js_lines=$(grep "^#[0-9]*:" "/tmp/js-battle-seed${seed}.txt" 2>/dev/null)
    rust_lines=$(grep "^#[0-9]*:" "/tmp/rust-battle-seed${seed}.txt" 2>/dev/null)

    js_count=$(echo "$js_lines" | grep -c "^#[0-9]*:" 2>/dev/null || echo "0")

    if [ "$js_count" = "0" ]; then
        log "Seed $seed: ERROR - no JS output"
        failed_seeds=$((failed_seeds + 1))
        continue
    fi

    if [ "$js_lines" = "$rust_lines" ]; then
        log "Seed $seed: ✓ PASS"
        passed_seeds=$((passed_seeds + 1))
    else
        # Find first divergence
        first_diff=$(diff <(echo "$js_lines") <(echo "$rust_lines") 2>/dev/null | grep "^<" | head -1 | sed 's/^< //')
        log "Seed $seed: ✗ FAIL - $first_diff"
        failed_seeds=$((failed_seeds + 1))
    fi
done

log ""
log "======================================="
log "Summary"
log "======================================="
log "Total minimized seeds: $total_seeds"
log "Passed: $passed_seeds"
log "Failed: $failed_seeds"
log ""

if [ "$passed_seeds" = "$total_seeds" ]; then
    log "✓ ALL MINIMIZED SEEDS PASSED!"
    exit 0
else
    pass_pct=$((passed_seeds * 100 / total_seeds))
    log "Pass rate: ${pass_pct}%"
    log "✗ Some seeds still failing"
    exit 1
fi
