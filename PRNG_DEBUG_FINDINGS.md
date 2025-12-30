# PRNG Desync Debug Findings

## Issue
Seed [0,0,0,2] has P2 HP off by 1 across all turns, indicating PRNG desynchronization.

## Root Cause Identified and Fixed
✅ **Gender Randomization** - FIXED in `src/battle/set_player.rs`
- Rust was calling `random(2)` for ALL Pokemon when gender was None
- Should only call `random(2)` if species.gender is also None/empty
- Fixed to check species.gender before randomizing
- Now correctly handles genderless Pokemon (species.gender = "N")

## Remaining Issues
❌ **Extra random(0,2) calls** - Still investigating

### PRNG Call Sequence Comparison

**JavaScript (expected):**
1. random(2) = 1 (P1 Passimian gender)
2. random(2) = 0 (P2 Passimian gender)
3. random(0,2) = 0 (insertChoice during battle.start())
4. random(0,2) = 0 (speedSort in runSwitch during battle.start())
5. random(0,2) = 0 (speedSort in eachEvent during battle.start())
6-N. More random(0,2) calls from makeChoices('default', 'default')

**Rust (current):**
1. random(2) = 1 (P1 Passimian gender) ✓
2. random(2) = 0 (P2 Passimian gender) ✓
3. random(0,2) = 0 (INSERT_RUN_SWITCH for P2) ✓
4. random(0,2) = 0 (INSERT_FIELD_ACTION for beforeTurn?) ❓
5. random(0,2) = 0 (EACH_EVENT Update speedSort) ✓
6-N. More random(0,2) calls from make_choices

### Key Differences Found

1. **INSERT_FIELD_ACTION** may be making an extra random call
   - Location: `src/battle/insert_field_action.rs` line 43
   - Called when inserting 'beforeTurn' action in turn_loop
   - Need to verify if JavaScript also does this

2. **EACH_EVENT frequency** may differ
   - Rust calls each_event multiple times with speedSort
   - Need to compare exact number of calls with JavaScript

3. **COMMIT_CHOICES timing** matches between implementations
   - Both call it from makeChoices
   - This is working correctly

## Next Steps

1. Add PRNG logging to JavaScript test-seeds.js to get exact call sequence
2. Compare call-by-call between JavaScript and Rust
3. Identify exactly which Rust calls are extra/missing/out-of-order
4. Fix the specific function(s) causing the discrepancy

## Files Modified

- ✅ `src/battle/set_player.rs` - Fixed gender randomization logic
- Added debug logging to:
  - `src/battle/shuffle_range.rs`
  - `src/battle/speed_sort.rs`
  - `src/battle_actions/run_switch.rs`
  - `src/battle/each_event.rs`
  - `src/battle/commit_choices.rs`
  - `src/battle/insert_run_switch_action.rs`
  - `src/battle/make_choices.rs`

## Test Command

```bash
node test-seeds.js
```

Stops at first mismatch (seed [0,0,0,2]).
