#!/bin/bash

# Test 100 seeds and compute pass rate
# Pass rate = (number of matching turns / total JS turns) * 100%

# Output file
RESULTS_FILE="100-seeds-results.txt"

# Start fresh output file
> "$RESULTS_FILE"

# Function to output to both console and file
log() {
    echo "$@" | tee -a "$RESULTS_FILE"
}

log "======================================="
log "Testing 100 Seeds with Pass Rate"
log "======================================="
log ""

total_seeds=100
passed_seeds=0
failed_seeds=0
total_pass_rate=0

for seed in $(seq 1 $total_seeds); do
    # Run the comparison test silently
    ./tests/compare-battles.sh $seed > /tmp/seed-${seed}-result.txt 2>&1
    result=$?

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
