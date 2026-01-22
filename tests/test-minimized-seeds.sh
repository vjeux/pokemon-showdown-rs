#!/bin/bash

# Test all minimized seed cases using fast parallel implementation
# Also runs first 1000 unified tests to check for regressions
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

# Track overall success
overall_success=true

#######################################
# Part 1: Run first 1000 unified tests
#######################################
log "======================================="
log "Part 1: Regression Tests (Seeds 1-1000)"
log "======================================="
log ""

# Build Rust binaries
log "Building Rust binaries..."
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build --release --example test_unified --example test_minimized 2>&1" > /dev/null

log "Running unified tests..."

# Run JS and Rust in parallel for unified tests
node "$SCRIPT_DIR/test-unified-parallel.js" 1 1000 > /tmp/js-unified.txt 2>/dev/null &
JS_UNIFIED_PID=$!

docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && ./target/release/examples/test_unified 1 1000 2>/dev/null" > /tmp/rust-unified.txt &
RUST_UNIFIED_PID=$!

wait $JS_UNIFIED_PID
wait $RUST_UNIFIED_PID

log "Comparing unified results..."
log ""

unified_passed=0
unified_failed=0
unified_failed_seeds=""

while IFS= read -r js_line && IFS= read -r rust_line <&3; do
    seed=$(echo "$js_line" | grep -o 'SEED [0-9]*' | cut -d' ' -f2)

    if [ "$js_line" = "$rust_line" ]; then
        ((unified_passed++))
    else
        ((unified_failed++))
        unified_failed_seeds="$unified_failed_seeds $seed"
        log "FAIL Seed $seed:"
        log "  JS:   $js_line"
        log "  Rust: $rust_line"
    fi
done < /tmp/js-unified.txt 3< /tmp/rust-unified.txt

log ""
log "Unified tests: $unified_passed/1000 passed"

if [ $unified_failed -gt 0 ]; then
    log "REGRESSION DETECTED! Failed seeds:$unified_failed_seeds"
    overall_success=false
else
    log "No regressions detected."
fi

#######################################
# Part 2: Run minimized seed tests (with minimized teams)
#######################################
log ""
log "======================================="
log "Part 2: Minimized Seeds Tests (minimized teams)"
log "======================================="
log ""

# Count minimized seeds
total_minimized=$(ls "$MINIMIZED_DIR"/seed*.json 2>/dev/null | wc -l | tr -d ' ')
log "Running $total_minimized minimized seed tests..."

# Run JS and Rust in parallel for minimized tests
node "$SCRIPT_DIR/test-minimized.js" > /tmp/js-minimized.txt 2>/dev/null &
JS_MIN_PID=$!

docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && ./target/release/examples/test_minimized tests/minimized 2>/dev/null" > /tmp/rust-minimized.txt &
RUST_MIN_PID=$!

wait $JS_MIN_PID
wait $RUST_MIN_PID

log "Comparing minimized results..."
log ""

minimized_passed=0
minimized_failed=0
minimized_failed_seeds=""

# Track passing and failing seeds for bug-analysis.txt update
PASSING_SEEDS_FILE="/tmp/passing-seeds.txt"
FAILING_SEEDS_FILE="/tmp/failing-seeds.txt"
> "$PASSING_SEEDS_FILE"
> "$FAILING_SEEDS_FILE"

while IFS= read -r js_line && IFS= read -r rust_line <&3; do
    seed=$(echo "$js_line" | grep -o 'SEED [0-9]*' | cut -d' ' -f2)

    if [ "$js_line" = "$rust_line" ]; then
        ((minimized_passed++))
        echo "$seed" >> "$PASSING_SEEDS_FILE"
    else
        ((minimized_failed++))
        minimized_failed_seeds="$minimized_failed_seeds $seed"
        echo "$seed" >> "$FAILING_SEEDS_FILE"
        log "FAIL Seed $seed (minimized):"
        log "  JS:   $js_line"
        log "  Rust: $rust_line"
    fi
done < /tmp/js-minimized.txt 3< /tmp/rust-minimized.txt

#######################################
# Part 3: Run full team tests for minimized seeds
#######################################
log ""
log "======================================="
log "Part 3: Full Team Tests (same seeds, generated teams)"
log "======================================="
log ""

# Get list of seeds from minimized directory
SEED_LIST=$(ls "$MINIMIZED_DIR"/seed*.json 2>/dev/null | sed 's/.*seed\([0-9]*\)\.json/\1/' | sort -n | tr '\n' ' ')

log "Running $total_minimized seeds with full generated teams..."

# Create seed list file in workspace (accessible to both host and Docker)
SEED_LIST_FILE="$SCRIPT_DIR/seed-list-tmp.txt"
echo "$SEED_LIST" | tr ' ' '\n' | grep -E '^[0-9]+$' > "$SEED_LIST_FILE"

# Run unified tests for just these seeds
node "$SCRIPT_DIR/test-unified-parallel.js" --seeds "$SEED_LIST_FILE" > /tmp/js-full.txt 2>/dev/null &
JS_FULL_PID=$!

docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && ./target/release/examples/test_unified --seeds tests/seed-list-tmp.txt 2>/dev/null" > /tmp/rust-full.txt &
RUST_FULL_PID=$!

wait $JS_FULL_PID
wait $RUST_FULL_PID

# Clean up temp file
rm -f "$SEED_LIST_FILE"

log "Comparing full team results..."
log ""

full_passed=0
full_failed=0
full_failed_seeds=""

while IFS= read -r js_line && IFS= read -r rust_line <&3; do
    seed=$(echo "$js_line" | grep -o 'SEED [0-9]*' | cut -d' ' -f2)

    if [ "$js_line" = "$rust_line" ]; then
        ((full_passed++))
    else
        ((full_failed++))
        full_failed_seeds="$full_failed_seeds $seed"
        log "FAIL Seed $seed (full team):"
        log "  JS:   $js_line"
        log "  Rust: $rust_line"
    fi
done < /tmp/js-full.txt 3< /tmp/rust-full.txt

#######################################
# Final Summary
#######################################
log ""
log "======================================="
log "Final Summary"
log "======================================="
log ""
log "Regression tests (1-1000): $unified_passed/1000 passed"
log "Minimized teams: $minimized_passed/$total_minimized passed"
log "Full teams: $full_passed/$total_minimized passed"
log ""

if [ $unified_failed -gt 0 ]; then
    log "REGRESSION DETECTED in unified tests!"
    log "Failed seeds:$unified_failed_seeds"
    overall_success=false
fi

if [ $full_failed -gt 0 ]; then
    log "Failed full team seeds:$full_failed_seeds"
fi

if [ "$minimized_passed" = "$total_minimized" ]; then
    log "All minimized seeds passed!"
    # Clear bug-analysis.txt since all tests pass
    > "$SCRIPT_DIR/bug-analysis.txt"
else
    pass_pct=$((minimized_passed * 100 / total_minimized))
    log "Minimized seeds pass rate: ${pass_pct}%"

    if [ $minimized_failed -gt 0 ]; then
        log "Failed minimized seeds:$minimized_failed_seeds"
    fi

    # Regenerate bug-analysis.txt with only failing seeds
    log ""
    log "Updating bug-analysis.txt..."
    node "$SCRIPT_DIR/analyze-minimized.js" --failing-only "$FAILING_SEEDS_FILE" > "$SCRIPT_DIR/bug-analysis.txt"
    log "Done. See tests/bug-analysis.txt for patterns in failing seeds."
fi

log ""
if [ "$overall_success" = true ] && [ $unified_failed -eq 0 ]; then
    log "SUCCESS: No regressions detected."
    exit 0
else
    log "FAILURE: Regressions or failures detected."
    exit 1
fi
