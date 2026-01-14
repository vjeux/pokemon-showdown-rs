#!/bin/bash

# Test all minimized seed cases (fast batch version)
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

# Get all seed numbers and create seeds list file
SEEDS_FILE="/tmp/minimized-seeds-list.txt"
ls "$MINIMIZED_DIR"/seed*.json 2>/dev/null | sed 's/.*seed\([0-9]*\).json/\1/' | sort -n > "$SEEDS_FILE"
total_seeds=$(wc -l < "$SEEDS_FILE" | tr -d ' ')

log "======================================="
log "Testing Minimized Seeds ($total_seeds tests)"
log "======================================="
log ""

# Step 1: Copy all minimized files to /tmp
log "Step 1: Copying team files..."
for seed in $(cat "$SEEDS_FILE"); do
    cp "$MINIMIZED_DIR/seed${seed}.json" "/tmp/teams-seed${seed}-js.json"
    cp "$MINIMIZED_DIR/seed${seed}.json" "/tmp/teams-seed${seed}-rust.json"
done

# Copy to docker
docker cp /tmp/teams-seed*-rust.json pokemon-rust-dev:/tmp/ 2>/dev/null || true
# Copy files one by one if bulk copy fails
for seed in $(cat "$SEEDS_FILE"); do
    docker cp "/tmp/teams-seed${seed}-rust.json" "pokemon-rust-dev:/tmp/teams-seed${seed}-rust.json" 2>/dev/null
done

# Also copy seeds list to docker
docker cp "$SEEDS_FILE" pokemon-rust-dev:/tmp/minimized-seeds-list.txt

# Step 2: Build Rust binary
log "Step 2: Building Rust binary..."
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build --release --example test_minimized_batch 2>&1" > /dev/null

# Step 3: Run JS batch
log "Step 3: Running JavaScript battles..."
node "$SCRIPT_DIR/test-minimized-batch-js.js"

# Step 4: Run Rust batch
log "Step 4: Running Rust battles..."
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && ./target/release/examples/test_minimized_batch /tmp/minimized-seeds-list.txt 2>/dev/null"

# Step 5: Copy Rust output files from container
log "Step 5: Copying results..."
for seed in $(cat "$SEEDS_FILE"); do
    docker cp "pokemon-rust-dev:/tmp/rust-battle-seed${seed}.txt" "/tmp/rust-battle-seed${seed}.txt" 2>/dev/null
done

# Step 6: Compare results
log ""
log "Step 6: Comparing results..."
log ""

passed_seeds=0
failed_seeds=0

# Track passing and failing seeds for bug-analysis.txt update
PASSING_SEEDS_FILE="/tmp/passing-seeds.txt"
FAILING_SEEDS_FILE="/tmp/failing-seeds.txt"
> "$PASSING_SEEDS_FILE"
> "$FAILING_SEEDS_FILE"

for seed in $(cat "$SEEDS_FILE"); do
    # Extract battle lines
    js_lines=$(grep "^#[0-9]*:" "/tmp/js-battle-seed${seed}.txt" 2>/dev/null)
    rust_lines=$(grep "^#[0-9]*:" "/tmp/rust-battle-seed${seed}.txt" 2>/dev/null)

    js_count=$(echo "$js_lines" | grep -c "^#[0-9]*:" 2>/dev/null || echo "0")

    if [ "$js_count" = "0" ]; then
        log "Seed $seed: ERROR - no JS output"
        failed_seeds=$((failed_seeds + 1))
        echo "$seed" >> "$FAILING_SEEDS_FILE"
        continue
    fi

    if [ "$js_lines" = "$rust_lines" ]; then
        log "Seed $seed: ✓ PASS"
        passed_seeds=$((passed_seeds + 1))
        echo "$seed" >> "$PASSING_SEEDS_FILE"
    else
        # Find first divergence
        first_diff=$(diff <(echo "$js_lines") <(echo "$rust_lines") 2>/dev/null | grep "^<" | head -1 | sed 's/^< //')
        log "Seed $seed: ✗ FAIL - $first_diff"
        failed_seeds=$((failed_seeds + 1))
        echo "$seed" >> "$FAILING_SEEDS_FILE"
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
    # Clear bug-analysis.txt since all tests pass
    > "$SCRIPT_DIR/bug-analysis.txt"
    exit 0
else
    pass_pct=$((passed_seeds * 100 / total_seeds))
    log "Pass rate: ${pass_pct}%"
    log "✗ Some seeds still failing"

    # Regenerate bug-analysis.txt with only failing seeds
    log ""
    log "Updating bug-analysis.txt..."
    node "$SCRIPT_DIR/analyze-minimized.js" --failing-only "$FAILING_SEEDS_FILE" > "$SCRIPT_DIR/bug-analysis.txt"
    log "Done. See tests/bug-analysis.txt for patterns in failing seeds."

    exit 1
fi
