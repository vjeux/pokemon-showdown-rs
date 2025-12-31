# Seed 2 PRNG Divergence Investigation

## Problem
Seed 2 had only 18 PRNG calls vs JavaScript's 57 across 20 turns.
- Turn 1: 5 calls (both) ✓
- Turn 2: 0 calls (Rust) vs 1 call (JS) ❌ **FIRST DIVERGENCE**
- Turn 3: 1 call (both) ✓
- Turns 4-17: 0 calls (Rust) vs 1+ calls each (JS) ❌

## ROOT CAUSE - FIXED ✅

**Bug**: kingsshield volatile was being added with `duration=None` instead of `duration=Some(1)`.

### Technical Details

1. **JavaScript behavior**:
   - kingsshield move has `volatileStatus: "kingsshield"` and `condition: { duration: 1 }`
   - When move hits, it adds kingsshield volatile with duration=1
   - During Residual event with `get_key="duration"`, both kingsshield and stall volatiles are added as handlers
   - Both have identical priority → triggers shuffle → makes PRNG call

2. **Rust bug**:
   - `run_move_effects.rs` was getting duration from `battle.dex.conditions.get("kingsshield")`
   - The global dex.conditions registry didn't have kingsshield's duration
   - Should have checked `move_data.condition.duration` first
   - This caused kingsshield volatile to have duration=None
   - Without duration, `has_get_key = get_key == Some("duration") && volatile_state.duration.is_some()` was false
   - kingsshield wasn't added as a Residual event handler
   - No tie between handlers → no shuffle → no PRNG call

### The Fix

In `src/battle_actions/run_move_effects.rs` line 143-149:

**Before**:
```rust
let default_duration = battle.dex.conditions.get(&volatile_id)
    .and_then(|cond| cond.duration);
```

**After**:
```rust
// Get default duration from move's condition first, then from dex.conditions
// When adding a volatile via move.volatileStatus, the duration comes from move.condition.duration
let move_condition_duration = move_data.condition.as_ref().and_then(|c| c.duration);
let dex_condition_duration = battle.dex.conditions.get(&volatile_id)
    .and_then(|cond| cond.duration);
let default_duration = move_condition_duration.or(dex_condition_duration);
```

### Verification

After fix, turn 2 Residual event properly finds both handlers:
```
[FIND_POKEMON_HANDLERS] Volatile 'stall' ... duration=Some(2) ... has_get_key=true
[FIND_POKEMON_HANDLERS] Adding volatile handler: stall
[FIND_POKEMON_HANDLERS] Volatile 'kingsshield' ... duration=Some(1) ... has_get_key=true
[FIND_POKEMON_HANDLERS] Adding volatile handler: kingsshield
[FIELD_EVENT] Sorting 2 handlers before processing
[SPEED_SORT TIE] Found tie between index 0 and 1
[PRNG #6] value=3880739566
```

## Impact

This fix should restore PRNG synchronization for seed 2 across all 20 turns. Any move with `volatileStatus` and `condition.duration` will now correctly apply the duration from the move's condition field.

## Commit

Committed as `0c31ea44`: "Fix kingsshield volatile duration bug"
