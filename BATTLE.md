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
- ✅ Turn 1-2 now match JavaScript exactly!
- ❌ PRNG diverges at turn 4: JavaScript makes 2 calls, Rust makes 3 calls (1 extra)

**Next Steps:**
- Investigate extra PRNG call in turn 4 (likely in move execution or speed resolution)

---

### Issue 3: PRNG divergence at turn 4 (IN PROGRESS 🔍)

**Problem:**
- Turns 1-2 match perfectly after weather fixes
- Turn 4 shows PRNG divergence:
  - JavaScript: prng=6->8 (2 calls)
  - Rust: prng=6->9 (3 calls)
- One extra PRNG call in Rust causing all subsequent turns to diverge

**Moves Used in Turn 4:**
- Sandaconda uses King's Shield (status move, priority +4)
- Metang uses Rock Throw (damaging move)

**Hypothesis:**
Extra PRNG call could be from:
1. Speed tie resolution (both moves have different priority, shouldn't tie)
2. Move accuracy check (King's Shield has no accuracy, Rock Throw has 90%)
3. Random damage multiplier being called when it shouldn't
4. Some effect callback making an unexpected PRNG call

**Next Steps:**
- Add PRNG tracing to identify exactly where the extra call occurs
- Compare JavaScript and Rust execution paths for turn 4

---

### Issue 4: King's Shield condition not registered (FIXED ✅)

**Problem:**
- King's Shield was adding the "kingsshield" volatile
- But the volatile condition was not registered in `conditions.rs`
- `has_callback("kingsshield", "onTryHit")` returned false
- Condition's `on_try_hit` handler never called
- Moves were not blocked by King's Shield

**Root Cause:**
In `src/data/conditions.rs`, the "kingsshield" condition was never added to the CONDITIONS registry, so `get_condition(ID::new("kingsshield"))` returned None.

**Fix:**
Added kingsshield condition definition in `src/data/conditions.rs` (lines 357-367):
```rust
map.insert(
    ID::new("kingsshield"),
    ConditionDef {
        id: ID::new("kingsshield"),
        name: "King's Shield".to_string(),
        condition_type: ConditionType::Volatile,
        duration: Some(1),
        protection: true,
        ..Default::default()
    },
);
```

**Results:**
- ✅ `has_callback("kingsshield", "onTryHit")` now returns true
- ✅ Condition's `on_try_hit` handler is now called
- ✅ King's Shield now blocks moves
- ❌ NEW ISSUE: Hardcoded kingsshield in conditions.rs (should use moves.json embedded condition)
- ❌ NEW ISSUE: King's Shield blocks moves BEFORE accuracy check (see Issue 5)

**UPDATE:** Reverted hardcoded kingsshield from conditions.rs. Found the root cause:

**Root Cause Analysis:**
1. King's Shield has an embedded condition in moves.json:
   ```json
   "volatileStatus": "kingsshield",
   "condition": { "duration": 1, "onTryHitPriority": 3 }
   ```

2. JavaScript's `addVolatile` receives the move parameter:
   ```javascript
   target.addVolatile(moveData.volatileStatus, source, move)
   ```
   This allows it to access the embedded condition from the move.

3. Rust's implementation (run_move_effects.rs:113) doesn't pass the move:
   ```rust
   Pokemon::add_volatile(battle, target_pos, volatile_id, Some(_source_pos), None, None);
   ```

4. Without the move parameter, `add_volatile` can't access the embedded condition, so:
   - `has_callback("kingsshield", "onTryHit")` returns false (condition not found)
   - Duration isn't set (volatile persists indefinitely)
   - Move isn't blocked (no onTryHit handler)

### Issue 4: King's Shield condition not registered (FIXED ✅)

**Problem:**
- King's Shield was adding the "kingsshield" volatile
- But the volatile condition was not registered in `conditions.rs`
- `has_callback("kingsshield", "onTryHit")` returned false
- Condition's `on_try_hit` handler never called
- Moves were not blocked by King's Shield

**Root Cause:**
King's Shield has an embedded condition in moves.json:
```json
"volatileStatus": "kingsshield",
"condition": { "duration": 1, "onTryHitPriority": 3 }
```

JavaScript's addVolatile receives the move parameter:
```javascript
target.addVolatile(moveData.volatileStatus, source, move)
```

Rust's implementation didn't pass the move data to access the embedded condition.

**Proper Fix IMPLEMENTED:**
1. Modified `Pokemon::add_volatile` to accept optional `&ConditionData` parameter
2. In `run_move_effects.rs`, pass `move_data.condition.as_ref()` when calling add_volatile
3. When condition not found in dex, use embedded_condition for duration, priorities, etc.
4. This is 1-to-1 with JavaScript which accesses move.condition when adding volatiles

**Commit:** 75809483

**Results:**
- ✅ Embedded conditions now accessible when adding volatiles
- ✅ Duration and priorities correctly applied from embedded conditions
- ✅ 120+ call sites updated to new signature
- ❌ Turn 2 still diverges with PRNG issue (different from King's Shield blocking)

---

### Issue 5: Turn 2 PRNG divergence (IN PROGRESS 🔍)

**Problem:**
- Turn 1 matches perfectly after embedded condition fix
- Turn 2 shows PRNG divergence:
  - JavaScript: prng=5->6 (1 call)
  - Rust: prng=5->7 (2 calls)
- One extra PRNG call in Rust

**Moves Used in Turn 2:**
- Sandaconda uses King's Shield (status move, priority +4)
- Metang uses Rock Throw (damaging move, 90% accuracy)

**Hypothesis:**
Extra PRNG call could be from:
1. King's Shield expiring (duration check)
2. Stall volatile handling
3. Some event callback making an unexpected PRNG call
4. Accuracy check happening when it shouldn't

**Next Steps:**
- Add PRNG tracing to identify exactly where the extra call occurs
- Compare JavaScript and Rust execution paths for turn 2

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

- Seed 1: ✅ PASSING (41 iterations match exactly - still passing after weather fixes!)
- Seed 2: ❌ FAILING (turns 1-2 match, diverges at turn 4 with PRNG issue)
