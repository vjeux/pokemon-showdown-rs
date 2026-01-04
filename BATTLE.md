### Battle Testing Status

**Current Results:**
- ⚠️ Seed 1: REGRESSION - Diverges at iteration #33, turn 27 (was fully passing before multi-hit changes)
  - Issue: 2 fewer PRNG calls in Rust, appears to "catch up" at turn 30
  - Hypothesis: Event timing difference, not actual logic error
- ❌ Seed 2: FAIL - Diverges at iteration #2, turn 3
  - Issue: kingsshield PRNG timing (1 call in JS, 0 in Rust)
- ✅ Seed 3: PASS - All iterations match ✅
- ⚠️ Seed 4: PARTIAL - First 5 iterations match, diverges at #6, turn 6
  - Issue: 1 extra PRNG call in Rust (shadowbone secondary)
  - Already deeply investigated, no resolution found
- ⚠️ Seed 5: PARTIAL - First 8 iterations match (was 0 before multi-hit fix)
  - **Diverges at iteration #9, turn 10**
  - **NEW FINDING**: Both JS and Rust use falseswipe (NOT Struggle!)
  - Turn 10 damage pattern:
    - Spoink: 233→166 (-67 HP) in JS, 233→233 (no damage) in Rust
    - Smokomodo: 198→178 (-20 HP) in both (from falseswipe)
  - falseswipe deals 20 damage correctly in both
  - **Root cause**: Spoink takes 67 self-damage in JS but not Rust
  - Missing 3 PRNG calls in Rust (56->58 vs 56->61)
  - Hypothesis: Residual/end-of-turn effect missing in Rust
  - Candidates: Z-move aftereffect, item consumption, ability trigger
  - Next: Trace JavaScript residual phase for turn 10

**Success Rate: 1/5 seeds fully passing (20%), 3/5 partial (60% total progress)**
**Net Progress from Multi-Hit Implementation: +8 iterations on seed 5, -full pass on seed 1**

**Multi-Hit Implementation Summary:**
- ✅ Fixed multihit field copying (get_active_move.rs) - major win for seed 5
- ✅ Implemented simplified multi-hit loop (try_spread_move_hit.rs)
- ✅ Added hit number tracking (active_move.hit)
- ✅ Implemented proper [2,5] range sampling with weighted distribution
- ⚠️ Seed 5 improved dramatically (0 -> 8 iterations matching)
- ⚠️ Seed 3 unaffected (still passing)
- ❌ Seed 1 regressed (full pass -> divergence at #33)
- Root cause of seed 1 regression: Unknown, may require proper hit_step_move_hit_loop.rs implementation

**Commits Made This Session:**
- de1581fb: Fix multi-hit moves - copy multihit field to ActiveMove
- 698f7677: Track hit number in multi-hit loop
- 75c3a521: Implement proper multi-hit Range sampling for [2,5]

**Seed 1 Investigation - BREAKTHROUGH:**

**Git Bisect Results:**
- ✅ Commit 177b3480 (Add created_turn tracking): Seed 1 PASSES
- ❌ Commit 16212962 (Fix confusion secondary effect): Seed 1 FAILS
- ❌ Commit de1581fb (First multi-hit commit): Seed 1 still FAILS
- ❌ Current HEAD: Seed 1 still FAILS

**Root Cause: NOT the multi-hit implementation!**
- The regression was caused by commit 16212962 "Fix confusion secondary effect application"
- This commit refactored spread_move_hit.rs by removing 55 lines of manual volatile addition
- Replaced with single call to `Pokemon::add_volatile()`
- The multi-hit commits were incorrectly blamed because they came AFTER the breaking commit

**What Changed in 16212962:**
- BEFORE: Manual volatile addition with explicit duration handling
- AFTER: Call to Pokemon::add_volatile() function
- The refactoring should have been neutral but introduced a behavioral difference

**Current Divergence:**
- Iteration #33 (turn 27): Rust makes 2 FEWER PRNG calls (4 vs 6)
  - JavaScript: prng=117->123 (6 calls during `make_choices()`)
  - Rust: prng=117->121 (4 calls during `make_choices()`)
- Moves executed: zenheadbutt (Zacian) and gmaxterror (Golem)
- HP difference: Zacian 196 (Rust) vs 195 (JS) - 1 HP difference

**Detailed Investigation:**
- Rust PRNG sequence at turn 26 (iteration #33):
  - zenheadbutt executes: 117->121 (4 calls)
    - 117->118: accuracy check
    - 118->119: crit check
    - 119->120: damage randomizer random(16)
    - 120->121: secondary chance random(100) for flinch
  - gmaxterror: BLOCKED, makes 0 PRNG calls

- JavaScript PRNG sequence at turn 26 (iteration #33):
  - zenheadbutt executes: ~117->121 (4 calls, assumed same as Rust)
  - gmaxterror executes: at least 1-2 calls
    - Makes crit check (confirmed in logs)
    - Possibly makes damage randomizer before basePower set to 0
  - Total: 117->123 (6 calls)

**Root Cause Found:**
- Rust blocks gmaxterror before it can execute and make PRNG calls
- JavaScript allows gmaxterror to start executing and makes PRNG calls (crit check minimum)
- This is likely due to different handling of flinch or move validation timing
- The blocking happens after RUN_MOVE entry but before USE_MOVE

**Next Steps:**
1. Find where Rust blocks gmaxterror (BeforeMove event? Flinch check?)
2. Determine why JavaScript allows partial execution with PRNG calls
3. Align Rust behavior to match JavaScript's move execution flow
4. May need to allow certain PRNG-generating checks before blocking moves

---

### Seed 5: Multi-Hit Move Bug (doubleironbash)

**Status:** MAJOR PROGRESS - First 6 iterations now match! ✅

**Problem:**
- doubleironbash should hit 2 times (`multihit: 2` in moves.json)
- JavaScript: deals ~14 damage (7 per hit × 2 hits)
- Rust was dealing only ~7 damage (1 hit)
- PRNG calls differed: JavaScript makes 7 calls per turn, Rust was making only 4 calls

**Root Cause:**
The `multihit` field from move data was not being copied to `active_move.multi_hit` when creating ActiveMove.
- `get_active_move.rs` line 103 had `multi_hit: None` hardcoded
- Should have been `multi_hit: move_data.multihit.clone()`

**Fixes Applied:**

1. ✅ **Copy multihit field to ActiveMove** - Modified get_active_move.rs:103
   - Changed from: `multi_hit: None,`
   - Changed to: `multi_hit: move_data.multihit.clone(),`
   - Now doubleironbash correctly has `Some(Fixed(2))`

2. ✅ **Implement simple multi-hit loop** - Modified try_spread_move_hit.rs:324-365
   - Added loop to call spread_move_hit multiple times based on num_hits
   - Accumulates damage across all hits
   - Handles Fixed(n) and Range(min, max) multihit types

**Results:**
- ✅ Iterations #1-#6 now match JavaScript exactly!
  - #1: turn=2, prng=0->7, Smokomodo: 303->289 (14 damage) ✅
  - #2: turn=3, prng=7->14, Smokomodo: 289->277 ✅
  - #3: turn=4, prng=14->21, Smokomodo: 277->263 ✅
  - #4: turn=5, prng=21->28, Smokomodo: 263->251 ✅
  - #5: turn=6, prng=28->35, Smokomodo: 251->237 ✅
  - #6: turn=7, prng=35->42, Smokomodo: 237->225 ✅
- ⚠️ Divergence starts at iteration #7 or later (different issue)

**Impact:** HIGH - This fix affects ALL multi-hit moves in the game

**Files Modified:**
- `src/dex/get_active_move.rs:103` - Copy multihit field ✅
- `src/battle_actions/try_spread_move_hit.rs:324-365` - Implement multi-hit loop ✅

**Commits:**
- de1581fb: Fix multi-hit moves - copy multihit field to ActiveMove

**Next Steps:**
1. Investigate iteration #7+ divergence (new issue unrelated to multi-hit)

---

### Seed 4: Confusion Secondary Effect Fix

**Status:** Major progress - confusion callback now working, first 5 iterations match!

**Problem:**
- dynamicpunch has `secondary.volatileStatus = "confusion"` with 100% chance
- Confusion volatile was not being applied from move secondaries
- Missing 2 PRNG calls per confusion application (1 for chance, 1 for duration)

**Root Cause:**
1. `spread_move_hit.rs` line 476 was calling `Pokemon::add_volatile()` with `None` for source_pos and move_id
2. `secondaries.rs` line 166 was also calling `Pokemon::add_volatile()` with `None` for source_pos and move_id
3. Without source_effect, `add_volatile` couldn't determine if source was "axekick" for min duration
4. confusion::on_start callback was not implemented (just TODO stub)

**Fixes Applied:**

1. ✅ **Implement confusion::on_start** - Modified confusion.rs
   - Sets random confusion duration: 2-5 turns normally, 3-5 for axekick
   - Uses `battle.random_with_range(min, 6)` to get [min, 5] inclusive
   - Checks if source_effect is "axekick" to determine min (3 vs 2)
   - Sets duration on confusion volatile's EffectState

2. ✅ **Pass source/effect to add_volatile in spread_move_hit.rs** - Line 476
   - Changed from: `Pokemon::add_volatile(battle, target_pos, volatile_id, None, None, None, None)`
   - Changed to: `Pokemon::add_volatile(battle, target_pos, volatile_id, Some(source_pos), Some(move_id), None, None)`
   - This allows confusion::on_start to access source_effect for axekick check

3. ✅ **Pass source/effect to add_volatile in secondaries.rs** - Line 166-167
   - Changed from: `Pokemon::add_volatile(battle, target_pos, volatile_id, None, None, None, None)`
   - Changed to: `Pokemon::add_volatile(battle, target_pos, volatile_id, Some(source_pos), Some(move_id), None, None)`
   - Ensures source and move context are available for all volatile additions

**Results:**
- ✅ Iterations #1-#5 now match JavaScript exactly (PRNG calls, HP values, all match)
- ⚠️ Divergence starts at iteration #6 (turn 6)
  - JavaScript: `prng=28->36` (8 calls)
  - Rust: `prng=28->37` (9 calls)
  - 1 extra PRNG call in Rust - different issue from confusion

**Impact:**
- confusion::on_start callback is now being called correctly
- PRNG calls for confusion duration are happening
- Source effect context is properly passed through add_volatile chain
- Major step forward for seed 4 accuracy

**Files Modified:**
- `src/battle_actions/spread_move_hit.rs:476` - Pass source/effect to add_volatile ✅
- `src/battle_actions/secondaries.rs:166-167` - Pass source/effect to add_volatile ✅
- `src/data/condition_callbacks/confusion.rs:26-73` - Implement on_start callback ✅
- `src/battle/dispatch_single_event.rs:42-48` - Add logging for debugging ✅

**Commits:**
- 16212962: Fix confusion secondary effect application

**Next Steps for Seed 4:**
1. **Investigate turn 6 divergence (1 extra PRNG call in Rust) - IN PROGRESS**
   - JavaScript: `prng=28->36` (8 calls total)
   - Rust: `prng=28->37` (9 calls total)
   - Breakdown:
     - dynamicpunch: 5 calls (28->33) - matches between JS and Rust
       - 28->29: accuracy check
       - 29->30: crit check
       - 30->31: damage roll
       - 31->32: secondary chance roll (confusion)
       - 32->33: confusion duration (on_start callback)
     - shadowbone: JS makes 3 calls (33->36), Rust makes 4 calls (33->37)
       - 33->34: accuracy check
       - 34->35: crit check
       - 35->36: damage roll
       - 36->37: secondary chance roll (Defense drop) - **EXTRA IN RUST**
   - HP divergence: Flabébé at 129/173 (JS) vs 131/173 (Rust) suggests damage calculation difference

   **Investigation findings:**
   - Added comprehensive logging to spread_move_hit.rs for secondaries processing
   - Confirmed Rust is processing shadowbone's secondary effect correctly:
     - filtered_targets has 1 valid target
     - ModifySecondaries event runs but doesn't filter secondaries
     - random(100) call is made unconditionally
     - Secondary roll=95, chance=20%, so effect doesn't apply
   - JavaScript secondaries() function (battle-actions.ts:1351-1367) also calls random(100) unconditionally for each secondary
   - No ability/item filters secondaries:
     - P1 Lickitung: Own Tempo, item=tr32
     - P2 Flabébé: Flower Veil, item=galaricacuff
     - Flower Veil only affects Grass-types, Flabébé is Fairy-type
   - **MYSTERY**: Both implementations appear to have identical logic, yet JavaScript makes 1 fewer PRNG call
   - Hypothesis: May be a JavaScript bug/quirk or undocumented behavior that skips secondary rolls in specific conditions
   - **TODO**: Need to instrument JavaScript code to trace exact PRNG sequence to identify root cause

2. Once turn 6 is fixed, continue fixing subsequent divergences

---

### Seed 2: Kingsshield Protection Investigation

**Status:** Partial fix - protection works but PRNG divergence remains

**Fixes Applied:**

1. ✅ **2-item tie shuffle bug** - Modified speed_sort.rs:83-86
   - Changed condition from `> 1` to `> 2` to skip shuffle for exactly 2 tied items
   - Matches JavaScript behavior

2. ✅ **Move-embedded condition callback detection** - Modified has_callback.rs:211-243
   - Added fallback in `condition_has_callback` to check dex.moves() for embedded conditions
   - Returns true for common move condition events (onTryHit, onStart, onHit, onEnd, etc.)
   - Allows kingsshield's onTryHit callback to be found and executed

3. ✅ **Volatile duration timing** - Modified event_system.rs, add_volatile.rs, field_event.rs
   - Added `created_turn: Option<i32>` field to EffectState
   - Set created_turn when adding volatiles
   - Skip duration decrement in Residual phase if volatile.created_turn == battle.turn
   - Matches JavaScript behavior where volatiles don't decrement on the same turn they're added
   - Protection now works correctly (HP stays at 173/189)

4. ✅ **random_chance order** - Modified random_chance.rs:12-21
   - Fixed to check `force_random_chance` BEFORE calling `prng.random_chance()`
   - Prevents extra PRNG calls when forced value is set
   - Matches JavaScript: `if (this.forceRandomChance !== null) return this.forceRandomChance;`

**Remaining Issue: Iteration #2 PRNG Divergence**

JavaScript iteration #2 makes 1 PRNG call (5->6) while Rust makes 0 calls (5->5).
- Both show turn=3, same HP values
- Stall check happens in Rust iteration #3 (when stall exists from prev turn)
- JavaScript presumably makes PRNG call from a different source on iteration #2
- Investigation inconclusive - may be JavaScript-specific feature or Rust missing implementation

**Impact:** Minor - protection works correctly, only PRNG alignment differs

**Files Modified:**
- `src/battle/speed_sort.rs:83-86` - Skip shuffle for 2-item ties ✅
- `src/battle/has_callback.rs:211-243` - Check move-embedded conditions ✅
- `src/event_system.rs:123-128` - Add created_turn field to EffectState ✅
- `src/pokemon/add_volatile.rs:236-254` - Set created_turn when adding volatiles ✅
- `src/battle/field_event.rs:425-456` - Skip same-turn duration decrement ✅
- `src/battle/random_chance.rs:12-21` - Fix force_random_chance check order ✅

**Commits:**
- 05b3945d: Fix move-embedded condition callback detection
- 928fdc2b: Add created_turn tracking
- 177b3480: Add created_turn tracking for volatiles and fix random_chance order

---

### Next Steps

1. Investigate seed 4 failure
2. Investigate seed 5 failure
3. Continue improving battle accuracy across more seeds
4. Optional: Deep-dive into seed 2 PRNG divergence if it becomes blocking
