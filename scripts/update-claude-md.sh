#!/bin/bash

# Update CLAUDE.md with new battle testing documentation

CLAUDE_MD="/Users/vjeux/random/showdown/CLAUDE.md"

# Create backup
cp "$CLAUDE_MD" "${CLAUDE_MD}.bak"

cat > "$CLAUDE_MD" << 'EOF'
# Pokemon Showdown Rust Port - Development Guide

This document contains important information for working on this project with Claude Code.

## Table of Contents

1. [Docker Environment](#docker-environment)
2. [Git Operations via HTTP Server](#git-operations-via-http-server)
3. [Battle Testing](#battle-testing)
4. [Development Workflow](#development-workflow)
5. [Debugging](#debugging)

---

## Docker Environment

**IMPORTANT:** All bash commands (cargo, rustc, etc.) must be run inside the Docker container.

The Docker container is named `pokemon-rust-dev` and the workspace is at `/home/builder/workspace`.

### Running Commands in Docker

Always use this pattern for running commands:

```bash
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && <command> 2>&1" | tail -20
```

### Examples

**Build the project:**
```bash
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build 2>&1" | tail -40
```

**Run tests:**
```bash
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo test 2>&1" | tail -20
```

**Check compilation:**
```bash
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo check 2>&1" | tail -20
```

**Run clippy:**
```bash
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo clippy 2>&1" | tail -20
```

---

## Git Operations via HTTP Server

Due to macOS network restrictions that block SSH connections from sandboxed processes, git operations must be performed via an HTTP server proxy.

### Starting the Git Server

The user must start the git server in their shell:

```bash
cd /Users/vjeux/random/showdown/pokemon-showdown-rs
node git-server.js
```

The server runs on `http://127.0.0.1:3456` and accepts git commands via HTTP POST requests.

### Using Git from Claude Code

Once the server is running, use curl to execute git commands:

**Push commits:**
```bash
curl -X POST http://127.0.0.1:3456/git \
  -H "Content-Type: application/json" \
  -d '{"command":"git push origin master"}'
```

**Pull changes:**
```bash
curl -X POST http://127.0.0.1:3456/git \
  -H "Content-Type: application/json" \
  -d '{"command":"git pull --rebase"}'
```

**Add files:**
```bash
curl -X POST http://127.0.0.1:3456/git \
  -H "Content-Type: application/json" \
  -d '{"command":"git add ."}'
```

**Commit:**
```bash
curl -X POST http://127.0.0.1:3456/git \
  -H "Content-Type: application/json" \
  -d '{"command":"git commit -m \"Your commit message\""}'
```

### Git Server Response Format

The server returns JSON with:
- `stdout`: Standard output from the git command
- `stderr`: Standard error from the git command
- `exitCode`: Exit code (0 = success)

Example response:
```json
{"stdout":"On branch master\nYour branch is up to date with 'origin/master'.\n","stderr":"","exitCode":0}
```

---

## Battle Testing

The project includes a comprehensive battle testing infrastructure to ensure the Rust implementation matches JavaScript behavior exactly.

### Test Infrastructure Overview

The battle testing system consists of:

1. **Fast Parallel Test** (`tests/test-unified.sh`) - Runs thousands of battles in parallel, comparing JS vs Rust
2. **Single Seed Debugger** (`tests/compare-battles.sh`) - Detailed turn-by-turn comparison for debugging
3. **Minimized Seeds Test** (`tests/test-minimized-seeds.sh`) - Runs 723 minimal bug reproduction cases

### Quick Start

**Run fast parallel tests (recommended for regression testing):**
```bash
./tests/test-unified.sh 1 500
```

This runs 500 seeds in ~8 seconds using parallel execution (10 threads in Rust via Rayon, 10 workers in Node.js).

**Run a single seed comparison (for debugging):**
```bash
./tests/compare-battles.sh [seed_number]
```

**Run all minimized bug tests:**
```bash
./tests/test-minimized-seeds.sh
```

This runs 723 minimal test cases and reports which bugs are fixed.

### Fast Parallel Test Details

The unified test (`tests/test-unified.sh`) uses:
- `tests/test-unified-parallel.js` - Parallel JS test runner using worker threads
- `examples/test_unified.rs` - Parallel Rust test runner using Rayon

Output format:
```
SEED <n>: turns=<t>, prng=<p>, winner=<w>
```

Performance (on 10-core machine):
- 500 seeds: ~8 seconds
- 2000 seeds: ~30 seconds
- Rust is ~3.5x faster than JavaScript

### Minimized Test Cases

The `tests/minimized/` directory contains 723 minimal reproduction cases for known bugs. Each file is a small team configuration that triggers a specific divergence between JS and Rust.

**Analyze bug patterns:**
```bash
node tests/analyze-minimized.js
```

This shows which moves, abilities, and items appear most frequently in failing tests.

**Create a new minimized test:**
```bash
node tests/minimize-failure.js [seed_number]
```

This takes a failing seed and reduces it to the smallest team that still fails.

### Understanding Test Failures

When tests fail, the comparison shows:

1. **Seed number** - Which seed failed
2. **Turn count mismatch** - Different number of turns indicates divergence
3. **PRNG call mismatch** - Different RNG calls indicates missing/extra events
4. **Winner mismatch** - Different battle outcomes

**Example failure:**
```
FAIL Seed 65:
  JS:   SEED 65: turns=18, prng=76, winner=p2
  Rust: SEED 65: turns=29, prng=132, winner=p2
```

---

## Development Workflow

### Porting Callbacks from JavaScript to Rust

1. Find the callback with `// TODO: Implement 1-to-1 from JS` comment
2. Read the JavaScript implementation from the comment block
3. Implement the Rust version following these rules:
   - **Must be exactly 1-to-1 with JavaScript**
   - Use two-phase borrow pattern (immutable read, then mutable if needed)
   - Return appropriate EventResult variant
   - If infrastructure is missing, document in TODO.md and return `EventResult::Continue`

4. Compile after each implementation:
```bash
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build 2>&1" | tail -40
```

5. Run fast parallel tests to check for regressions:
```bash
./tests/test-unified.sh 1 500
```

6. For debugging specific failures, use the single seed comparison:
```bash
./tests/compare-battles.sh [seed_number]
```

7. Commit if successful:
```bash
git add <files>
git commit -m "Implement <callback_name> for <move/item>"
curl -X POST http://127.0.0.1:3456/git -H "Content-Type: application/json" -d '{"command":"git push origin master"}'
```

### Common Patterns

**Two-phase borrow pattern:**
```rust
// Phase 1: Extract data immutably
let (some_data, other_data) = {
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    (pokemon.some_field, pokemon.other_field)
};

// Phase 2: Use mutable reference
let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
    Some(p) => p,
    None => return EventResult::Continue,
};
pokemon_mut.modify_something();
```

**EventResult variants:**
- `EventResult::Continue` - Continue execution
- `EventResult::Stop` - Stop execution
- `EventResult::NotFail` - Don't fail
- `EventResult::Boolean(bool)` - Return boolean
- `EventResult::Number(i32)` - Return number
- `EventResult::Null` - Return null (prevents default behavior)

**Type multipliers:**
Use `chain_modify_fraction(numerator, denominator)` for damage/stat modifiers:
- 1.3x boost: `chain_modify(1.3)`
- 1/2x reduction: `chain_modify_fraction(1, 2)`

---

## Debugging

### Rust Stack Traces

To print stack traces in Rust:

```rust
use std::backtrace::Backtrace;

println!("Custom backtrace: {}", Backtrace::force_capture());
```

### JavaScript Stack Traces

To print stack traces in JavaScript:

```js
console.trace('Debug point');
```

### Battle Debugging Strategy

When debugging battle divergences:

1. **Identify the divergence point** using comparison tests
2. **Add logging** at that specific turn/iteration
3. **Compare event order** between JS and Rust
4. **Trace PRNG calls** to find missing/extra randomness
5. **Verify damage calculations** step by step
6. **Check ability/item callbacks** are firing correctly

### Common Issues

**PRNG call count mismatch:**
- Missing event handlers
- Extra random chance checks
- Different execution order

**HP differences:**
- Damage calculation bugs
- Modifier application issues (double-applying, not applying)
- Type effectiveness errors

**Turn count mismatch:**
- Pokemon fainting early/late due to damage bugs
- Speed calculation differences
- Priority move handling

---

## Important Notes

- Never run `cargo` or other build commands directly - always use docker
- Never run `git push` directly - always use the HTTP server
- Check if the git server is running before attempting git operations
- All file paths should be absolute when possible
- Commit frequently with descriptive messages
- Document missing infrastructure in TODO.md files rather than inventing solutions
- **Always run battle tests after making changes to verify no regressions**

---

## Project Philosophy

**1-to-1 Porting:**

This is a direct port of Pokemon Showdown from JavaScript to Rust. The goal is NOT to improve or optimize, but to match the JavaScript behavior exactly.

- NO workarounds or approximations
- Find exact JavaScript implementation
- Port line-by-line to Rust
- May require infrastructure changes - do them
- Test after each change
- Use battle comparison tests to verify correctness

**Testing First:**

The battle comparison tests are the source of truth:

- If tests pass, implementations match
- If tests fail, Rust needs to change to match JavaScript
- Never change JavaScript to match Rust
- Generate multiple seeds to test different scenarios
- All 1-to-1 ports must maintain test passing status
EOF

echo "Updated $CLAUDE_MD"
echo "Backup saved to ${CLAUDE_MD}.bak"
