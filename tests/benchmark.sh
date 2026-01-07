#!/bin/bash

# Performance Benchmark: JavaScript vs Rust
#
# Compares battle execution speed between JS and Rust implementations
# Runs 100 battles and measures throughput

echo "======================================="
echo "Pokemon Showdown: JS vs Rust Benchmark"
echo "======================================="
echo ""

num_seeds=100
start_seed=1
end_seed=$num_seeds

# Step 1: Build Rust benchmark binary in release mode
echo "Step 1: Building Rust benchmark binary (release mode)..."
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build --release --example benchmark_battle_rust 2>&1" | tail -10
echo "✓ Build complete"
echo ""

# Step 2: Generate teams if needed
echo "Step 2: Checking team files..."
teams_needed=0
for seed in $(seq $start_seed $end_seed); do
    if [ ! -f "/tmp/teams-seed${seed}-js.json" ]; then
        teams_needed=$((teams_needed + 1))
    fi
done

if [ "$teams_needed" -gt 0 ]; then
    echo "  Generating $teams_needed team files..."
    for seed in $(seq $start_seed $end_seed); do
        if [ ! -f "/tmp/teams-seed${seed}-js.json" ]; then
            node tests/generate-test-teams.js $seed > /dev/null 2>&1
            docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example generate_test_teams_rust $seed 2>&1" > /dev/null 2>&1
            docker cp pokemon-rust-dev:/tmp/teams-seed${seed}-rust.json /tmp/teams-seed${seed}-rust.json 2>/dev/null
        fi
    done
fi
echo "✓ Teams ready"
echo ""

# Step 3: Run JavaScript benchmark
echo "Step 3: Running JavaScript benchmark..."
echo ""
node tests/benchmark-battle-js.js $start_seed $end_seed
echo ""

# Step 4: Run Rust benchmark
echo "Step 4: Running Rust benchmark..."
echo ""
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && /home/builder/workspace/target/release/examples/benchmark_battle_rust $start_seed $end_seed 2>/dev/null"
echo ""

# Step 5: Summary
echo "======================================="
echo "Benchmark Complete"
echo "======================================="
echo ""
echo "Compare the results above to see:"
echo "  - Total execution time (JS vs Rust)"
echo "  - Battles per second (throughput)"
echo "  - Average ms per battle"
echo ""
echo "Note: Rust is built with --release optimizations"
echo "      JavaScript runs with standard V8 JIT"
echo ""
