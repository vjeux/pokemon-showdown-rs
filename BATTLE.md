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

### Issue 5: Turn 2 PRNG divergence - King's Shield onTryHit not called (IN PROGRESS 🔍)

**Problem:**
- Turn 1 matches perfectly after embedded condition fix
- Turn 2 shows PRNG divergence:
  - JavaScript: prng=5->6 (1 call)
  - Rust: prng=5->7 (2 calls)
- One extra PRNG call in Rust

**Moves Used in Turn 2:**
- Sandaconda uses King's Shield (status move, priority +4, goes first)
- Metang uses Rock Throw (damaging move, 90% accuracy)

**Root Cause Found:**
Using improved PRNG stack trace logging, identified the divergence:

JavaScript PRNG #6: shuffle <- speedSort <- fieldEvent (NO accuracy check for Rock Throw)
Rust PRNG #6: random_chance <- hit_step_accuracy (accuracy check for Rock Throw)
Rust PRNG #7: random_with_range <- shuffle_range <- speed_sort

Rock Throw's accuracy is being checked in Rust but not in JavaScript. This is because:
1. King's Shield executes first (correct priority)
2. King's Shield adds `kingsshield` volatile to Sandaconda
3. Rock Throw targets Sandaconda (has kingsshield volatile)
4. **BUG**: King's Shield's onTryHit callback is NEVER CALLED
5. Accuracy check proceeds (PRNG #6)
6. Move is not blocked

In JavaScript, the kingsshield onTryHit blocks the move BEFORE the accuracy check, so no PRNG call is made.

**Next Steps:**
- Investigate why onTryHit callback is not being found/called for kingsshield volatile
- Check has_callback() or event handler registration for embedded condition callbacks

**Debugging Improvements Made:**
- Added compact stack trace logging to both JavaScript and Rust PRNG
- TRACE_PRNG now shows full call chain making it easy to identify divergences

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


---

## Session 2026-01-03: Post-Refactor Fixes

### Issue 1: ConditionData.name must be optional (FIXED ✅)

**Problem:**
- After refactoring conditions to use dex pattern, Rust battle panicked
- Error: "missing field `name` at line 371" in moves.json
- Couldnt deserialize move data

**Root Cause:**
- JavaScript allows embedded conditions in moves without a `name` field
- Example: Move "Acupressure" has `condition: { duration: 2, counterMax: 729 }`
- Rusts ConditionData required `name: String`, causing deserialization to fail
- Embedded conditions in moves dont have names because the move itself provides the identifier

**Fix:**
1. Changed `ConditionData.name` from `String` to `Option<String>`
2. Updated `get_effect_fullname()` to use `.unwrap_or(effect_str)` for missing names
3. Added detailed error messages to `load_from_json()` to identify which JSON file fails

**Commit:** 97baa728

**Status:** ✅ Fixed - Rust battle now runs successfully

**Results:**
- Seed 1 now runs but diverges at turn 26 (PRNG: JS=113->117 vs Rust=113->119)
- Rust makes 2 extra PRNG calls
- Need to investigate whats causing the extra calls

---

### Current Issue: PRNG Divergence at Turn 26 (IN PROGRESS 🔍)

**Observation:**
- JavaScript: turn 26, PRNG 113->117 (4 calls)
- Rust: turn 26, PRNG 113->119 (6 calls)
- Rust makes 2 extra PRNG calls at this turn

**Next Steps:**
1. Check turn 26 logs to see what moves are being executed
2. Identify which code path is calling PRNG extra times
3. Compare JavaScript and Rust execution at that turn



**Investigation Progress:**

Using improved PRNG tracing (commit 1b003677), identified the extra calls:

**JavaScript turn 26 (PRNG 113->117):**
- #113: from=100 (accuracy/chance check)
- #114: from=100 (accuracy/chance check)
- #115: from=24 (damage roll)
- #116: from=16 (random selection)

**Rust turn 26 (PRNG 113->119):**
- #113: from=100 (accuracy/chance check)
- #114: from=100 (accuracy/chance check) 
- #115: from=24 (damage roll)
- #116: from=16 (random selection)
- #117: from=100 (EXTRA secondary effect roll - 20% chance) <- spread_move_hit
- #118: from=24 (EXTRA damage roll) <- get_damage

**Root Cause FOUND and FIXED ✅:**
The extra PRNG calls were from gmaxterror (G-Max Terror move):
- gmaxterror is a Gengar-specific Max move with `isMax: "Gengar"`
- Golem-Alola doesn't have the 'dynamax' volatile
- JavaScript: Max move check returns early BEFORE crit calculation → NO PRNG calls
- Rust (before fix): Max move check happened AFTER crit calculation → 2 PRNG calls (crit + randomizer)

**Fix Applied (Commit 61840515):**
1. Moved Max move dynamax check to happen BEFORE crit calculation in get_damage.rs
2. Added early return when Max move used without dynamax volatile
3. This prevents all PRNG calls for Max moves without dynamax

**Result:**
- ✅ Turn 26 now matches perfectly: both make 4 PRNG calls (113->117)
- ✅ Zacian HP matches: 196/331 in both
- ✅ Golem HP matches: 197/288 in both

---

### Issue 5: Turn 26 PRNG Divergence - Max moves without dynamax (FIXED ✅)

**Problem:**
- Rust made 6 PRNG calls vs JavaScript's 4 calls in turn 26
- gmaxterror (G-Max move) was making crit and damage randomizer calls even with basePower=0

**Root Cause:**
- Max move dynamax check happened AFTER crit calculation
- This allowed basePower to trigger crit check before being set to 0
- JavaScript avoids this by having BasePower event return 0, triggering early return

**Fix (Commit 61840515, final fix in current commit):**
1. Moved Max move dynamax check to happen BEFORE crit calculation
2. Added early return when Max move used without dynamax volatile
3. Returns Some(0) immediately - no damage, no PRNG calls

**Result:**
- ✅ Turn 26 now matches perfectly: both make 4 PRNG calls (113->117)
- ✅ Zacian HP matches: 196/331 in both
- ✅ Golem HP matches: 197/288 in both
- ✅ gmaxterror deals 0 damage as expected

**Next Divergence - Turn 27 Investigation:**
- JavaScript: prng=117->123 (6 calls), Zacian takes 1 damage
- Rust: prng=117->121 (4 calls), Zacian takes 0 damage
- Rust is missing 2 PRNG calls (crit + damage randomizer)
- PRNG breakdown:
  - JS: #117-120 (4 calls) + #121-123 (3 calls including crit+randomizer)
  - Rust: #117-121 (5 calls), missing final damage randomizer
- Hypothesis: gmaxterror might be dealing 1 damage in JS but 0 in Rust
- Need to investigate if there's a minimum damage mechanic or different damage calculation

---

### Issue 6: Max move basePowerCallback (FIXED ✅)

**Problem After Moving Max move check:**
After moving Max move check to after crit calculation:
- Turn 26: JavaScript makes 4 PRNG calls (113->117), Rust makes 6 calls (113->119)
- Rust makes EXTRA crit+randomizer calls for gmaxterror
- JavaScript Zacian: 196 HP (no damage from gmaxterror)
- Rust Zacian: 194 HP (2 damage from gmaxterror - critical hit!)

**Root Cause Found:**
JavaScript has an IMPLICIT basePowerCallback for ALL Max/G-Max moves that:
1. Returns 0 if Pokemon doesn't have dynamax volatile
2. This triggers early return in getDamage (line 1611) BEFORE crit calculation
3. Prevents PRNG calls for crit and randomizer

This basePowerCallback is not explicitly defined in moves.json/moves.ts, but is part of the
Max move system logic.

**Fix (Commit 29690f19):**
1. Added Max move check at beginning of `dispatch_base_power_callback()` in move_callbacks/mod.rs
2. Returns `EventResult::Number(0)` if move is Max/G-Max and Pokemon doesn't have dynamax volatile
3. This triggers early return in get_damage.rs (line 247) BEFORE crit calculation
4. Removed incorrect Max move check from get_damage.rs that was setting basePower=0 AFTER crit

**Result:**
- ✅ Turn 26 now matches perfectly: prng=113->117 (4 calls)
- ✅ Golem HP matches: 197/288 in both
- ✅ Zacian HP matches: 196/331 in both
- ✅ gmaxterror correctly doesn't make crit/randomizer PRNG calls

**Next Issue - Turn 27:**
- JavaScript: prng=117->123 (6 calls), Zacian=195/331
- Rust: prng=117->121 (4 calls), Zacian=196/331
- Rust missing 2 PRNG calls, Zacian HP differs by 1
- Hypothesis: gmaxterror should deal 1 damage in turn 27 (minimum damage mechanic)

---

### Debugging Tool Improvement: PRNG Tracing (COMPLETED ✅)

**Solution:**
Added from/to parameter logging to random() function before calling next_raw().

**Changes:**
- Modified src/prng.rs random() function to log parameters when TRACE_PRNG is set
- Now shows: [random(from=100, to=None)] before each PRNG call
- Created tests/compare-prng-trace.sh to compare JS and Rust PRNG traces side-by-side

**Commit:** 1b003677

**Results:**
- ✅ Can now identify accuracy checks (from=100)
- ✅ Can identify damage rolls (from=24)
- ✅ Can identify random selections (from=16, etc.)
- ✅ Easy to spot divergences in PRNG usage patterns


