# Turn 33 HP Divergence - Root Cause Analysis

## Summary
The battle diverges at **Turn 33** (not Turn 38 as initially thought). Rust calculates **8 more HP damage** than JavaScript even with identical PRNG sequences.

## Key Findings

### Turn 33 Divergence
**JavaScript:**
- Zacian HP: 196 → 195 (took 1 damage)
- PRNG: 117 → 123 (6 calls)
- Golem-Alola HP: 197 → 101 (took 96 damage)

**Rust:**
- Zacian HP: 196 → 187 (took 9 damage) **← 8 HP difference!**
- PRNG: 117 → 123 (6 calls) **← Same PRNG calls**
- Golem-Alola HP: 197 → 101 (took 96 damage)

### Root Cause
**Damage calculation bug in Rust.** With identical:
- PRNG sequences (same 6 calls)
- Teams (after fixing Vivillon's ability)
- Turn state

Rust calculates **different damage** than JavaScript. This indicates a fundamental issue in the damage calculation formula, stat calculations, type effectiveness application, or damage modifiers.

## Investigation History

1. Initially thought Turn 38 was the first divergence
2. Found that Turn 38 showed different behavior (Zacian fainted in Rust, Vivillon in JS)
3. Traced backwards to find Turn 33 was the actual first divergence
4. Discovered team file discrepancy: Vivillon had "No Ability" in Rust vs "Shield Dust" in JS
5. Fixed team file, but divergence persisted
6. Confirmed root cause: **Damage calculation bug in Rust**

## Cascade Effects

The 8 HP difference at Turn 33 cascades forward:
- Turn 33: Zacian 195 (JS) vs 187 (Rust)
- Turn 34: Zacian 194 (JS) vs 178 (Rust) - grows to 16 HP
- Turn 36: Zacian 194 (JS) vs 178 (Rust) - maintains 16 HP
- Turn 37: Zacian 104 (JS) vs 88 (Rust) - still 16 HP difference
- Turn 38: Completely different outcomes due to accumulated divergence

## Next Steps

1. **Create detailed Turn 33 debug script** showing:
   - Exact moves used
   - Damage calculation steps
   - All modifiers applied
   - Type effectiveness calculations

2. **Compare damage formulas** between JS and Rust:
   - Base damage calculation
   - Type effectiveness application
   - Stat modifiers
   - Random damage roll
   - Rounding behavior

3. **Fix the damage calculation** to match JavaScript exactly

4. **Verify fix** by confirming Turn 33 damage matches (1 HP for both)

5. **Continue to Turn 38+** to find any remaining divergences

## Test Case
- Seed: [0, 0, 0, 1]
- Format: gen9randombattle
- Divergence point: Turn 33 (makeChoices call #33)
- Teams: teams-js.json / teams-rust.json (now identical after Vivillon ability fix)
