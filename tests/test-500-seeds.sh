#!/bin/bash

# Test seeds and compute pass rate
# Uses batch runners for efficiency - all seeds run in single JS/Rust invocations
# Usage: ./test-500-seeds.sh [start] [end]
#   - No args: tests seeds 1-500
#   - Two args: tests seeds start-end

# Parse arguments
start_seed=${1:-1}
end_seed=${2:-500}

# Output file named after range
RESULTS_FILE="${start_seed}-${end_seed}-seeds-results.txt"

# Start fresh output file
> "$RESULTS_FILE"

# Function to output to both console and file
log() {
    echo "$@" | tee -a "$RESULTS_FILE"
}

total_seeds=$((end_seed - start_seed + 1))

log "======================================="
log "Testing Seeds ${start_seed}-${end_seed} (${total_seeds} seeds)"
log "======================================="
log ""
passed_seeds=0
failed_seeds=0
total_pass_rate=0

# Step 1: Generate all teams (batch)
log "Step 1: Generating teams..."
node tests/generate-teams-batch.js $start_seed $end_seed > /dev/null 2>&1
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --release --example generate_teams_batch $start_seed $end_seed 2>/dev/null" > /dev/null

# Step 2: Run all JS battles (batch)
log "Step 2: Running JavaScript battles..."
node tests/test-battle-batch.js $start_seed $end_seed > /dev/null 2>&1

# Step 3: Run all Rust battles (batch)
log "Step 3: Running Rust battles..."
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --release --example test_battle_batch $start_seed $end_seed 2>/dev/null" > /dev/null

# Copy Rust output files from container
for seed in $(seq $start_seed $end_seed); do
    docker cp pokemon-rust-dev:/tmp/rust-battle-seed${seed}.txt /tmp/rust-battle-seed${seed}.txt 2>/dev/null
    docker cp pokemon-rust-dev:/tmp/teams-seed${seed}-rust.json /tmp/teams-seed${seed}-rust.json 2>/dev/null
done

log "Step 4: Comparing results..."
log ""

# Step 4: Compare results
for seed in $(seq $start_seed $end_seed); do
    # Count turns in JavaScript output
    js_turns=$(grep "^#" /tmp/js-battle-seed${seed}.txt 2>/dev/null | wc -l | tr -d ' ')

    # Count turns in Rust output
    rust_turns=$(grep "^#" /tmp/rust-battle-seed${seed}.txt 2>/dev/null | wc -l | tr -d ' ')

    if [ "$js_turns" = "0" ]; then
        log "Seed $seed: ERROR - no JS output"
        failed_seeds=$((failed_seeds + 1))
        continue
    fi

    # Find first divergence point
    diff_line=$(diff /tmp/js-battle-seed${seed}.txt /tmp/rust-battle-seed${seed}.txt 2>/dev/null | grep "^<" | head -1)

    if [ -z "$diff_line" ] && [ "$js_turns" = "$rust_turns" ]; then
        # Perfect match
        matching_turns=$js_turns
        pass_rate=100
        log "Seed $seed: ✓ PASS (100%) - ${matching_turns}/${js_turns} turns"
        passed_seeds=$((passed_seeds + 1))
    else
        # Find how many turns matched before divergence
        if [ -z "$diff_line" ]; then
            # No diff but different counts
            matching_turns=$([ "$js_turns" -lt "$rust_turns" ] && echo "$js_turns" || echo "$rust_turns")
        else
            # Extract turn number from first diverging line
            turn_num=$(echo "$diff_line" | grep -o "turn=[0-9]*" | head -1 | cut -d= -f2)
            if [ -z "$turn_num" ]; then
                matching_turns=0
            else
                matching_turns=$((turn_num - 1))
            fi
        fi

        pass_rate=$((matching_turns * 100 / js_turns))
        log "Seed $seed: ✗ FAIL (${pass_rate}%) - ${matching_turns}/${js_turns} turns"
        failed_seeds=$((failed_seeds + 1))
    fi

    total_pass_rate=$((total_pass_rate + pass_rate))
done

log ""
log "======================================="
log "Summary"
log "======================================="
log "Total seeds tested: $total_seeds"
log "Passed (100%): $passed_seeds"
log "Failed (<100%): $failed_seeds"
log ""
average_pass_rate=$((total_pass_rate / total_seeds))
log "Average pass rate: ${average_pass_rate}%"
log ""

if [ "$passed_seeds" = "$total_seeds" ]; then
    log "✓ ALL SEEDS PASSED!"
    exit 0
else
    log "✗ Some seeds failed"
    exit 1
fi
