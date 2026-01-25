# Pokemon Showdown - Rust Port

A high-performance Rust implementation of the [Pokemon Showdown](https://pokemonshowdown.com/) battle simulator.

## Overview

This project is a 1-to-1 port of the Pokemon Showdown battle engine from TypeScript to Rust. The goal is to achieve exact behavioral parity with the original JavaScript implementation while benefiting from Rust's performance and safety guarantees.

**Key Features:**
- Complete battle simulation engine for Generation 9 (Scarlet/Violet) random battles
- Compatible PRNG implementation (ChaCha20-based, matching Showdown's sodium library)
- Comprehensive test suite comparing Rust output against JavaScript reference
- ~50 battles/second single-threaded, with parallel execution support via Rayon

## Project Structure

```
showdown/
├── pokemon-showdown-rs/     # Rust implementation
│   ├── src/
│   │   ├── battle/          # Core battle logic
│   │   ├── battle_actions/  # Move execution, damage calculation
│   │   ├── battle_queue/    # Turn order and action queue
│   │   ├── dex/             # Move, ability, item data access
│   │   ├── pokemon/         # Pokemon state and mutations
│   │   └── ...
│   ├── data/                # JSON data files (moves, abilities, items, etc.)
│   ├── tests/               # Battle comparison test suite
│   └── examples/            # Profiling and testing utilities
├── pokemon-showdown-ts/     # Original TypeScript implementation (reference)
├── Dockerfile               # Development environment
└── docker-compose.yml
```

## Getting Started

### Prerequisites

- Docker and Docker Compose
- Node.js (for running comparison tests)

### Setup

1. **Start the Docker container:**
   ```bash
   docker-compose up -d
   ```

2. **Build the project:**
   ```bash
   docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build --release 2>&1"
   ```

3. **Run the test suite:**
   ```bash
   ./tests/test-unified.sh 1 100
   ```

### Running Commands

All Rust commands must be run inside the Docker container:

```bash
# Build
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build 2>&1"

# Run tests
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo test 2>&1"

# Check for errors
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo check 2>&1"

# Run clippy lints
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo clippy 2>&1"
```

## Testing

The test infrastructure compares Rust battle output against the JavaScript reference implementation to ensure exact behavioral parity.

### Quick Test

```bash
# Run 100 random battles and compare results
./tests/test-unified.sh 1 100
```

### Debug a Specific Seed

```bash
# Compare JS vs Rust for a specific battle seed
./tests/compare-battles.sh 42
```

This generates detailed logs at `/tmp/js-battle-seed42-full.txt` and `/tmp/rust-battle-seed42-full.txt` for side-by-side comparison.

### Minimized Test Cases

The `tests/minimized/` directory contains minimal reproduction cases for known behavioral differences:

```bash
# Run all minimized test cases
./tests/test-minimized-seeds.sh

# Analyze which moves/abilities/items appear in failing tests
node tests/analyze-minimized.js
```

## Performance

Current benchmarks (single-threaded, release build):

| Metric | Value |
|--------|-------|
| Battles/sec | ~50 |
| Turns/sec | ~2,400 |
| ms/battle | ~20 |
| Avg turns/battle | ~49 |

Run the profiler:
```bash
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --release --example profile_battle 1000 2>&1"
```

See [PERFORMANCE.md](PERFORMANCE.md) for detailed analysis and optimization proposals.

## Development

### Porting Callbacks from JavaScript

When implementing new move/ability/item callbacks:

1. Find the callback with `// TODO: Implement 1-to-1 from JS` comment
2. Read the JavaScript implementation from the comment block
3. Implement the Rust version as an exact 1-to-1 port
4. Build and verify:
   ```bash
   docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build 2>&1"
   ./tests/test-unified.sh 1 500
   ```
5. Debug failures with:
   ```bash
   ./tests/compare-battles.sh [seed_number]
   ```

### Key Patterns

**Two-phase borrow pattern** (common in Rust port):
```rust
// Phase 1: Extract data immutably
let data = {
    let pokemon = battle.pokemon_at(pos.0, pos.1)?;
    pokemon.some_field
};

// Phase 2: Mutate
let pokemon_mut = battle.pokemon_at_mut(pos.0, pos.1)?;
pokemon_mut.modify_something(data);
```

**EventResult variants:**
- `EventResult::Continue` - Continue execution
- `EventResult::Stop` - Stop execution
- `EventResult::Boolean(bool)` - Return boolean
- `EventResult::Number(i32)` - Return number
- `EventResult::Null` - Prevent default behavior

### Debug Logging

Enable verbose debug logging (impacts performance):
```bash
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build --features debug-logging 2>&1"
```

## Architecture

The Rust port mirrors the JavaScript architecture:

- **Battle** - Main battle state container
- **Side** - Player side (team, active Pokemon slots)
- **Pokemon** - Individual Pokemon state
- **Dex** - Data access layer for moves, abilities, items, species
- **PRNG** - Deterministic random number generator (ChaCha20)
- **BattleActions** - Move execution, damage calculation, effects
- **Event System** - Callback dispatch for abilities, items, moves

## Status

The port is under active development. Current focus areas:

- Implementing remaining move/ability/item callbacks
- Fixing behavioral differences found in battle comparison tests
- Improving test coverage across different battle scenarios

See [BATTLE.md](../BATTLE.md) for detailed testing status and known issues.

## License

MIT License - see [Cargo.toml](Cargo.toml) for details.

## Acknowledgments

- [Pokemon Showdown](https://github.com/smogon/pokemon-showdown) - Original battle simulator
- [Smogon](https://www.smogon.com/) - Competitive Pokemon community
