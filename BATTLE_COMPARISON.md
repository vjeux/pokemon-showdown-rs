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

#### Current Status: ROOT CAUSE IDENTIFIED

**JavaScript (Turn 20-21):**
- Turn 20: 0 PRNG calls (total stays at 69)
- Turn 21: 3 PRNG calls (#70-72)
  - One move executes: `p1b:visegrip`
  - Call #70: random_chance(100/100) - accuracy check
  - Call #71: random_chance(1/24) - critical hit check
  - Call #72: random(16) - damage roll

**Rust (Turn 18, self.turn=17):**
- 7 PRNG calls total (#70-76)
  - TWO moves execute in sequence:
  - `p1b:visegrip` (calls #70-72): accuracy, crit, damage
  - `p2b:psychic` (calls #73-76): accuracy, crit, damage, + random(100)

**The Problem:**
Rust executes BOTH `p1b:visegrip` AND `p2b:psychic` on the same turn (turn=17), while JavaScript only executes visegrip on turn 21.

From Rust logs:
```
RUN_ACTION Move: p1a uses knockoff
RUN_ACTION Move: p1b uses visegrip     ← Calls #70-72
RUN_ACTION Move: p2b uses psychic      ← Calls #73-76 (SHOULD NOT EXECUTE IN JS)
```

**Possible Causes:**
1. Someone faints after visegrip in JavaScript, ending the turn or battle
2. Turn boundary is handled differently - psychic might be deferred to next turn in JS
3. Action queue processing differs between JS and Rust
4. BeforeMove or other event prevents psychic execution in JS

**Next Investigation:**
Need to check JavaScript battle log to see:
1. If anyone faints after visegrip on turn 21
2. If psychic is attempted but blocked
3. If the turn ends immediately after visegrip

### Next Steps

1. **Identify source of random_chance calls**: Add stack trace logging to determine what code is calling random_chance
2. **Compare with JavaScript**: Check if JavaScript also makes 4 random_chance calls on these turns
3. **Fix turn attribution**: Ensure PRNG calls are attributed to the correct turn
4. **Verify Quick Claw timing**: Check if Quick Claw activation timing differs between JS and Rust

## Battle Outcomes

- **JS**: Player 2 wins in 41 turns (148 PRNG calls)
- **Rust**: Player 1 wins in 47 turns (190 PRNG calls)

The divergence causes completely different battle outcomes.
