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
- Call stack shows:
  1. hitStepAccuracy (accuracy check)
  2. getDamage (critical hit check)
  3. modifyDamage (damage randomization)

**Rust (Turn 21):**
- 7 PRNG calls (calls #70, #71, #72, #73, #74, #75, #76)
- Total after turn 21: 76 calls
- Extra calls: #73, #74, #75, #76 (4 extra calls)
- All extra calls are `random(n=6)` which suggests:
  - Likely `shuffle_range()` being called
  - Shuffling 5 items makes exactly 4 PRNG calls
  - This happens in `speed_sort()` when there are ties

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

#### Current Status: DIVERGENCE PERSISTS

Despite implementing f64 speeds and fractional adjustments for EventListener, the divergence on turn 21 still occurs. This suggests:
- The fractional adjustment may need to be applied elsewhere (not just SwitchIn handlers)
- There may be a different sorting operation that's causing ties
- The ties might be in action queue sorting, not event handler sorting

### Next Steps

1. **Add detailed logging for turn 21**: Track all PRNG calls and their sources
2. **Check action speed calculation**: Verify if actions need fractional adjustments too
3. **Investigate speed_sort calls**: Find all places where speed_sort is called on turn 21
4. **Compare with JavaScript behavior**: Identify exact differences in sorting logic

## Battle Outcomes

- **JS**: Player 2 wins in 41 turns (148 PRNG calls)
- **Rust**: Player 1 wins in 47 turns (190 PRNG calls)

The divergence causes completely different battle outcomes.
