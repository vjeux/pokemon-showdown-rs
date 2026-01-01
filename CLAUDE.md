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

The project includes a comprehensive battle testing infrastructure to ensure the Rust implementation matches JavaScript behavior exactly, turn by turn.

### Test Infrastructure Overview

The battle testing system consists of:

1. **JavaScript Team Generator** (`tests/generate-test-teams.js`) - Generates random battle teams using JavaScript
2. **Rust Team Generator** (`examples/generate_test_teams_rust.rs`) - Generates random battle teams using Rust
3. **JavaScript Test Runner** (`tests/test-battle-js.js`) - Runs battles in JavaScript
4. **Rust Test Runner** (`examples/test_battle_rust.rs`) - Runs battles in Rust
5. **Comparison Script** (`tests/compare-battles.sh`) - Compares outputs in 4 steps:
   - Step 1: Generate teams in JavaScript → `teams-seed*-js.json`
   - Step 2: Generate teams in Rust → `teams-seed*-rust.json`
   - Step 3: Compare team generation (must match exactly!)
   - Step 4: Compare battle execution (if teams match)

**Important:** Each language generates its own teams to ensure the random team generation logic is also synchronized. Teams must match before battle testing can proceed.

### Quick Start

**Run a battle comparison test:**

```bash
./tests/compare-battles.sh [seed_number]
```

This will:
1. Generate teams if they don't exist
2. Run the battle in JavaScript
3. Run the battle in Rust (in Docker)
4. Compare outputs line by line
5. Report PASS or FAIL

**Example:**

```bash
./tests/compare-battles.sh 1
```

### Test Output Format

Each test outputs lines in this format:

```
#<iteration>: turn=<turn>, prng=<before>-><after>, P1=[Pokemon(HP/MaxHP), ...], P2=[Pokemon(HP/MaxHP), ...]
```

**Example:**
```
#1: turn=2, prng=0->6, P1=[Grubbin(168/168)], P2=[Slowbro(263/263)]
#2: turn=3, prng=6->12, P1=[Grubbin(145/168)], P2=[Slowbro(240/263)]
```

This format captures:
- **Turn number** - Current battle turn
- **PRNG calls** - Number of random number generator calls (critical for detecting divergence)
- **Pokemon state** - Name and HP of all active Pokemon

### Running Tests Manually

**1. Generate teams:**

JavaScript:
```bash
node tests/generate-test-teams.js [seed_number]
```

Rust:
```bash
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example generate_test_teams_rust [seed_number] 2>&1"
```

This creates:
- `tests/teams-seed[N]-js.json` - Teams generated by JavaScript
- `tests/teams-seed[N]-rust.json` - Teams generated by Rust

**2. Compare team generation:**

```bash
diff tests/teams-seed1-js.json tests/teams-seed1-rust.json
```

If teams differ, the random team generation logic is not synchronized.

**3. Run JavaScript battle:**

```bash
node tests/test-battle-js.js [seed_number]
```

**4. Run Rust battle:**

```bash
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example test_battle_rust [seed_number] 2>&1"
```

**5. Compare battle outputs:**

```bash
node tests/test-battle-js.js 1 2>&1 | grep '^#' > js-output.txt
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example test_battle_rust 1 2>&1 | grep '^#'" > rust-output.txt
diff js-output.txt rust-output.txt
```

If the outputs match exactly, the implementations are synchronized!

### Understanding Test Failures

**Team Generation Failures:**

If teams differ between JS and Rust:
- Random team generation logic is not synchronized
- Could be due to different PRNG call patterns
- Different sorting/ordering of species/moves/items
- Different filtering logic

Fix team generation before testing battles, as battles will definitely diverge if teams are different.

**Battle Execution Failures:**

When battle tests fail, the comparison script shows:

1. **First divergence point** - Which turn/iteration differs
2. **PRNG call mismatch** - Different number of RNG calls indicates missing/extra events
3. **HP differences** - Different damage calculations or battle outcomes

**Example failure:**

```
❌ FAIL: Outputs differ!

JavaScript:
#1: turn=2, prng=0->6, P1=[Medicham(168/243)], P2=[Alcremie(141/263)]

Rust:
#1: turn=1, prng=0->3, P1=[Medicham(243/243)], P2=[Alcremie(0/263)]
```

This shows:
- Rust dealt more damage (Alcremie at 0 HP vs 141 HP)
- Rust made fewer PRNG calls (3 vs 6)
- Turn count differs (battle progressed differently)

### Creating New Test Seeds

Different seeds generate different random battles, which can expose different bugs:

```bash
# Generate and test seed 5
./tests/compare-battles.sh 5

# Generate and test seed 42
./tests/compare-battles.sh 42

# Generate and test seed 100
./tests/compare-battles.sh 100
```

Each seed creates a unique battle scenario with different:
- Pokemon species
- Move sets
- Abilities
- Items
- Battle flow

### Integration with Development

**Workflow for fixing divergences:**

1. Run comparison test to identify failure
2. Find the first turn where outputs diverge
3. Add detailed logging to Rust code at that turn
4. Compare with JavaScript implementation line by line
5. Fix the Rust code to match JavaScript exactly
6. Re-run test to verify fix
7. Commit changes

**Example:**

```bash
# 1. Run test
./tests/compare-battles.sh 1

# 2. If it fails, examine the divergence point
# The script shows the first differing lines

# 3. Fix the code in src/

# 4. Re-test
./tests/compare-battles.sh 1

# 5. If pass, commit
git add .
git commit -m "Fix damage calculation divergence in seed 1"
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

5. Run battle tests to verify:
```bash
./tests/compare-battles.sh 1
```

6. Commit if successful:
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
