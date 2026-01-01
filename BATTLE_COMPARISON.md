# Battle Comparison Investigation

## Infrastructure Setup ✓

- `run-battle-full-trace.js`: Runs full JS battles with PRNG tracking
- `tests/battle_state_comparison.rs`: Rust test with PRNG tracking
- `compare-traces.js`: Compares traces and identifies divergences
- `debug-turn.js`: Debug specific turns in JS with stack traces
- `debug-turn21.js`: Focused debug script for turns 19-22

## Findings for Seed 1

### First Divergence: Turn 21 (ONGOING)

**JavaScript (Turn 21):**
- 3 PRNG calls (calls #70, #71, #72)
- Total after turn 21: 72 calls

**Rust (Turn 21):**
- 7 PRNG calls (calls #70-76)
- Total after turn 21: 76 calls
- Extra calls: 4

### Investigation Progress

#### Fixes Applied:
1. ✓ Changed `PriorityItem.speed` from `i32` to `f64` (src/battle.rs:596)
2. ✓ Changed Action speeds to `f64`:
   - `MoveAction.speed` (src/battle_queue.rs:58)
   - `SwitchAction.speed` (src/battle_queue.rs:108)
   - `TeamAction.speed` (src/battle_queue.rs:132)
   - `PokemonAction.speed` (src/battle_queue.rs:168)
3. ✓ Changed `EventListener.speed` from `Option<i32>` to `Option<f64>` (src/battle.rs:214)
4. ✓ Implemented fractional speed adjustment for SwitchIn event handlers (src/battle/resolve_priority.rs:201-216)
5. ✓ Updated all comparison functions to use `total_cmp()` for f64
6. ✓ Fixed direct PRNG wrapper bypasses:
   - psywave.rs: Changed `battle.prng.random_range()` to `battle.random_with_range()`
   - quickclaw.rs: Changed `battle.prng.random_chance()` to `battle.random_chance()`

#### Current Status: on_effect parameter fix COMPLETED - New divergence on Turn 27

**Previous Root Cause (FIXED):**
Rust's `run_event` function was missing the `on_effect` parameter that JavaScript uses to determine whether to call the sourceEffect (move/item/ability)'s event handler.

**The Fix (COMPLETED):**
1. ✓ Renamed main `run_event` to `run_event_internal` with `on_effect: bool` parameter
2. ✓ Implemented logic to add sourceEffect handler when `on_effect` is true
3. ✓ Updated get_damage.rs to use `run_event_with_effect` for BasePower event
4. ✓ Created public wrapper functions (`run_event` and `run_event_with_effect`)
5. ✓ Code compiles successfully

**Result:**
- Turns 1-26 now match between JS and Rust!
- First divergence moved from Turn 21 to Turn 27
- Turn 27: JS has 4 PRNG calls, Rust has 7 (3 extra calls)

**Next Steps:**
- Investigate what's happening on Turn 27 that causes the divergence
- Check battle log to identify which move/action is behaving differently

### Previous Investigation

**JavaScript Turn 21:**
```
|move|p1a: Genesect|Vise Grip|p2a: Cinderace
|-damage|p2a: Cinderace|0 fnt
|faint|p2a: Cinderace      ← CINDERACE FAINTS!
```
- Genesect uses Vise Grip on Cinderace
- Cinderace takes fatal damage and faints
- Turn ends, psychic is never executed
- PRNG calls: #70 (accuracy), #71 (crit), #72 (damage roll)

**Rust Turn 21 (turn=17):**
- Genesect uses Vise Grip on Cinderace
- Cinderace survives (INCORRECT!)
- Cinderace then uses Psychic
- PRNG calls: #70-72 (visegrip), #73-76 (psychic)

**HP Divergence Found on Turn 17:**
- JavaScript: Cinderace 206 → 122 HP (lost 84 HP from Knock Off)
- Rust: Cinderace 206 → 149 HP (lost 57 HP from Knock Off)
- Difference: 27 HP

**Damage Calculation Comparison (Turn 17 Knock Off):**
- JavaScript: base_damage=90, roll=6, result=84
- Rust: base_damage=61, roll=6, result=57
- Missing: 1.5x boost (90/61 = 1.475 ≈ 1.5)

### Next Steps

1. **Identify source of random_chance calls**: Add stack trace logging to determine what code is calling random_chance
2. **Compare with JavaScript**: Check if JavaScript also makes 4 random_chance calls on these turns
3. **Fix turn attribution**: Ensure PRNG calls are attributed to the correct turn
4. **Verify Quick Claw timing**: Check if Quick Claw activation timing differs between JS and Rust

## Battle Outcomes

- **JS**: Player 2 wins in 41 turns (148 PRNG calls)
- **Rust**: Player 1 wins in 47 turns (190 PRNG calls)

The divergence causes completely different battle outcomes.
