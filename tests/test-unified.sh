#!/bin/bash

# Ultra-Fast Unified Test - No file I/O, direct comparison
#
# Usage: ./tests/test-unified.sh [start] [end]

start_seed=${1:-1}
end_seed=${2:-500}

total_seeds=$((end_seed - start_seed + 1))

echo "======================================="
echo "Unified Testing Seeds ${start_seed}-${end_seed} (${total_seeds} seeds)"
echo "======================================="
echo ""

# Run JS and Rust in parallel, capture outputs
echo "Running battles..."
node tests/test-unified-parallel.js $start_seed $end_seed > /tmp/js-unified.txt 2>&1 &
JS_PID=$!

docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && ./target/release/examples/test_unified $start_seed $end_seed 2>/dev/null" > /tmp/rust-unified.txt &
RUST_PID=$!

wait $JS_PID
wait $RUST_PID

echo "Comparing results..."
echo ""

passed=0
failed=0
failed_seeds=""

while IFS= read -r js_line && IFS= read -r rust_line <&3; do
    seed=$(echo "$js_line" | grep -o 'SEED [0-9]*' | cut -d' ' -f2)

    if [ "$js_line" = "$rust_line" ]; then
        ((passed++))
    else
        ((failed++))
        failed_seeds="$failed_seeds $seed"
        echo "FAIL Seed $seed:"
        echo "  JS:   $js_line"
        echo "  Rust: $rust_line"
    fi
done < /tmp/js-unified.txt 3< /tmp/rust-unified.txt

echo ""
echo "======================================="
echo "Summary"
echo "======================================="
echo "Total: $total_seeds"
echo "Passed: $passed"
echo "Failed: $failed"

if [ $failed -gt 0 ]; then
    echo ""
    echo "Failed seeds:$failed_seeds"
    exit 1
else
    echo ""
    echo "✓ ALL SEEDS PASSED!"
    exit 0
fi
