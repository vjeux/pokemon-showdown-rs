# Battle Testing Infrastructure

This directory contains the infrastructure for comparing JavaScript and Rust battle implementations.

## Files

- **`generate-test-teams.js`** - Generates random battle teams with a specific seed
- **`test-battle-js.js`** - Runs battles in JavaScript
- **`../examples/test_battle_rust.rs`** - Runs battles in Rust
- **`compare-battles.sh`** - Compares JS and Rust outputs line by line
- **`teams-seed*.json`** - Generated team data for each seed

## Quick Start

Run a comparison test for seed 1:

```bash
./tests/compare-battles.sh 1
```

This will automatically:
1. Generate teams if needed
2. Run the battle in both JavaScript and Rust
3. Compare outputs
4. Report PASS or FAIL

## Output Format

Each line shows the battle state after a turn:

```
#<iteration>: turn=<turn>, prng=<before>-><after>, P1=[Pokemon(HP/MaxHP)], P2=[Pokemon(HP/MaxHP)]
```

**Example:**
```
#1: turn=2, prng=0->5, P1=[Decidueye(295/295)], P2=[Noctowl(188/344)]
#2: turn=2, prng=5->10, P1=[Decidueye(295/295)], P2=[Noctowl(0/344)]
```

## What the Test Captures

- **Turn number** - Current battle turn
- **PRNG calls** - Critical for detecting execution order differences
- **Pokemon HP** - Verifies damage calculations match
- **Active Pokemon** - Confirms switches and KOs happen identically

## Understanding Failures

When outputs differ, the comparison shows:

1. **First divergence point**
2. **PRNG call differences** - Indicates missing/extra events
3. **HP differences** - Shows damage calculation bugs
4. **Turn count differences** - Battle flow diverged

## Manual Testing

Generate teams:
```bash
node tests/generate-test-teams.js 5
```

Run JavaScript only:
```bash
node tests/test-battle-js.js 5
```

Run Rust only:
```bash
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo run --example test_battle_rust 5 2>&1"
```

## Testing Multiple Seeds

Each seed generates a unique random battle:

```bash
./tests/compare-battles.sh 1   # Decidueye vs Noctowl
./tests/compare-battles.sh 5   # Different matchup
./tests/compare-battles.sh 42  # Different matchup
```

Use different seeds to test various scenarios:
- Different Pokemon species
- Different move sets
- Different abilities and items
- Different battle flows

## Integration with Development

After making changes to Rust code:

1. Run comparison test: `./tests/compare-battles.sh 1`
2. If it fails, examine the first divergence point
3. Fix the Rust code to match JavaScript
4. Re-run test to verify fix
5. Commit changes

## Current Status

See `../CRITICAL_BUG_REPORT.md` for known issues.

The test infrastructure is working correctly and detecting divergences between implementations. This is the foundation for ensuring 1-to-1 parity between JavaScript and Rust.
