#!/bin/bash
# Test PRNG synchronization for battles across multiple seeds

set -e

echo "========================================="
echo "Battle PRNG Synchronization Test"
echo "========================================="
echo ""

# Compile Rust first
echo "Compiling Rust examples..."
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build --example run_battle_seed 2>&1" | grep -E "(Compiling|Finished)" || true
echo ""

# Test seeds 1-5
results=""
for seed in {1..5}; do
    echo "========================================="
    echo "Testing seed $seed battle"
    echo "========================================="

    # Run JS battle
    echo "Running JavaScript battle..."
    js_output=$(node run-battle-seed.js $seed 2>&1)
    js_total=$(echo "$js_output" | grep "Total PRNG calls" | grep -oE '[0-9]+$')

    # Run Rust battle
    echo "Running Rust battle..."
    rust_output=$(docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example run_battle_seed $seed 2>&1" | grep -E "RUST:")
    rust_total=$(echo "$rust_output" | grep "Total PRNG calls" | grep -oE '[0-9]+$')

    echo ""
    echo "Results for seed $seed:"
    echo "  JS total:   $js_total PRNG calls"
    echo "  Rust total: $rust_total PRNG calls"

    if [ "$js_total" == "$rust_total" ]; then
        echo "  ✅ MATCH"
        results="${results}Seed $seed: ✅ MATCH ($js_total calls)\n"
    else
        echo "  ❌ MISMATCH"
        results="${results}Seed $seed: ❌ MISMATCH (JS: $js_total, Rust: $rust_total)\n"
    fi

    echo ""
done

echo "========================================="
echo "Summary"
echo "========================================="
echo -e "$results"
echo "Test complete!"
