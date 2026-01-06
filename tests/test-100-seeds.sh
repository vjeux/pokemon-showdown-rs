#!/bin/bash

# Test 100 seeds and compute pass rate
# Pass rate = (number of matching turns / total JS turns) * 100%

echo "======================================="
echo "Testing 100 Seeds with Pass Rate"
echo "======================================="
echo ""

total_seeds=100
passed_seeds=0
failed_seeds=0
total_pass_rate=0

for seed in $(seq 1 $total_seeds); do
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
    diff_line=$(diff /tmp/js-battle-seed${seed}.txt /tmp/rust-battle-seed${seed}.txt 2>/dev/null | grep "^<" | head -1)
    
    if [ -z "$diff_line" ] && [ "$js_turns" = "$rust_turns" ]; then
        # Perfect match
        matching_turns=$js_turns
        pass_rate=100
        echo "✓ PASS (100%) - ${matching_turns}/${js_turns} turns"
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
        echo "✗ FAIL (${pass_rate}%) - ${matching_turns}/${js_turns} turns"
        failed_seeds=$((failed_seeds + 1))
    fi
    
    total_pass_rate=$((total_pass_rate + pass_rate))
done

echo ""
echo "======================================="
echo "Summary"
echo "======================================="
echo "Total seeds tested: $total_seeds"
echo "Passed (100%): $passed_seeds"
echo "Failed (<100%): $failed_seeds"
echo ""
average_pass_rate=$((total_pass_rate / total_seeds))
echo "Average pass rate: ${average_pass_rate}%"
echo ""

if [ "$passed_seeds" = "$total_seeds" ]; then
    echo "✓ ALL SEEDS PASSED!"
    exit 0
else
    echo "✗ Some seeds failed"
    exit 1
fi
