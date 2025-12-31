#!/bin/bash

# Build and run Rust with PP tracing
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example trace-seed -- 2 2>&1" | grep -E "(Initial State|Move|pp=)" | head -20
