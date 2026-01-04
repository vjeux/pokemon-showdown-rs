# Battle Debugging Notes

## Seed 2 Investigation

### Issue 1: Z-move selection divergence (FIXED ✅)

**Problem:**
- JavaScript selected `tectonicrage` (Z-move) as first move in turn 1
- Rust selected `kingsshield` instead
- PRNG calls diverged (JS: 0->5, Rust: 0->3)

**Root Cause:**
The Rust `auto_choose()` function had this code:
```rust
// IMPORTANT: Do NOT select Z-moves during auto_choose
// JavaScript's request.moves does not include Z-moves
if move_slot.is_z {
    continue;
}
```

This comment was INCORRECT! When a Z-move like `tectonicrage` is in the Pokemon's base moveset (from team generation), it DOES appear in `request.moves` and should be selectable.

**Fix:**
Removed the Z-move check from `src/side/auto_choose.rs`. Z-moves that are in the moveset ARE selectable, matching JavaScript behavior.

**Commit:** 2946cb9f

---

### Issue 2: Weather event target handling and immunity (PARTIALLY FIXED ✅)

**Problem:**
- Turn 1 results diverged significantly
- Rust showed Pokemon taking duplicate sandstorm damage
- Both Pokemon were taking weather damage despite type immunities

**Root Causes Found:**
1. **Field event handlers had no target** - `find_event_handlers` was passing `None` as target for Weather events
2. **Effect type detection wrong** - `get_effect_type` checked moves before active weather, causing "sandstorm" to be identified as "Move" instead of "Weather"
3. **Weather callback didn't pass effect ID** - `dispatch_on_weather` called `battle.damage()` without the sandstorm effect ID

**Fixes Applied:**
1. Updated `find_event_handlers` (line 198-203): For non-prefixed events (Weather, Update, BeforeTurn), field handlers now use the target Pokemon from `find_event_handlers` parameter, not None
2. Updated `get_effect_type` (line 21-30): Check active field weather/terrain BEFORE checking moves to properly identify weather effects
3. Updated `dispatch_on_weather` (line 758-759): Pass `Some(&sandstorm_id)` to `battle.damage()` so immunity checks can run

**Results:**
- ✅ Sandstorm weather damage now applies to correct Pokemon (not duplicated)
- ✅ Type immunity checks now work (Ground/Steel types immune to sandstorm)
- ❌ Still seeing incorrect move damage values (different issue to investigate)

**Next Steps:**
- Investigate why move damage differs (Sandaconda: 27 vs 16 HP, Metang: 15 vs 2 HP)

---

## Debugging Tools Needed

### Current Status:
- ✅ Battle comparison script (`tests/compare-battles.sh`)
- ✅ PRNG call counting
- ✅ Turn-by-turn HP tracking
- ✅ Detailed Rust battle logs with [GET_DAMAGE], [MODIFY_DAMAGE], etc.

### Improvements Needed:
1. **HP Change Tracker**: Add explicit logging when `pokemon.hp` changes, showing:
   - What caused the change (move, weather, recoil, etc.)
   - Before/after HP values
   - Call stack or context

2. **Weather Event Tracker**: Add logging for when weather:
   - Is created
   - Deals damage
   - Is checked for immunity

3. **Turn Summary**: At end of each turn, print:
   - All HP changes that occurred
   - All weather/field effects active
   - All abilities that triggered

---

## Data Files Fixed

### conditions.json (UPDATED ✅)
- Was only 58 lines, missing most conditions
- Created `scripts/extract-conditions.js` to load JavaScript module and export metadata
- Now includes all 33 conditions with proper effectType, duration, priorities, etc.
- Includes all weather conditions with `effectType: "Weather"`

### typechart.json (UPDATED ✅)
- Added weather/status immunities:
  - `sandstorm: 3` for Ground, Rock, Steel types
  - `psn/tox: 3` for Poison, Steel types
  - `par: 3` for Electric
  - `brn: 3` for Fire
  - `frz: 3` for Ice
  - `hail: 3` for Ice
  - `powder: 3` for Grass
  - `prankster: 3` for Dark
  - `trapped: 3` for Ghost

---

## Code Fixes

### src/side/auto_choose.rs (FIXED ✅)
- Removed incorrect Z-move filtering
- Updated comments to reflect correct behavior
- Now matches JavaScript 1-to-1

### src/data/condition_callbacks.rs (IMPLEMENTED ✅)
- Added `dispatch_on_field_residual` for sandstorm
- Added `dispatch_on_weather` for sandstorm damage
- Matches JavaScript implementation

### src/battle/has_callback.rs (FIXED ✅)
- Reordered effect type priority: conditions checked BEFORE moves
- Added "FieldResidual" and "Weather" events to `condition_has_callback`
- This ensures sandstorm (condition) is found before sandstorm (move)

### src/pokemon/run_status_immunity.rs (FIXED ✅)
- Changed from hardcoded type checks to proper `battle.dex.get_immunity()` call
- Now uses `pokemon.get_types(battle, false)` to match JavaScript
- 1-to-1 implementation with JavaScript

---

## Notes

- Seed 1: ✅ PASSING (41 iterations match exactly)
- Seed 2: ❌ FAILING (diverges at turn 1 with damage issue)
