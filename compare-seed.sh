#!/bin/bash
# Detailed turn-by-turn comparison for a specific seed

SEED=${1:-2}

echo "========================================="
echo "Detailed PRNG comparison for seed $SEED"
echo "========================================="
echo ""

echo "Running JavaScript battle..."
js_output=$(node run-battle-seed.js $SEED 2>&1 | grep "Turn")

echo "Running Rust battle..."
rust_output=$(docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example run_battle_seed $SEED 2>&1" | grep "RUST:" | grep "Turn")

echo ""
echo "JavaScript turns:"
echo "$js_output"

echo ""
echo "Rust turns:"
echo "$rust_output"

echo ""
echo "Comparison:"
paste <(echo "$js_output") <(echo "$rust_output") | while IFS=$'\t' read -r js rust; do
    js_calls=$(echo "$js" | grep -oE 'PRNG calls: [0-9]+' | grep -oE '[0-9]+')
    js_total=$(echo "$js" | grep -oE 'total: [0-9]+' | grep -oE '[0-9]+')
    rust_calls=$(echo "$rust" | grep -oE 'PRNG calls: [0-9]+' | grep -oE '[0-9]+')
    rust_total=$(echo "$rust" | grep -oE 'total: [0-9]+' | grep -oE '[0-9]+')

    if [ "$js_total" == "$rust_total" ]; then
        echo "✅ Total: $js_total"
    else
        echo "❌ JS: $js_total, Rust: $rust_total (diff: $((rust_total - js_total)))"
    fi
done
