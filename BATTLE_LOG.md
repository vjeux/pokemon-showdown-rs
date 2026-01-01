# Battle Testing Log

This file documents fixes made to synchronize JavaScript and Rust battle implementations.

## 2026-01-01: Team Generation Synchronization

### Issue
Team generation differed between JavaScript and Rust, preventing battle comparison tests from running.

**Seed 123 differences:**
- Line 100: Vivillon-Sandstorm ability: JS "Shield Dust" vs Rust "No Ability"
- Line 137: Move name: JS "hiddenpower" vs Rust "hiddenpowerpsychic"

### Root Causes

1. **Hidden Power moves**: JavaScript's `Dex.moves.all()` returns normalized move IDs where all Hidden Power variants have ID "hiddenpower". Rust uses raw dex keys with distinct IDs like "hiddenpowerpsychic".

2. **Cosmetic forme abilities**: Species like Vivillon-Sandstorm are cosmetic formes without their own ability definitions. They should inherit from their base species (Vivillon).

### Fixes

**JavaScript (tests/generate-test-teams.js):**
- Changed from `Dex.moves.all().map(m => m.id)` to `Object.keys(Dex.data.Moves)` to get raw move IDs
- Added logic to check `baseSpecies` for abilities when cosmetic forme has none

**Rust (src/team_generator.rs):**
- Added logic to check `base_species` for abilities when cosmetic forme has none

### Result
✅ Team generation now matches exactly between JS and Rust.

### Next Issue
Seed 123 now has a battle divergence starting at turn 5:
- JS: Tinkaton(296/316) - took 20 damage
- Rust: Tinkaton(299/316) - took 17 damage
- PRNG calls match (17->20), so event flow is correct
- Damage calculation difference of 3 HP per turn

---

## 2026-01-01: PRNG Stack Trace Debugging System

### Implementation
Created a comprehensive PRNG tracing system to debug divergences:

**Features:**
- Configurable via `TRACE_PRNG` environment variable
- Supports ranges (1-5), individual calls (10), and lists (1,5,10)
- Shows stack traces for specific PRNG calls in both JS and Rust
- New script: `tests/trace-prng.sh` for easy debugging
- Updated comparison script to support PRNG tracing
- Complete documentation in `PRNG_TRACING.md`

**Usage examples:**
```bash
# Trace PRNG call #1
./tests/trace-prng.sh 100 1

# Trace calls 1-5
./tests/trace-prng.sh 100 1-5

# Trace in comparison test
./tests/compare-battles.sh 100 1-5
```

### Seed 100 Root Cause Found

Using the new tracing system, identified why Rust makes 0 PRNG calls on turn 2 while JS makes 1:

**JavaScript (turn 2 Residual event):**
- Has 3 handlers: pickup (order=28), stall (order=undefined), spikyshield (order=undefined)
- All have same speed=119, priority=0, subOrder=2
- pickup sorts first (order=28 < undefined)
- stall and spikyshield tie (both order=undefined) → shuffled → 1 PRNG call

**Rust (turn 2 Residual event):**
- Has only 2 handlers: spikyshield, pickup
- **Missing "stall" volatile** from Linoone-Galar
- pickup shows order=None (should be order=28 from onResidualOrder)
- No ties → no shuffle → 0 PRNG calls

**Issues identified:**
1. Rust is missing "stall" volatile - need to investigate why it's not being added
2. Rust's default order (i32::MAX) differs from JavaScript's (4294967296 = 2^32)
3. Rust's get_callback_order not reading pickup's onResidualOrder=28

**Status:** Root cause identified, fixes pending

---

## 2026-01-01: Display Name and Comparison Script Fixes

### Issue
Test comparison was failing due to:
1. Pokemon name display differences (JS: "Slowbro" vs Rust: "Slowbro-Mega")
2. Comparison script including header lines that differ

### Fixes

**examples/test_battle_rust.rs:**
- Strip forme suffixes using `split('-').next()` for display
- Now shows "Slowbro" instead of "Slowbro-Mega" to match JavaScript

**tests/compare-battles.sh:**
- Changed grep pattern from '^#' to '^#[0-9]'
- Only compares numbered battle state lines, excludes headers

### Results
- ✅ Seed 1: PASS (perfect match!)
- ❌ Seed 42: FAIL (PRNG diverges from turn 1)
- ❌ Seed 100: FAIL (battle logic divergence)
- ❌ Seed 123: FAIL (damage calculation divergence from turn 5)

### Analysis
- Display/comparison issues are resolved
- Seed 1 passes completely - provides a working baseline
- Remaining failures are actual battle logic bugs
- Need to investigate: damage calculations, stat modifiers, or move implementations

---

## 2026-01-01: willCrit Field Implementation

### Issue
Moves with guaranteed critical hits (like Wicked Blow) were not properly applying crits.

**Seed 42 divergence:**
- JS turn 1: prng=0->6
- Rust turn 1: prng=0->7 (extra PRNG call)

### Root Cause
`MoveData` struct was missing the `willCrit` field from JavaScript move data. When `get_active_move` created an `ActiveMove`, it always set `will_crit: None`, causing the crit logic to roll for a random crit instead of using the guaranteed crit from the move data.

### Fix
- Added `will_crit: Option<bool>` to `MoveData` struct in `src/dex.rs`
- Updated `get_active_move` to copy `will_crit` from move_data
- Now moves like Wicked Blow (`"willCrit": true`) properly guarantee critical hits

### Results
- ✅ Seed 1: Still passes (no regression)
- ⚠️ Seed 42: Improved! First 4 turns now match exactly
  - Divergence moved from turn 1 to turn 4-5
  - New issue: Different damage/faint timing (needs investigation)

---

## 2026-01-01: 16-bit Truncation Fix in modifyDamage

### Issue
JavaScript's modifyDamage returns `tr(baseDamage, 16)` which applies a 16-bit truncation mask, but Rust was calling `trunc(final_damage, None)` without the mask.

### Fix
- Updated `src/battle_actions/modify_damage.rs` line 236 to use `battle.trunc(final_damage as f64, Some(16))`
- Now matches JavaScript's `return tr(baseDamage, 16);` exactly

### Results
- ✅ Seed 1: Still passes perfectly
- ⚠️ Seed 42: Improved again! Now matches through turn 4 (was turn 4-5 divergence)
  - JS: Eelektrik faints turn 4
  - Rust: Eelektrik survives to turn 5 (new issue)
- ❌ Seed 123: Still has 3 HP damage differences (different root cause)  
- ❌ Seed 100: Still has divergences

### Analysis
The 16-bit trunc fixed seed 42's turn 4-5 issue but revealed a new faint detection problem. Seed 123's damage differences persist, suggesting a different calculation issue (not related to final truncation).

---
## 2026-01-01: Seed 100 and Seed 123 Investigation

### Seed 100 Issue
**Problem:** Rust makes one fewer PRNG call than JavaScript from turn 1
- JS turn 1: prng=0->1 (1 call)
- Rust turn 1: prng=0->0 (0 calls)
- This off-by-one persists throughout the entire battle

**Analysis:**
- Team: Linoone-Galar (Pickup) vs Raging Bolt (Protosynthesis)
- Protosynthesis ability is completely unimplemented in Rust (all callbacks return EventResult::Continue)
- The ability should trigger WeatherChange event at battle start, which likely accounts for the missing PRNG call

**Status:** Needs Protosynthesis implementation

### Seed 123 Issue
**Problem:** PRNG calls match perfectly, but damage calculations differ by 2-3 HP per turn
- Example turn 5: JS deals 20 damage, Rust deals 17 damage
- Differences vary: sometimes 0 HP, sometimes 2-3 HP

**Analysis:**
- Investigated type effectiveness calculation - found it working correctly
- Type chart verified: Grass vs Fairy is neutral (1x), Grass vs Steel is 0.5x
- `get_type_effectiveness_mod` correctly iterates through both types
- Tinkaton has "powertrick" volatile (swaps Attack/Defense stats)
- Base damage calculation and randomizer appear correct

**Status:** Still investigating - damage formula differences not yet identified

---

## 2026-01-01: Protosynthesis Ability Implementation

### Implementation
Implemented all Protosynthesis ability callbacks:

**Main ability callbacks:**
- `on_start`: Triggers WeatherChange event
- `on_weather_change`: Adds/removes protosynthesis volatile based on sunny day
- `on_end`: Removes volatile and adds silent end message

**Condition (volatile) callbacks:**
- `on_start`: Calculates best stat and displays activation message
- `on_modify_atk/def/spa/spd`: Applies 1.3x boost (5325/4096) if best stat matches
- `on_modify_spe`: Applies 1.5x boost (3/2 ratio) if Spe is best stat
- `on_end`: Displays end message

### Challenges
- Borrow checker issues required inline calculation of best stat instead of calling `pokemon.get_best_stat()`
- ID type conversion needed for volatile management
- Arg import path needed correction

### Results
- ✅ Code compiles successfully
- ❌ Seed 100: Still shows PRNG off-by-one (0->0 vs 0->1 per turn)
  - Ability callbacks are registered in has_callback
  - Need to investigate why PRNG calls aren't happening

**Status:** Implementation complete but not yet fixing the PRNG divergence

---


---

## 2026-01-01: Seed 100 PRNG Divergence Investigation

### Issue
Seed 100 shows PRNG divergence starting at turn 5:
- JavaScript: turn 5, prng=4->5 (1 call)
- Rust: turn 5, prng=4->6 (2 calls) - ONE EXTRA CALL

### Investigation

**Key Finding:** Rust's onPrepareHit event routing is correct - the move's handler IS being called.

**Added debug logging in `dispatch_single_event.rs`:**
```rust
if event_id == "PrepareHit" {
    eprintln!("[DISPATCH_SINGLE_EVENT] PrepareHit check: checking if {} is a move", effect_id);
    if let Some(_move_def) = self.dex.moves().get(effect_id.as_str()) {
        eprintln!("[DISPATCH_SINGLE_EVENT] PrepareHit: Calling handle_move_event for move");
        return self.handle_move_event(event_id, effect_str, target);
    } else {
        eprintln!("[DISPATCH_SINGLE_EVENT] PrepareHit: {} not found in moves dex", effect_id);
    }
}
```

**Pattern Discovered:**
- Rust makes StallMove PRNG calls on turns 2, 4, 6, 8...  
- JavaScript makes StallMove calls on turns 3, 4, 6, 8...
- Pattern is offset by one turn on initial occurrence

**Root Cause:** The `stall` volatile is being managed differently in terms of when it's first added or how duration is tracked. When Spiky Shield is used multiple turns in a row, the stall volatile's duration decrements at end of turn. If duration reaches 0, volatile is removed. The first StallMove check with an active stall volatile triggers the PRNG call.

**Status:** Complex timing issue - needs deeper investigation of volatile duration management and turn structure. The move callbacks are working correctly; the issue is in the lifecycle of the stall volatile.

