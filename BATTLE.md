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

- Seed 1: ✅ PASSING (41 iterations match exactly - fixed flinch condition and Max move minimum damage!)
- Seed 2: ❌ FAILING (AI divergence - Rust uses King's Shield in turn 2, JavaScript uses different move, causing stall counter mismatch)


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

**Fix (Commit 47840e13):**
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
- JavaScript: prng=117->123 (6 calls), Zacian=195/331 (takes 1 damage)
- Rust: prng=117->121 (4 calls), Zacian=196/331 (takes 0 damage)
- Rust missing 2 PRNG calls (crit + damage randomizer)
- Hypothesis: gmaxterror should deal minimum damage (1 HP) in turn 27, not 0

---

### Issue 7: Turn 26-27 - AI move evaluation consuming PRNG (IN PROGRESS 🔍)

**NEW DISCOVERY:**
- Turn 26: Golem didn't use ANY move (PP unchanged for all moves)
  - gmaxterror: 16/16 -> 16/16 (not used)
  - kingsshield: 16/16 -> 16/16 (not used)
  - All other moves unchanged
- Turn 27: Golem used gmaxterror (PP decreased)
  - gmaxterror: 16/16 -> 15/16 (USED)

**Key Finding:**
The gmaxterror getDamage logging appears in BOTH turns 26 and 27, but:
- Turn 26: getDamage called during AI EVALUATION (move not used)
- Turn 27: getDamage called during actual MOVE EXECUTION

This means:
1. JavaScript AI calls getDamage to evaluate potential moves
2. getDamage makes PRNG calls (crit check, randomizer)
3. These PRNG calls are counted in the turn's total
4. The move might not be selected/used, but PRNG was still consumed

**Problem:**
- Turn 26: JavaScript makes 4 PRNG calls, Rust makes 6 calls
- Turn 27: JavaScript makes 6 PRNG calls, Rust makes 4 calls

**Hypothesis:**
The difference in PRNG call counts is because:
- JavaScript AI evaluates moves and consumes PRNG during evaluation
- Rust AI might not evaluate moves the same way, or might not consume PRNG
- Need to investigate how Rust AI (auto_choose) selects moves

**Next Steps:**
1. Check if Rust auto_choose() evaluates moves by calling getDamage
2. If not, this is a fundamental difference in AI implementation
3. Need to ensure 1-to-1 parity in how AI evaluates and selects moves

---

### Issue 7: Turn 27 - gmaxterror minimum damage (IN PROGRESS 🔍)

**Problem:**
- Turn 26 now matches perfectly after basePowerCallback fix
- Turn 27 shows divergence:
  - JavaScript: prng=117->123 (6 calls), Zacian takes 1 damage (196->195)
  - Rust: prng=117->121 (4 calls), Zacian takes 0 damage (196->196)
- Rust is missing 2 PRNG calls (crit + damage randomizer)
- JavaScript makes crit and randomizer calls, suggesting basePower is NOT 0

**Hypothesis:**
In turn 27, gmaxterror might be:
1. Used by a Pokemon WITH dynamax volatile (so basePowerCallback doesn't return 0)
2. basePower goes through normal calculation
3. Final damage is 0 but minimum damage check makes it 1

Need to investigate:
- Check JavaScript battle-actions.ts line 1832 for minimum damage mechanic
- Check if turn 27 has different game state than turn 26
- Verify which Pokemon uses gmaxterror in turn 27

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



### Issue 8: Turn 25-26 - zenheadbutt secondary effect and flinch (FIXED ✅)

**Problem:**
- Rust iteration #32 labeled as "turn 26": JavaScript makes 4 PRNG calls (113->117), Rust makes 5 calls (113->118)
- Zacian HP matches (196) after fixing Max move early return
- But Rust makes 1 extra PRNG call

**Root Cause FOUND:**
After adding detailed logging, discovered:
1. Rust's turn numbers in output are OFF BY 1 (iteration #32 is actually turn 25, not 26)
2. Turn 25 in Rust executes:
   - zenheadbutt: PRNG 113->117 (4 calls total: accuracy + damage rolls + **secondary flinch roll at #117**)
   - gmaxterror: PRNG 117->118 (1 call: crit check at #118, then early return due to basePower=0)
3. JavaScript turn 26 (equivalent to Rust turn 25):
   - Only makes 4 PRNG calls (113->117)
   - Does NOT make the secondary flinch roll at #117
   - Does NOT make the gmaxterror crit check at #118

**The Extra Calls:**
- PRNG #117: zenheadbutt secondary effect (flinch) - Rust makes this call, JavaScript doesn't
- PRNG #118: gmaxterror crit check - Rust makes this call, JavaScript doesn't

**Root Cause Analysis:**
The flinch condition's `onBeforeMove` callback was not implemented - it just returned `EventResult::Continue`, which allowed the Pokemon to move even when flinched.

JavaScript implementation:
```javascript
flinch: {
    name: 'flinch',
    duration: 1,
    onBeforeMovePriority: 8,
    onBeforeMove(pokemon) {
        this.add('cant', pokemon, 'flinch');
        this.runEvent('Flinch', pokemon);
        return false;  // Prevents move
    },
},
```

**Fix Applied:**
1. Implemented `on_before_move` in `src/data/condition_callbacks/flinch.rs`
2. Added battle log message using `battle.add("cant", &[pokemon_slot, "flinch"])`
3. Triggered 'Flinch' event via `battle.run_event_bool("Flinch", ...)`
4. Returned `EventResult::Boolean(false)` to prevent the move

**Result:**
- ✅ Turn 26 now matches perfectly: both make 4 PRNG calls (113->117)
- ✅ Golem flinches and cannot use gmaxterror
- ✅ No extra PRNG calls from gmaxterror crit check
- ❌ Turn 27 now diverges: JavaScript makes 6 calls (117->123), Rust makes 5 calls (117->122)

**Commit:** [next commit]

---

### Issue 9: Turn 27 PRNG divergence - Max move early return preventing minimum damage (FIXED ✅)

**Problem:**
- Turn 26 matches after flinch fix
- Turn 27 shows PRNG divergence:
  - JavaScript: prng=117->123 (6 calls), Zacian takes 1 damage (196->195)
  - Rust: prng=117->122 (5 calls), Zacian takes 0 damage (196->196)
- Rust missing 1 PRNG call and not applying minimum damage

**Investigation:**
- Both implementations have gmaxterror being used without dynamax volatile
- JavaScript's gmaxterror deals 1 damage despite basePower=0
- Rust's gmaxterror deals 0 damage
- Both have minimum damage check: `if (gen !== 5 && !baseDamage) return 1;`

**Root Cause Found:**
In JavaScript (battle-actions.ts lines 1675-1683):
```javascript
// Hacked Max Moves have 0 base power, even if you Dynamax
if ((!source.volatiles['dynamax'] && move.isMax) || (move.isMax && this.dex.moves.get(move.baseMove).isMax)) {
    basePower = 0;
}
// ... continues with damage calculation
// ... eventually minimum damage check returns 1
```

In Rust (get_damage.rs lines 375-380, BEFORE fix):
```rust
if !has_dynamax_volatile {
    eprintln!("[GET_DAMAGE] Max/G-Max move used without dynamax volatile, basePower=0, returning Some(0)");
    return Some(0); // BUG: Early return skips minimum damage check!
}
```

**The Issue:**
- JavaScript sets basePower=0 and CONTINUES with damage calculation → minimum damage = 1
- Rust returned Some(0) immediately, SKIPPING:
  1. Rest of damage calculation
  2. Minimum damage check in modify_damage
  3. Result: gmaxterror deals 0 damage instead of 1

**Fix Applied:**
Changed get_damage.rs lines 375-391 to set `base_power = 0` and continue instead of early return:
```rust
if !has_dynamax_volatile {
    eprintln!("[GET_DAMAGE] Max/G-Max move used without dynamax volatile, setting basePower=0 and continuing");
    base_power = 0; // Set basePower to 0, but continue calculation (minimum damage will be 1)
} else if let Some(ref base_move_id) = move_data.base_move {
    if let Some(base_move_data) = battle.dex.moves().get(base_move_id.as_str()) {
        if base_move_data.is_max.is_some() {
            eprintln!("[GET_DAMAGE] Hacked Max Move detected (base move is also Max move), setting basePower=0 and continuing");
            base_power = 0; // Set basePower to 0, but continue calculation (minimum damage will be 1)
        }
    }
}
```

**Result:**
- ✅ Turn 27 now matches: both make 6 PRNG calls (117->123)
- ✅ Zacian takes 1 damage in both (196->195)
- ✅ gmaxterror with basePower=0 correctly deals minimum damage (1 HP)
- ✅ **ALL 41 TURNS NOW MATCH - SEED 1 FULLY PASSING!**

**Commit:** [next commit]

---

## Seed 3 Investigation

### Issue 10: enigmaberry on_hit callback not executing (FIXED ✅)

**Problem:**
- Seed 3 Turn 9: Dedenne takes 60 damage in Rust vs 54 damage in JavaScript
- Same PRNG calls (25->32) but different HP outcomes
- Dedenne has enigmaberry item and Cheek Pouch ability
- Expected: Berry triggers on super-effective hit, Cheek Pouch heals on berry consumption
- Actual: Berry callback never executed

**Investigation:**
1. Added logging to enigmaberry.rs on_hit callback → NO LOGS APPEARED
2. Traced event system to verify event flow:
   - ✅ run_event("Hit", ...) is called in spread_move_hit.rs
   - ✅ find_pokemon_event_handlers finds enigmaberry handler (logging confirmed)
   - ✅ Handler added to handlers list (logging confirmed)
   - ✅ dispatch_single_event called with enigmaberry (logging confirmed)
   - ✅ handle_item_event called for Hit event (logging confirmed)
3. **ROOT CAUSE FOUND**: dispatch_on_hit in item_callbacks/mod.rs was just a STUB:
   ```rust
   pub fn dispatch_on_hit(
       _battle: &mut Battle,
       _item_id: &str,
       _pokemon_pos: (usize, usize),
   ) -> EventResult {
       EventResult::Continue  // BUG: Just returns Continue without dispatching!
   }
   ```

**Fix Applied:**
1. Updated `dispatch_on_hit` function signature to accept source and move_id parameters
2. Implemented match on item_id to call enigmaberry::on_hit
3. Updated `handle_item_event.rs` Hit event case to pass source and move_id

**Implementation:**
```rust
pub fn dispatch_on_hit(
    battle: &mut Battle,
    item_id: &str,
    pokemon_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    move_id: &str,
) -> EventResult {
    use crate::dex_data::ID;
    match ID::from(item_id).as_str() {
        "enigmaberry" => enigmaberry::on_hit(battle, Some(pokemon_pos), source_pos, move_id),
        _ => EventResult::Continue,
    }
}
```

**Result:**
- ✅ enigmaberry on_hit now triggers when hit by super-effective move
- ✅ Berry consumed, heals 1/4 max HP (52 HP)
- ✅ Cheek Pouch ability triggers, heals 1/3 max HP (70 HP)
- ✅ Seed 3 iterations #1-#15 now match JavaScript exactly (previously diverged at turn 9)
- ❌ NEW DIVERGENCE at iteration #16 (turn 13):
  - JavaScript: prng=47->54, Dedenne(107/210)
  - Rust: prng=47->53, Dedenne(133/210)

**Commit:** d681cbdf

**Next Steps:**
- Investigate turn 13 divergence (PRNG calls differ by 1, HP differs by 26)

---

### Issue 11: partiallytrapped on_residual not dealing damage (FIXED ✅)

**Problem:**
- Seed 3 iteration #16 (turn 13): Dedenne HP differed by 26
  - JavaScript: Dedenne at 107/210 HP
  - Rust: Dedenne at 133/210 HP
- PRNG calls were same (47->53/54) but HP outcomes differed
- Dedenne max HP is 210, and 210/8 = 26.25 → 26 HP residual damage expected

**Root Cause:**
The `on_residual` callback for partiallytrapped condition was not implemented - it just returned `EventResult::Continue` without dealing any damage.

JavaScript implementation (data/conditions.ts):
```javascript
partiallytrapped: {
    onResidual(pokemon) {
        const source = this.effectState.source;
        // Gen 1 damage is 1/16, Gen 2+ damage is 1/8
        const damage = this.clampIntRange(pokemon.baseMaxhp / (this.gen >= 2 ? 8 : 16), 1);
        this.damage(damage, pokemon, source);
    }
}
```

**Fix Applied:**
Implemented on_residual in `src/data/condition_callbacks/partiallytrapped.rs`:
```rust
pub fn on_residual(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let (base_maxhp, gen) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.base_maxhp, battle.gen)
    };

    // Calculate damage: Gen 1 = 1/16 max HP, Gen 2+ = 1/8 max HP
    let divisor = if gen >= 2 { 8 } else { 16 };
    let damage = std::cmp::max(1, base_maxhp / divisor);

    battle.damage(damage, Some(pokemon_pos), None, Some(&ID::from("partiallytrapped")), false);

    EventResult::Continue
}
```

**Result:**
- ✅ Iteration #16 HP now matches: Dedenne at 107/210 in both
- ✅ Residual trap damage correctly applied (26 HP)
- ❌ PRNG calls still differ by 1:
  - JavaScript: 47->54 (7 calls)
  - Rust: 47->53 (6 calls)
- ❌ Battles diverge significantly after iteration #16 due to PRNG offset

**Commit:** c30675e8

**Next Steps:**
- Investigate missing PRNG call in turn 13
- Find what JavaScript does that Rust doesn't (or vice versa)

---

### Issue 12: partiallytrapped duration_callback not making PRNG call (FIXED ✅)

**Problem:**
- After fixing enigmaberry, iteration #16 still had PRNG divergence
  - JavaScript: prng=47->54 (7 calls)
  - Rust: prng=47->53 (6 calls)
- HP values matched (Dedenne at 107/210), but missing 1 PRNG call

**Investigation:**
Traced all PRNG calls in turn 13:
1. Spectralthief accuracy: RANDOM_CHANCE (roll=69)
2. Spectralthief crit: RANDOM_CHANCE (roll=20)
3. Spectralthief damage randomizer: RANDOM (n=16)
4. Whirlpool accuracy: RANDOM_CHANCE (roll=76)
5. Whirlpool crit: RANDOM_CHANCE  
6. Whirlpool damage randomizer: RANDOM

That's 6 calls total, but JavaScript makes 7. The missing call is related to partiallytrapped duration.

**Root Cause:**
The `duration_callback` for partiallytrapped condition was not implemented - just returned `EventResult::Continue`. JavaScript's implementation:
```javascript
partiallytrapped: {
    durationCallback(target, source) {
        if (source?.hasItem('gripclaw')) return 8;
        return this.random(5, 7);  // Returns 5 or 6
    }
}
```

This callback randomly determines trap duration (5-6 turns normally, 7-8 turns if source has Grip Claw). Rust was not making this random call.

**Fix Applied:**
Implemented duration_callback in `src/data/condition_callbacks/partiallytrapped.rs`:
```rust
pub fn duration_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // this.random(5, 7) returns a number from 5 to 6 inclusive
    let duration = battle.prng.random_range(5, 7);
    EventResult::Number(duration)
}
```

**Result:**
- ✅ Iteration #16 PRNG now matches: prng=47->54 (7 calls) in both
- ✅ **Iterations #1-#28 now match perfectly!** (PRNG fully synchronized)
- ✅ Seed 3 progressed from 15 matching iterations to 28 matching iterations
- ❌ NEW DIVERGENCE at iteration #28 (turn 22):
  - JavaScript: Kirlia at 113/195 HP
  - Rust: Kirlia at 89/195 HP
  - Difference: 24 HP (might be another missing residual damage)

**Commit:** 00e7712e

**Summary of Seed 3 Progress:**
- Issue 10: enigmaberry on_hit not executing (FIXED) → Iterations 1-15 match
- Issue 11: partiallytrapped on_residual not dealing damage (FIXED) → HP matches at iteration 16
- Issue 12: partiallytrapped duration_callback missing PRNG call (FIXED) → Iterations 1-28 match

**Next Steps:**
- Fix battle flow bug causing turn 23 to execute twice
- After forced switch, turn should increment without executing moves
- This is a core battle logic issue, not a simple callback fix

---

### Issue 14: Turn executing twice after forced switch (IN PROGRESS 🔍)

**Problem:**
- After fixing partiallytrapped source faint, iteration #30 diverges
  - JavaScript #30: turn=24, prng=106->109, Kirlia at 113/195 HP (alive)
  - Rust #30: turn=23, prng=106->113, Kirlia at 0/195 HP (faints)
- Rust makes 4 extra PRNG calls (7 vs 3) and Kirlia takes lethal damage

**Investigation:**
Turn flow analysis:
- Iteration #28: turn 22, Swirlix faints
- Iteration #29: turn 23, Seedot switches in, PRNG 106->106 (0 calls) - MATCHES JS
- Iteration #30:
  - JavaScript: turn **24**, PRNG 106->109 (3 calls), no moves in turn 23
  - Rust: turn **23**, PRNG 106->113 (7 calls), Seedot uses outrage, Kirlia faints

**Root Cause Found:**
Rust battle loop executes turn 23 MULTIPLE times:
```
>>> Turn 23 completed. PRNG: 106->106 (+0 calls)   # 1st execution: forced switch processed
>>> Making choices for turn 23...                    # Called again (BUG!)
>>> Turn 23 completed. PRNG: 106->113 (+7 calls)  # 2nd execution: moves executed
>>> Making choices for turn 23...                    # Called again (BUG!)
```

JavaScript executes turn 23 ONCE:
```
>>> Making choices for turn 23...
>>> Turn 24 completed. PRNG: 106->109 (+3 calls)  # Turn incremented to 24
```

**Expected Behavior:**
When a Pokemon faints and forces a switch:
1. Process the forced switch (choose replacement Pokemon)
2. Switch the Pokemon in
3. **Increment turn counter and END the turn** (no moves executed)
4. Next turn begins with both Pokemon making normal move choices

**Current Rust Behavior:**
1. Process the forced switch
2. Switch the Pokemon in
3. Execute moves in the SAME turn (incorrect!)
4. Turn executes multiple times before incrementing

**Impact:**
- Seedot uses outrage in turn 23 instead of turn 24
- Kirlia takes 135 damage and faints (should survive with 113 HP)
- PRNG desynchronizes (4 extra calls)
- All subsequent turns diverge

**Root Cause IDENTIFIED:**
After processing a forced switch, Rust sets mid_turn=false in turn_loop():
```rust
self.end_turn();
self.mid_turn = false;  // BUG! Should stay true after forced switch
self.queue.clear();
```

This causes the NEXT call to turn_loop() to add beforeTurn/residual actions, treating it as a NEW turn with moves to execute.

**Correct Behavior:**
After a forced switch is processed:
- mid_turn should remain TRUE
- Next turn_loop() call should NOT add beforeTurn/residual
- Should only process the switch and increment turn
- mid_turn becomes false only when a turn completes WITHOUT interruptions

**Fix Strategy:**
Need to detect if turn_loop() is being called after a forced switch and NOT set mid_turn=false in that case. Possible approaches:
1. Check if request_state was Switch at start of turn_loop()
2. Track if this turn_loop() call processed a forced switch
3. Add a flag like "processed_forced_switch"

**Status:** Root cause identified, implementing fix next 🔧

**UPDATE 2026-01-03 (Session 2):**
After extensive investigation, discovered the issue is more complex than initially thought:

1. **Both JS and Rust set mid_turn=false after forced switch**
   - Iteration #29 (forced switch) correctly processes switch
   - Calls end_turn() → turn increments 22→23
   - Sets mid_turn=false at end

2. **Turn 23 is called TWICE in Rust**
   - Iteration #29: `>>> Making choices for turn 23...` (forced switch, 0 PRNG calls)
   - Iteration #30: `>>> Making choices for turn 23...` (moves executed, 7 PRNG calls)
   - Should be: iteration #30 should be turn 24

3. **JavaScript correctly moves to turn 24**
   - Iteration #29: turn 23 (forced switch)
   - Iteration #30: turn 24 (normal moves)

4. **Test runners are identical**
   - Both use `battle.turn` for logging
   - Both call `makeChoices()` in a loop
   - No difference in loop logic

5. **Investigation needed**
   - Why does iteration #30 show turn=23 instead of turn=24?
   - If end_turn() was called in iteration #29 (logs confirm it was), turn should be 23
   - Then iteration #30 should call end_turn() again → turn=24
   - But iteration #30 shows turn=23, meaning end_turn() was NOT called
   - Likely: Kirlia faints in iteration #30, triggers forced switch, early return without end_turn()
   - But WHY does Kirlia take damage in Rust and not in JavaScript?

**Status:** Still investigating - need to understand why Kirlia takes damage in iteration #30 🔍

---

### Issue 13: partiallytrapped volatile not expiring when source faints (FIXED ✅)

**Problem:**
- After fixing duration_callback, iteration #28 still had HP divergence
  - JavaScript: Kirlia at 113/195 HP
  - Rust: Kirlia at 89/195 HP (24 HP less)
- PRNG calls matched, but HP outcome differed
- The 24 HP difference is exactly one partiallytrapped residual damage (195/8 = 24)

**Investigation:**
Traced the battle flow:
- Turn 21: Whirlpool hits Kirlia, adds partiallytrapped with duration=5
- Turn 21 Residual: duration 5→4, deals 24 damage (both JS and Rust match)
- Turn 22: Swirlix faints during the turn
- Turn 22 Residual:
  - JavaScript: Kirlia takes NO trap damage (113 HP)
  - Rust: Kirlia takes trap damage (89 HP)

**Root Cause:**
JavaScript partiallytrapped condition has source validation in onResidual:
```javascript
onResidual(pokemon) {
    const source = this.effectState.source;
    const gmaxEffect = ['gmaxcentiferno', 'gmaxsandblast'].includes(this.effectState.sourceEffect.id);
    if (source && (!source.isActive || source.hp <= 0 || !source.activeTurns) && !gmaxEffect) {
        delete pokemon.volatiles['partiallytrapped'];
        this.add('-end', pokemon, this.effectState.sourceEffect, '[partiallytrapped]', '[silent]');
        return;  // Exit WITHOUT dealing damage!
    }
    this.damage(pokemon.baseMaxhp / this.effectState.boundDivisor);
}
```

When the source Pokemon faints (hp <= 0) or is no longer active, the trap is REMOVED without dealing damage. This prevents "phantom" trap damage after the trapping Pokemon is gone.

Exception: G-Max traps (gmaxcentiferno, gmaxsandblast) continue even after source faints.

**Fix Applied:**
Updated `src/data/condition_callbacks/partiallytrapped.rs` on_residual:
1. Get source Pokemon position from volatile's EffectState
2. Check if source Pokemon exists and has hp > 0
3. If source is invalid (fainted or doesn't exist):
   - Remove partiallytrapped volatile
   - Return early WITHOUT dealing damage
4. Otherwise, deal normal residual damage
5. Pass source_pos to battle.damage() for proper attribution

**Result:**
- ✅ Iteration #28 HP now matches: Kirlia at 113/195 in both
- ✅ Trap correctly removed when Swirlix faints in turn 22
- ✅ **Iterations #1-#29 now match perfectly!** (up from #1-#28)
- ❌ NEW DIVERGENCE at iteration #30 (turn 23):
  - JavaScript: Kirlia at 113/195 HP (alive)
  - Rust: Kirlia at 0/195 HP (faints), prng=106->113 (7 calls vs 3)

**Commit:** a5191a53

**Next Steps:**
