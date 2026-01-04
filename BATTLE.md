### Seed 2: Kingsshield Protection Investigation (IN PROGRESS 🔍)

**Current Status:** Fixed move-embedded condition callback detection. Now debugging kingsshield duration timing.

**Fixes Applied:**

1. ✅ **2-item tie shuffle bug** - Modified speed_sort.rs:83-86
   - Changed condition from `> 1` to `> 2` to skip shuffle for exactly 2 tied items
   - Matches JavaScript behavior

2. ✅ **Move-embedded condition callback detection** - Modified has_callback.rs:211-243
   - Added fallback in `condition_has_callback` to check dex.moves() for embedded conditions
   - Returns true for common move condition events (onTryHit, onStart, onHit, onEnd, etc.)
   - Allows kingsshield's onTryHit callback to be found and executed

**Current Issue: Kingsshield Duration Timing**

Kingsshield volatile has duration=1 in moves.json, causing it to expire too early:

Turn 2 execution:
- Kingsshield used, adds volatile with duration=1
- Rockthrow used (not blocked, kingsshield protects NEXT attack)
- Turn 2 Residual: duration decrements 1->0, volatile EXPIRES

Turn 3:
- NO PROTECTION (kingsshield volatile is gone!)
- Stall check makes PRNG call: 5->6
- But without protection, battle flow diverges

**Divergence:**
- JavaScript turn 3: prng=5->6, HP stays at 173/189 (protection works)
- Rust turn 3: prng=5->6, HP stays at 173/189 but then turn 4 differs

**Root Cause Found: Volatile Duration Timing**

JavaScript likely doesn't decrement volatile duration in the SAME turn the volatile is added.

Evidence:
- Kingsshield has duration=1 in moves.json (same as JavaScript)
- When used on turn 2, volatile is added with duration=1
- Rust Residual phase decrements duration 1->0 on turn 2, removes volatile
- JavaScript keeps the volatile through turn 3 (doesn't decrement on turn 2)

Test with duration=2:
- Changing kingsshield duration to 2 makes protection work
- But PRNG patterns don't match (hack, not real solution)

**Solution Needed:**
Add "created_turn" field to EffectState and skip decrement if created on current turn.
Requires changes to:
- event_system.rs: Add created_turn field to EffectState
- pokemon.rs: Set created_turn when adding volatiles
- field_event.rs: Skip decrement if volatile.created_turn == battle.turn

**Files Modified:**
- `src/battle/speed_sort.rs:83-86` - Skip shuffle for 2-item ties ✅
- `src/battle/has_callback.rs:211-243` - Check move-embedded conditions ✅
- `src/data/condition_callbacks/sandstorm.rs` - Weather callbacks (previous work) ✅
- `src/battle/spread_damage.rs` - Weather immunity (previous work) ✅

**Commits:**
- 05b3945d: Fix move-embedded condition callback detection

**Next Steps:**
1. Add created_turn tracking to EffectState
2. Skip duration decrement for same-turn volatiles
3. Verify all protection moves work correctly
