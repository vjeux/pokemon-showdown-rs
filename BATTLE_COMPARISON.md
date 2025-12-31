# Battle Comparison Investigation

## Infrastructure Setup ✓

- `run-battle-full-trace.js`: Runs full JS battles with PRNG tracking
- `tests/battle_state_comparison.rs`: Rust test with PRNG tracking
- `compare-traces.js`: Compares traces and identifies divergences
- `debug-turn.js`: Debug specific turns in JS with stack traces
- `debug-turn21.js`: Focused debug script for turns 19-22

## Findings for Seed 1

### First Divergence: Turn 21

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

### Turn Pattern

| Turn | JS Calls | Rust Calls | Difference | Cumulative JS | Cumulative Rust |
|------|----------|------------|------------|---------------|-----------------|
| 19   | 4        | 4          | 0 (✓)      | 69            | 69              |
| 20   | 0        | 0          | 0 (✓)      | 69            | 69              |
| 21   | 3        | 7          | +4 (✗)     | 72            | 76              |
| 22   | 0        | 3          | +3 (✗)     | 72            | 79              |

### Root Cause Hypothesis

The 4 extra `random(n=6)` calls on turn 21 strongly suggest:

1. **Speed sorting with ties**: Rust's `speed_sort()` calls `shuffle_range()` when items have equal priority
2. **Shuffle makes N-1 calls**: Shuffling 5 items makes exactly 4 PRNG calls
3. **Incorrect tie detection**: Rust is finding 5 tied items when JavaScript doesn't

Possible causes:
- Rust's `compare_priority()` function might be finding ties where JS doesn't
- Different priority calculation between JS and Rust
- Speed/priority values might differ slightly due to floating point or calculation differences
- Rust might be calling `speed_sort()` in a place where JS doesn't

### Key Code Locations

**Rust:**
- `src/battle/speed_sort.rs`: Main sorting function
- `src/battle/shuffle_range.rs`: Makes the extra PRNG calls
- `src/battle/compare_priority.rs`: Determines if items tie

**JavaScript:**
- `dist/sim/battle.js`: speedSort() function
- `dist/sim/prng.js`: shuffle() function

### Next Steps

1. **Add logging to compare_priority**: Log when Rust detects ties
2. **Compare priority values**: Check if priority/speed values match between JS and Rust
3. **Track speed_sort calls**: Log when speed_sort is called and how many ties it finds
4. **Root cause**: Find why Rust detects 5 tied items when JS doesn't
5. **Fix**: Ensure priority calculation matches JS exactly

## Battle Outcomes

- **JS**: Player 2 wins in 41 turns (148 PRNG calls)
- **Rust**: Player 1 wins in 47 turns (190 PRNG calls)

The divergence causes completely different battle outcomes.
