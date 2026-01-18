#!/bin/bash

# Minimize failing seeds in parallel
#
# Usage:
#   ./minimize-seeds.sh <seeds-file> [num-workers]
#   echo "943 1101 2719" | ./minimize-seeds.sh - [num-workers]
#
# Examples:
#   ./minimize-seeds.sh failing-seeds.txt           # From file, 8 workers
#   ./minimize-seeds.sh failing-seeds.txt 16        # From file, 16 workers
#   echo "943 1101" | ./minimize-seeds.sh -         # From stdin

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
MINIMIZED_DIR="$SCRIPT_DIR/minimized"
SEEDS_INPUT="${1:--}"
NUM_WORKERS="${2:-10}"

# Read seeds from file or stdin
if [ "$SEEDS_INPUT" = "-" ]; then
    SEEDS=$(cat | tr ' ' '\n' | grep -E '^[0-9]+$' | sort -n | uniq)
else
    if [ ! -f "$SEEDS_INPUT" ]; then
        echo "Seeds file not found: $SEEDS_INPUT"
        echo ""
        echo "Usage: ./minimize-seeds.sh <seeds-file> [num-workers]"
        echo "       echo \"943 1101\" | ./minimize-seeds.sh - [num-workers]"
        exit 1
    fi
    SEEDS=$(cat "$SEEDS_INPUT" | tr ' ' '\n' | grep -E '^[0-9]+$' | sort -n | uniq)
fi

if [ -z "$SEEDS" ]; then
    echo "No seeds provided"
    exit 1
fi

# Build Rust binary once upfront
echo "Building Rust binary..."
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build --release --example test_battle_rust 2>&1" > /dev/null
if [ $? -ne 0 ]; then
    echo "Failed to build Rust binary"
    exit 1
fi
echo "Build complete."
echo ""

RUST_BINARY="/home/builder/workspace/target/release/examples/test_battle_rust"

# Create worker script
WORKER_SCRIPT=$(mktemp)
cat > "$WORKER_SCRIPT" << 'EOF'
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

# Run quick comparison to verify still fails
node "$SCRIPT_DIR/test-battle-js.js" "$seed" > /tmp/js-battle-seed${seed}.txt 2>&1
docker exec pokemon-rust-dev bash -c "$RUST_BINARY $seed 2>/dev/null" > /tmp/rust-battle-seed${seed}.txt 2>&1

js_lines=$(grep "^#[0-9]" /tmp/js-battle-seed${seed}.txt 2>/dev/null)
rust_lines=$(grep "^#[0-9]" /tmp/rust-battle-seed${seed}.txt 2>/dev/null)

if [ "$js_lines" = "$rust_lines" ]; then
    echo "Seed $seed: Now passes (fixed)"
    exit 0
fi

# Run minimization
node "$SCRIPT_DIR/minimize-failure.js" "$seed" > /dev/null 2>&1

if [ -f "$MINIMIZED_DIR/seed${seed}.json" ]; then
    echo "Seed $seed: Minimized"
else
    echo "Seed $seed: Failed to minimize"
fi
EOF
chmod +x "$WORKER_SCRIPT"

# Count seeds
total=$(echo "$SEEDS" | wc -l | tr -d ' ')
already_minimized=$(echo "$SEEDS" | while read seed; do
    [ -f "$MINIMIZED_DIR/seed${seed}.json" ] && echo "$seed"
done | wc -l | tr -d ' ')
to_process=$((total - already_minimized))

echo "Seeds: $total total, $already_minimized already minimized, $to_process to process"
echo "Workers: $NUM_WORKERS"
echo ""

# Run in parallel
echo "$SEEDS" | xargs -P "$NUM_WORKERS" -I {} bash "$WORKER_SCRIPT" "$SCRIPT_DIR" "$MINIMIZED_DIR" "$RUST_BINARY" {}

# Cleanup
rm -f "$WORKER_SCRIPT"

echo ""
echo "======================================="
echo "Done. Minimized files: $(ls "$MINIMIZED_DIR"/*.json 2>/dev/null | wc -l | tr -d ' ')"
echo "======================================="
