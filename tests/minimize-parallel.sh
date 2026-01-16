#!/bin/bash

# Parallel minimization of failing seeds
# Usage: ./minimize-parallel.sh <seeds-file> [num-workers]

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MINIMIZED_DIR="$SCRIPT_DIR/minimized"
SEEDS_FILE="${1:-$SCRIPT_DIR/../failing-seeds.txt}"
NUM_WORKERS="${2:-8}"

if [ ! -f "$SEEDS_FILE" ]; then
    echo "Seeds file not found: $SEEDS_FILE"
    exit 1
fi

# Build Rust binary once upfront
echo "Building Rust binary (one-time)..."
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build --release --example test_battle_rust 2>&1" > /dev/null
if [ $? -ne 0 ]; then
    echo "Failed to build Rust binary"
    exit 1
fi
echo "Rust binary built successfully."
echo ""

RUST_BINARY="/home/builder/workspace/target/release/examples/test_battle_rust"

# Create a worker script that processes a single seed
WORKER_SCRIPT=$(mktemp)
cat > "$WORKER_SCRIPT" << 'WORKER_EOF'
#!/bin/bash
SCRIPT_DIR="$1"
MINIMIZED_DIR="$2"
RUST_BINARY="$3"
seed="$4"

# Skip if already minimized
if [ -f "$MINIMIZED_DIR/seed${seed}.json" ]; then
    echo "Seed $seed: Already minimized"
    exit 0
fi

# Generate teams
node "$SCRIPT_DIR/generate-test-teams.js" "$seed" > /dev/null 2>&1

# Run quick comparison
node "$SCRIPT_DIR/test-battle-js.js" "$seed" > /tmp/js-battle-seed${seed}.txt 2>&1
docker exec pokemon-rust-dev bash -c "$RUST_BINARY $seed 2>/dev/null" > /tmp/rust-battle-seed${seed}.txt 2>&1

# Compare just the battle lines
js_lines=$(grep "^#[0-9]" /tmp/js-battle-seed${seed}.txt 2>/dev/null)
rust_lines=$(grep "^#[0-9]" /tmp/rust-battle-seed${seed}.txt 2>/dev/null)

if [ "$js_lines" = "$rust_lines" ]; then
    echo "Seed $seed: Now passes"
    exit 0
fi

# Run minimization
node "$SCRIPT_DIR/minimize-failure.js" "$seed" > /dev/null 2>&1

if [ -f "$MINIMIZED_DIR/seed${seed}.json" ]; then
    echo "Seed $seed: Minimized"
else
    echo "Seed $seed: Failed to minimize"
fi
WORKER_EOF
chmod +x "$WORKER_SCRIPT"

# Filter out already minimized seeds
SEEDS_TO_PROCESS=$(cat "$SEEDS_FILE" | while read seed; do
    if [ ! -f "$MINIMIZED_DIR/seed${seed}.json" ]; then
        echo "$seed"
    fi
done)

total=$(echo "$SEEDS_TO_PROCESS" | grep -c .)
already_done=$(($(cat "$SEEDS_FILE" | wc -l) - total))

echo "Total seeds in file: $(cat "$SEEDS_FILE" | wc -l | tr -d ' ')"
echo "Already minimized: $already_done"
echo "Seeds to process: $total"
echo "Workers: $NUM_WORKERS"
echo ""
echo "Starting parallel minimization..."
echo ""

# Run in parallel using xargs
echo "$SEEDS_TO_PROCESS" | xargs -P "$NUM_WORKERS" -I {} bash "$WORKER_SCRIPT" "$SCRIPT_DIR" "$MINIMIZED_DIR" "$RUST_BINARY" {}

# Cleanup
rm -f "$WORKER_SCRIPT"

echo ""
echo "======================================="
echo "Summary"
echo "======================================="
echo "Minimized files in: $MINIMIZED_DIR"
ls "$MINIMIZED_DIR"/*.json 2>/dev/null | wc -l | xargs echo "Total minimized files:"
