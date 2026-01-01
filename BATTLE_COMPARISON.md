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

#### Current Status: DIVERGENCE PERSISTS

**Root Cause Identified:**
- The 4 extra PRNG calls are from `Battle::random_chance()`
- Turn 20 shows 4 random_chance calls in logs, but summary says 0 PRNG calls
- Turn 21 shows 4 random_chance calls + 3 random calls = 7 total
- This suggests turn attribution issue or phase mismatch

**Hypothesis:**
- The random_chance calls might be happening at end of turn vs beginning of turn
- JavaScript might not make these random_chance calls on turn 21
- Need to identify WHAT is calling random_chance (likely Quick Claw item checks)

### Next Steps

1. **Identify source of random_chance calls**: Add stack trace logging to determine what code is calling random_chance
2. **Compare with JavaScript**: Check if JavaScript also makes 4 random_chance calls on these turns
3. **Fix turn attribution**: Ensure PRNG calls are attributed to the correct turn
4. **Verify Quick Claw timing**: Check if Quick Claw activation timing differs between JS and Rust

## Battle Outcomes

- **JS**: Player 2 wins in 41 turns (148 PRNG calls)
- **Rust**: Player 1 wins in 47 turns (190 PRNG calls)

The divergence causes completely different battle outcomes.
