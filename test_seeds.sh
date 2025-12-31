#!/bin/bash
for seed in 2 3 4 5; do
  echo "=== Seed $seed ==="
  js_total=$(node run-battle-seed-detailed.js $seed 2>&1 | grep "JS:.*Turn 20" | grep -oE "total: [0-9]+" | cut -d' ' -f2)
  rust_total=$(docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example run_battle_seed $seed 2>&1" | grep "RUST:.*Turn 20" | grep -oE "total: [0-9]+" | cut -d' ' -f2)
  echo "JS: $js_total, Rust: $rust_total"
  if [ "$js_total" = "$rust_total" ]; then
    echo "✓ MATCH"
  else
    echo "✗ MISMATCH"
  fi
  echo ""
done
