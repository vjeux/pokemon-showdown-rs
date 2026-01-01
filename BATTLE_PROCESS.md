# Battle Testing Process

This document describes the process for ensuring JS/Rust battle synchronization.

## Process

1. **Run test**: `./tests/compare-battles.sh <seed>`
2. **Check output**: Note first divergence (PRNG count or HP difference)
3. **Trace PRNG calls** (if counts differ):
   ```bash
   # JavaScript
   TRACE_PRNG=5,6 node tests/test-battle-js.js 100

   # Rust
   docker exec -e TRACE_PRNG="5,6" pokemon-rust-dev bash -c \
     "cd /home/builder/workspace && cargo run --example test_battle_rust 100 2>&1"
   ```
4. **Find JS code**: Use stack traces to locate exact implementation
5. **Port to Rust**: 1-to-1 port, no workarounds, do infrastructure changes if needed
6. **Compile & test**: Build and verify fix works
7. **Document**: Update BATTLE_LOG.md (keep concise)
8. **Commit & push**: Clear commit message

## Current Status

- ✅ Team generation synchronized
- ✅ Display names fixed (formes display as base species)
- ✅ Comparison script fixed (ignores headers)
- ✅ willCrit field implemented (guaranteed crits work)
- ✅ 16-bit truncation in modifyDamage fixed
- ✅ Stall volatile duration fixed (all protecting moves)
- ✅ Dynamic callback order/subOrder loading from JSON
- ✅ Seed 1: Perfect match!
- ⚠️ Seed 100: Turn 5 PRNG divergence (Residual event timing issue)
- ⚠️ Seed 123: Small damage differences (3 HP per turn, PRNG matches)
- ⚠️ Seed 42: Faint detection timing (Eelektrik faints turn 4 JS, turn 5 Rust)

## Next Steps

Multiple issues to investigate:
1. Seed 42: Faint detection - Eelektrik faints turn 4 in JS, turn 5 in Rust
2. Seed 123: Small damage calculation differences (3 HP per turn, PRNG matches)
3. Seed 100: Not yet analyzed

## Quick Commands

```bash
# Test a seed
./tests/compare-battles.sh 123

# Compile Rust
docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build 2>&1" | tail -40

# Commit
git add -A && git commit -m "Fix: <description>" && curl -X POST http://127.0.0.1:3456/git -H "Content-Type: application/json" -d '{"command":"git push origin master"}'
```
