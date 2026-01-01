#!/bin/bash
# Quick test script to run multiple seed battles and check for crashes

echo "Testing multiple random battle seeds..."
echo ""

for seed in 1 2 3 4 5; do
    echo "=== Testing seed $seed ==="

    if [ ! -f "examples/compare_seed${seed}.rs" ]; then
        echo "  Skipping - no test file"
        continue
    fi

    # Run battle and extract summary
    result=$(timeout 60 cargo run --example compare_seed${seed} 2>&1 | tail -4)

    if [ $? -eq 0 ]; then
        echo "$result"
        echo "  ✓ PASSED"
    else
        echo "  ✗ FAILED or TIMEOUT"
    fi
    echo ""
done

echo "Testing complete!"
