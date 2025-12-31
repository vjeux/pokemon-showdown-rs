# Seed 2 PRNG Divergence Investigation

## Problem
Seed 2 has only 18 PRNG calls vs JavaScript's 57 across 20 turns.
- Turn 1: 5 calls (both) ✓
- Turn 2: 0 calls (Rust) vs 1 call (JS) ❌ **FIRST DIVERGENCE**
- Turn 3: 1 call (both) ✓
- Turns 4-17: 0 calls (Rust) vs 1+ calls each (JS) ❌

## ROOT CAUSE IDENTIFIED

**Bug**: In turn 2, `run_event("StallMove")` returns a result WITHOUT calling `find_pokemon_event_handlers`.

### Evidence:

**Turn 1 (first Kings Shield use):**
```
[KINGSSHIELD::ON_PREPARE_HIT] Calling run_event StallMove
[FIND_POKEMON_HANDLERS] event_id=onStallMove, target=(0, 0)
[FIND_POKEMON_HANDLERS] Checking 0 volatiles for Sandaconda  ← stall not created yet
[KINGSSHIELD::ON_PREPARE_HIT] StallMove result: Some(1)
...later...
[KINGSSHIELD::ON_HIT] Added 'stall' volatile. Total volatiles: 1  ← created here
```

**Turn 2 (second Kings Shield use):**
```
[KINGSSHIELD::ON_PREPARE_HIT] Calling run_event StallMove
[KINGSSHIELD::ON_PREPARE_HIT] StallMove result: Some(0)  ← NO find_pokemon_event_handlers called!
[KINGSSHIELD::ON_PREPARE_HIT] Returning false
```

**Turn 3 (third Kings Shield use):**
```
[KINGSSHIELD::ON_PREPARE_HIT] Calling run_event StallMove
[FIND_POKEMON_HANDLERS] event_id=onStallMove, target=(0, 0)
[FIND_POKEMON_HANDLERS] Checking 2 volatiles for Sandaconda
[FIND_POKEMON_HANDLERS] Volatile 'stall' condition_has_callback(onStallMove)=true  ← found!
[STALL] Success chance: 33%
[RANDOM_CHANCE] Called with 1/3  ← PRNG call made!
[STALL] randomChance(1, 3) = false
[STALL] Removed stall volatile (failed)
```

## Analysis

1. **Turn 2 is missing find_pokemon_event_handlers call**
   - run_event("StallMove") is called
   - But it returns Some(0) immediately without checking for handlers
   - This means no randomChance() call is made
   - Stall volatile exists at this point but is not checked

2. **Possible causes**:
   - run_event has a cache that's incorrectly returning a cached result
   - There's a short-circuit path in run_event that skips handler lookup
   - The event system has state that prevents re-execution

3. **Impact**:
   - Turn 2 misses the stall check → 0 PRNG calls instead of 1
   - All subsequent turns with Kings Shield are affected similarly
   - This cascades to create the 18 vs 57 divergence

## Next Steps

1. **PRIORITY**: Investigate `run_event` implementation
   - Check for caching mechanisms
   - Look for short-circuit logic
   - Verify handler lookup is always performed

2. Compare with JavaScript's `runEvent()` to find 1-to-1 differences

3. Test fix and verify turn 2 makes exactly 1 PRNG call

## Related Files
- src/battle/run_event.rs - Event system (likely culprit)
- src/battle/find_pokemon_event_handlers.rs - Handler lookup
- src/data/move_callbacks/kingsshield.rs - Kings Shield onPrepareHit
- src/data/condition_callbacks.rs - Stall's onStallMove (lines 488-545)
