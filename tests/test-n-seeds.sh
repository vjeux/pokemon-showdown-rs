#!/bin/bash

# Test N seeds and compute pass rate
# Usage: ./tests/test-n-seeds.sh [N]
# Pass rate = (number of matching turns / total JS turns) * 100%

N=${1:-5}

echo "======================================="
echo "Testing $N Seeds with Pass Rate"
echo "======================================="
echo ""

passed_seeds=0
failed_seeds=0
total_pass_rate=0

for seed in $(seq 1 $N); do
    echo -n "Seed $seed: "
    
    # Run the comparison test silently
    ./tests/compare-battles.sh $seed > /tmp/seed-${seed}-result.txt 2>&1
    result=$?
    
    # Count turns in JavaScript output
    js_turns=$(grep "^#" /tmp/js-battle-seed${seed}.txt 2>/dev/null | wc -l | tr -d ' ')
    
    # Count turns in Rust output  
    rust_turns=$(grep "^#" /tmp/rust-battle-seed${seed}.txt 2>/dev/null | wc -l | tr -d ' ')
    
    if [ "$js_turns" = "0" ]; then
        echo "ERROR - no JS output"
        failed_seeds=$((failed_seeds + 1))
        continue
    fi
    
    # Find first divergence point
    diff_output=$(diff /tmp/js-battle-seed${seed}.txt /tmp/rust-battle-seed${seed}.txt 2>/dev/null)
    
    if [ -z "$diff_output" ]; then
        # Perfect match
        matching_turns=$js_turns
        pass_rate=100
        echo "✓ PASS (100%) - ${matching_turns}/${js_turns} turns"
        passed_seeds=$((passed_seeds + 1))
    else
        # Find first line number where they differ
        first_diff=$(echo "$diff_output" | grep -n "^<" | head -1 | cut -d: -f1)
        
        if [ -z "$first_diff" ]; then
            # Rust has extra lines at end
            matching_turns=$js_turns
        else
            # They diverged at some point
            # Extract the turn number from the line before divergence
            matching_turns=$((first_diff / 2))
            if [ "$matching_turns" -gt "$js_turns" ]; then
                matching_turns=$js_turns
            fi
        fi
        
        pass_rate=$((matching_turns * 100 / js_turns))
        echo "✗ FAIL (${pass_rate}%) - ${matching_turns}/${js_turns} turns"
        failed_seeds=$((failed_seeds + 1))
    fi
    
    total_pass_rate=$((total_pass_rate + pass_rate))
done

echo ""
echo "======================================="
echo "Summary"
echo "======================================="
echo "Total seeds tested: $N"
echo "Passed (100%): $passed_seeds"
echo "Failed (<100%): $failed_seeds"
echo ""
average_pass_rate=$((total_pass_rate / N))
echo "Average pass rate: ${average_pass_rate}%"
echo ""

if [ "$passed_seeds" = "$N" ]; then
    echo "✓ ALL SEEDS PASSED!"
    exit 0
else
    echo "✗ Some seeds failed"
    exit 1
fi
