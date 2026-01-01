# Battle Testing Process

This document describes the process for ensuring JS/Rust battle synchronization.

## Process

1. **Run test**: `./tests/compare-battles.sh <seed>`
2. **Check output**:
   - If teams differ → fix team generation
   - If teams match but battles differ → note first divergence point
3. **Analyze divergence**:
   - Compare PRNG call counts (if different → missing/extra events)
   - Compare HP values (if PRNG matches → damage calculation bug)
4. **Find JS code**: Locate exact JS implementation causing difference
5. **Port to Rust**: Implement 1-to-1 port, no workarounds
6. **Compile**: `docker exec pokemon-rust-dev bash -c "cd /home/builder/workspace && cargo build 2>&1" | tail -40`
7. **Retest**: Verify fix, test multiple seeds
8. **Document**: Update BATTLE_LOG.md with fix details (keep concise)
9. **Commit & push**: Git commit with clear message

## Current Status

- ✅ Team generation synchronized
- ✅ Display names fixed (formes display as base species)
- ✅ Comparison script fixed (ignores headers)
- ✅ willCrit field implemented (guaranteed crits work)
- ✅ 16-bit truncation in modifyDamage fixed
- ✅ Seed 1: Perfect match!
- ⚠️ Seed 42: Improved to turn 4, investigating faint detection at turn 4-5
- ❌ Seed 123: Persistent 3 HP damage differences (investigating)
- ❌ Seed 100: Battle logic divergences

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
