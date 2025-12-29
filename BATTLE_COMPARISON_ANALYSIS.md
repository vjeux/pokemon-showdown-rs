# Battle Comparison Analysis - Root Cause Found

## Executive Summary

The Rust battle implementation fails to match JavaScript because **stats are not calculated from base stats + EVs + IVs + nature**. All Pokemon have default stats of 0, leading to massive damage and negative HP values.

## Evidence

### Test Results Comparison

**JavaScript Battle** (working correctly):
- Turn 0: Passimian HP 302/302
- Turn 1: Passimian HP 302/302 (survived attack)
- Battle ends in 39 turns, Player 2 wins

**Rust Battle** (broken):
- Turn 0: Passimian HP 100/100 (wrong max HP)
- Turn 1: Passimian HP 18/100 (took 82 damage - way too much)
- Turn 2: Passimian HP **-61/100** (NEGATIVE HP, not fainted)
- Turn 3+: HP stays at -61, battle never ends
- Runs full 100 turns without winner

### Root Cause: Unimplemented Stat Calculation

**File:** `src/pokemon.rs:236-277` (`Pokemon::new()`)

```rust
stored_stats: StatsTable::default(),  // Line 272 - All stats = 0!
maxhp: 100,                           // Line 274 - Hardcoded
base_maxhp: 100,                      // Line 275 - Hardcoded
hp: 100,                              // Line 277 - Hardcoded
```

`StatsTable::default()` returns all zeros:
- HP: 0
- Attack: 0
- Defense: 0
- Sp. Attack: 0
- Sp. Defense: 0
- Speed: 0

### Why This Causes Massive Damage

**Damage Formula** (`src/battle.rs:12992`):
```rust
let base_damage = ((2 * level / 5 + 2) * base_power * attack / defense.max(1)) / 50;
```

With default stats:
- `attack` = 0 (from stored_stats.atk)
- `defense` = 0 → clamped to 1 via `.max(1)`

**Example calculation for Gunk Shot (level 83 Passimian):**
```
attack = 0  (WRONG - should be ~300)
defense = 1 (WRONG - should be ~200)
base_power = 120

damage = ((2 * 83 / 5 + 2) * 120 * 0 / 1) / 50
       = ((166 / 5 + 2) * 120 * 0 / 1) / 50
       = ((33 + 2) * 120 * 0 / 1) / 50
       = (35 * 120 * 0) / 50
       = 0

Wait, that should be 0 damage...
```

Let me re-examine this. If attack is 0, damage should be 0, not 82. There must be something else going on.

### WAIT - Re-examining the Evidence

Looking back at the damage calculation usage, I see that `stored_stats` is used. Let me check if these stats are perhaps calculated elsewhere after Pokemon creation...

## Hypothesis Revision Required

The damage IS being dealt, which means stats are NOT actually 0. Let me investigate where stats get populated.

## Files to Investigate

1. `src/battle.rs` - Where Pokemon are added to battle, stats might be calculated there
2. `src/side.rs` - Side initialization
3. `src/pokemon.rs` - Any stat calculation functions after `new()`

## JavaScript Reference

**File to port:** `/Users/vjeux/random/showdown/pokemon-showdown-ts/dist/sim/pokemon.js`

Key functions:
- `calculateStat()` - Calculate individual stat from base + EVs + IVs + nature + level
- HP formula: `floor(floor((2 * base + IV + floor(EV / 4)) * level / 100) + level + 10)`
- Other stats: `floor(floor((2 * base + IV + floor(EV / 4)) * level / 100 + 5) * nature_modifier)`

## Next Steps

1. Search for where `stored_stats` is set after Pokemon creation
2. If not set anywhere, implement stat calculation 1-to-1 from JavaScript
3. Implement HP calculation 1-to-1 from JavaScript
4. Verify damage calculations become correct
5. Fix faint logic to prevent negative HP
