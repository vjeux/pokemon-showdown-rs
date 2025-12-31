# Seed 2 Infinite Loop Investigation

## Problem
Seed 2 has only 18 PRNG calls vs JavaScript's 57 across 20 turns, with turns 2-17 having 0 calls each.

## Root Cause Found
There's an infinite loop where `turn_loop` is being called 16 times per turn instead of once.

## Call Stack Pattern
```
commit_choices → turn_loop → (something) → AUTO_CHOOSE → make_choices → commit_choices → ...
```

## Evidence
- Turn 1: `turn_loop` is called 10+ times
- Turn 2: `turn_loop` is called 10+ times
- Seed 3: Also affected (21 turn_loop calls for 2 turns)

## Key Findings

1. **Volatiles disappear between turns**
   - stall volatile added in turn 1
   - Gone by turn 2 (logs show "Checking 0 volatiles")
   - This prevents onStallMove from firing, causing 0 PRNG calls

2. **Infinite Loop Sequence**
   - `commit_choices` calls `turn_loop`
   - `turn_loop` processes actions
   - Somewhere, `AUTO_CHOOSE` is triggered
   - `AUTO_CHOOSE` → `make_choices` → `commit_choices` → repeat

3. **Not in obvious places**
   - `turn_loop` doesn't call `make_choices` or `commit_choices`
   - `run_action` doesn't call `make_choices`
   - `Battle::choose()` DOES call `commit_choices`, but `make_choices` calls `Side::choose()`
   - No recursive calls found in expected locations

## Next Steps
1. Add detailed logging to track exact call stack when AUTO_CHOOSE is triggered
2. Check if `request_state` changes are triggering re-entry
3. Look for any hidden control flow (callbacks, event handlers) that might call make_choices
4. Check if the issue is in how "default" choices are processed

## Related Files
- src/battle/turn_loop.rs - Main loop
- src/battle/commit_choices.rs - Calls turn_loop
- src/battle/make_choices.rs - Entry point
- src/side/auto_choose.rs - Where AUTO_CHOOSE logs come from
