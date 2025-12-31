# Multi-Seed PRNG Synchronization Test Results

## Date: Dec 31, 2024

## Summary

Tested PRNG synchronization between JavaScript and Rust implementations across 5 different seeds (1-5).

### Team Generation: ✅ 100% Success
All 5 seeds generate **perfectly matching teams**:
- Seed 1: Grubbin lead ✅
- Seed 2: Sandaconda lead ✅
- Seed 3: Braviary-Hisui lead ✅
- Seed 4: Lickitung lead ✅
- Seed 5: Spoink lead ✅

### Battle Synchronization: 20% Success

| Seed | JS PRNG Calls | Rust PRNG Calls | Status | Difference |
|------|---------------|-----------------|---------|-----------|
| 1    | 69            | 75              | ❌      | +6        |
| 2    | 57            | 83              | ❌      | +26       |
| 3    | 74            | 84              | ❌      | +10       |
| **4**| **96**        | **96**          | **✅**  | **0**     |
| 5    | 93            | 100             | ❌      | +7        |

## Detailed Analysis

### Seed 2 Turn-by-Turn Breakdown
The largest discrepancy (26 extra calls) occurs with seed 2:

| Turn | JS Calls | Rust Calls | Cumulative Diff |
|------|----------|------------|-----------------|
| 1    | 5        | 5          | 0 ✅            |
| 2    | 1        | 3          | +2 ❌           |
| 3    | 2        | 5          | +5 ❌           |
| 20   | 8        | 4          | +26 ❌          |

**Finding**: Divergence begins at turn 2, suggesting the issue is triggered by specific Pokemon, moves, or abilities in seed 2's teams.

## Known Issues Fixed
✅ **Punishment Bug** (Dec 31, 2024): Moves with `base_power: 0` that rely on BasePower callbacks now work correctly.

## Remaining Issues

### Issue 1: Extra PRNG Calls in Rust
- **Severity**: High
- **Affected Seeds**: 1, 2, 3, 5 (80% of tested seeds)
- **Pattern**: Rust consistently makes MORE calls than JavaScript
- **Range**: +6 to +26 extra calls over 20 turns
- **Hypothesis**: Possible causes:
  - Speed ties being handled differently
  - Extra shuffle/sort calls
  - Duplicate event handlers
  - Move selection logic differences

### Seed 4: Working Reference
Seed 4 provides a **perfect working example** (96 calls match exactly). This is valuable for:
- Identifying what conditions lead to correct synchronization
- Comparing team compositions and move sets
- Using as regression test

## Teams for Investigation

### Seed 2 (Highest Divergence: +26)
**P1**: Sandaconda, Poliwag, Slowbro, Houndoom, Smoguana, Sinistea-Antique
**P2**: Metang, Beheeyem, Lechonk, Mudkip, Politoed, Kyurem

### Seed 4 (Perfect Match: 0)
**P1**: Lickitung, Nacli, Tepig, Aggron-Mega, Ninjask, Rebble
**P2**: Celesteela, Flabébé, Cubone, Maushold-Four, Dachsbun, Sandslash-Alola

## Next Steps

1. **Compare seed 2 vs seed 4** to identify what's different
2. **Add detailed PRNG tracing** for seed 2 turn 2 to see exactly which calls are extra
3. **Check for**:
   - Speed sorting differences
   - Event handler duplication
   - Move/ability-specific bugs similar to Punishment
4. **Use seed 4 as regression test** - must remain at 96 calls

## Testing Scripts

Created automated testing tools:
- `test-seeds.sh` - Tests team generation across seeds
- `test-battle-seeds.sh` - Tests battle PRNG synchronization
- `run-battle-seed.js` - JavaScript battle runner with seed parameter
- `examples/run_battle_seed.rs` - Rust battle runner with seed parameter
- `compare-seed.sh` - Detailed turn-by-turn comparison

## Conclusion

Significant progress made:
- ✅ Team generation: 100% synchronized
- ✅ 1 of 5 battle seeds fully synchronized (20%)
- ✅ Punishment bug fixed
- ❌ 4 of 5 seeds still have PRNG desync issues

The presence of seed 4 as a perfect match proves full synchronization is achievable. The varying differences across seeds (+6, +26, +10, +7) suggest multiple distinct bugs rather than one systematic issue.
