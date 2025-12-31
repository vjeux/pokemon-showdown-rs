# PRNG Desynchronization - Final Fix

## Status: ✅ RESOLVED (Dec 31, 2024)

## Problem

PRNG desynchronization between Rust and JavaScript implementations occurred when using moves with dynamically-calculated base power (like Punishment).

### Symptoms
- JavaScript: Made 42 PRNG calls for battle turns 1-11
- Rust (before fix): Made only 40 PRNG calls
- Missing: PRNG calls #41 and #42

### Affected Turn
**Turn 11 (Battle Turn 10)**:
- P1 (Grubbin): 240/240 HP
- P2 (Latias-Mega): 4/168 HP
- Latias-Mega uses Water Gun → Grubbin (240→189 HP)
- Grubbin uses Knock Off → Should have made 2 PRNG calls but didn't

## Root Cause

File: `src/battle_actions/get_damage.rs`

**Bug:** Early return when `base_power == 0` happened BEFORE the BasePower event was fired.

```rust
// ❌ BEFORE (buggy code)
let mut base_power = move_data.base_power;
if base_power == 0 {
    return Some(0);  // Returns here for Punishment (base_power=0)
}
// BasePower event never fires!
```

For moves like Punishment:
- Move data has `base_power: 0`
- BasePower callback calculates actual power: `60 + 20 × target.boosts`
- Early return prevented callback from running
- Damage calculation was skipped entirely
- No accuracy check (PRNG #41)
- No damage roll (PRNG #42)

## Solution

Move the `base_power == 0` check to AFTER the BasePower event:

```rust
// ✅ AFTER (fixed code)
let mut base_power = move_data.base_power;

// ... critical hit calculation ...

// Trigger BasePower event (line 142-153)
if let Some(modified_bp) = battle.run_event("BasePower", ...) {
    base_power = modified_bp;  // Punishment callback sets this to actual value
}

// Check AFTER event
if base_power == 0 {
    return Some(0);
}
```

## Results

### PRNG Call Comparison

| Call # | JavaScript | Rust (Before) | Rust (After) |
|--------|------------|---------------|--------------|
| #37    | 1699260408 | 1699260408 ✓  | 1699260408 ✓ |
| #38    | 3404192745 | 3404192745 ✓  | 3404192745 ✓ |
| #39    | 359787311  | 359787311 ✓   | 359787311 ✓  |
| #40    | 4260025370 | 4260025370 ✓  | 4260025370 ✓ |
| #41    | 2776172195 | ❌ MISSING     | 2776172195 ✓ |
| #42    | 1089458516 | ❌ MISSING     | 1089458516 ✓ |

### Total Calls (Turns 1-11)
- JavaScript: 42
- Rust (before fix): 40 ❌
- Rust (after fix): 42 ✅

## Commit

```
commit f1469dac
Author: vjeux
Date: Dec 31, 2024

Fix PRNG desync: Allow BasePower event before checking base_power=0
```

## Files Changed

- `src/battle_actions/get_damage.rs` (+6, -3)

## Impact

This fix ensures:
1. ✅ Punishment and similar moves work correctly
2. ✅ PRNG stays synchronized with JavaScript
3. ✅ Battle outcomes match between implementations
4. ✅ Damage calculations happen when they should

## Affected Moves

Potentially any move with `base_power: 0` that uses BasePower callbacks:
- Punishment (confirmed affected)
- Potentially: Electro Ball, Gyro Ball, Grass Knot, Low Kick, etc.

## Testing

Verified with seed `[0, 0, 0, 1]`:
- Team generation: 425 PRNG calls matched ✅
- Battle turns 1-11: 42 PRNG calls matched ✅
- All PRNG values identical between JS and Rust ✅
