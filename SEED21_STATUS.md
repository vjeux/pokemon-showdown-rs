# Seed 21 Status

## Progress
- ✅ Fixed: Turn increment issue (turn 32->33 mismatch)
- ✅ Fixed: switch_flag not being cleared when Pokemon switches out
- ✅ Fixed: CRITICAL infrastructure bug - effectState modifications were being lost
- ⚠️ Remaining: New divergence at iteration 45 (turn 40)

## Fixes Applied

### 1. switch_flag clearing (commit 9143ec7d)
- When a Pokemon switches out, its switch_flag is now cleared
- This prevents false positives when the Pokemon switches back in later
- Fixes the issue where Chuggon's old "partingshot" switch_flag from iteration 8 was still set in iteration 36

### 2. effectState infrastructure fix (commit 6afbdee8) - CRITICAL
**Root cause**: In JavaScript, `this.effectState` is a REFERENCE to `this.volatiles[status.id]`. In Rust, we were CLONING it, so all modifications to `battle.current_effect_state` were lost when the callback returned.

**Infrastructure fix** in `dispatch_single_event.rs`:
- After calling the condition callback, copy modified `current_effect_state` back to `pokemon.volatiles`
- This fixes ALL condition callbacks that need to modify effectState (trueDuration, move, etc.)

**Fixes** in `lockedmove.rs`:
- `on_start`: Use `current_effect_state` instead of `pokemon.volatiles`
- `on_start`: Get move ID from `source_effect` or `current_effect`
- `on_residual`: Read/write from `current_effect_state`
- `on_restart`: Use `current_effect_state`
- `on_end`: Use `current_effect_state` with fallback
- `on_lock_move`: Use `current_effect_state` with fallback

This fix affects all volatiles that use effectState data (confusion duration, lockedmove trueDuration, etc.)

## Current Divergence
Iteration 45 (Turn 40):
- JavaScript: prng=87->92 (5 calls), Weepinbell 28/174, Grovyle 85/225
- Rust: prng=87->89 (2 calls), Weepinbell 104/174, Grovyle 85/225
- JavaScript makes 3 extra PRNG calls
- Weepinbell takes different damage

## Next Steps
1. Investigate iteration 45 divergence
2. Check what moves are being used at turn 40
3. Compare damage calculations and PRNG call patterns
