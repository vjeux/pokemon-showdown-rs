#!/bin/bash
# Test PRNG synchronization across multiple seeds

set -e

echo "Testing PRNG synchronization across multiple seeds..."
echo ""

# Test seeds 1-5
for seed in {1..5}; do
    echo "========================================="
    echo "Testing seed $seed"
    echo "========================================="

    # Generate teams
    echo "Generating teams for seed $seed..."
    node generate-teams.js $seed > /dev/null 2>&1
    docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example export_teams $seed 2>&1 | grep -E '(exported|team)'" || true

    # Compare team files
    if [ $seed -eq 1 ]; then
        js_file="teams-js.json"
        rust_file="teams-rust.json"
    else
        js_file="teams-js-seed${seed}.json"
        rust_file="teams-rust-seed${seed}.json"
    fi

    # Check if files exist
    if [ ! -f "$js_file" ] || [ ! -f "$rust_file" ]; then
        echo "❌ Missing team files for seed $seed"
        continue
    fi

    # Quick comparison (just show first Pokemon from each team)
    echo ""
    echo "JS P1:   $(cat $js_file | jq -r '.p1[0].species')"
    echo "Rust P1: $(cat $rust_file | jq -r '.p1[0].species')"

    if [ "$(cat $js_file | jq -r '.p1[0].species')" == "$(cat $rust_file | jq -r '.p1[0].species')" ]; then
        echo "✅ P1 lead matches for seed $seed"
    else
        echo "❌ P1 lead MISMATCH for seed $seed"
    fi

    echo ""
done

echo ""
echo "========================================="
echo "Test complete!"
echo "========================================="
