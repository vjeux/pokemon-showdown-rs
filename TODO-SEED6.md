# Seed 6 Divergence Analysis

## Status
Partially fixed - turns 1-10 now match after Sticky Barb fix.
Remaining divergence at turn 18.

## Completed Fixes

### 1. Sticky Barb Residual Damage (FIXED ✓)
**Problem**: Sticky Barb wasn't dealing residual damage
**Root Cause**: stickybarb's on_residual callback existed but wasn't registered in dispatch_on_residual
**Fix**: Added stickybarb to the match statement in item_callbacks/mod.rs
**Result**: Turns 1-10 now match perfectly

## Remaining Issues

### 2. Sky Drop PRNG Divergence (Turn 18)
**Problem**: Turn 18 shows different PRNG call counts:
- JavaScript: 86->91 (5 calls)
- Rust: 86->93 (7 calls - 2 extra)

**Moves Executed During Turn 17** (which ends at turn 18):
1. Rock Slide (from Wormadam)
2. Sky Drop turn 2 (from Exeggcute - two-turn move)

**PRNG Call Breakdown**:

Rust:
- Rock Slide: 86->90 (4 calls)
  1. Accuracy check (90%)
  2. Crit check (1/24)
  3. Damage roll
  4. Flinch check (30%)
- Sky Drop: 90->93 (3 calls)
  1. Accuracy check (100%) ← EXTRA
  2. Crit check (1/24) ← EXTRA
  3. Damage roll
- Total: 7 calls

JavaScript:
- Rock Slide: 4 calls (same as Rust)
- Sky Drop: 1 call (damage roll only)
- Total: 5 calls

**Analysis**:
Sky Drop on turn 2 (second turn of two-turn move) appears to skip accuracy and crit checks in JavaScript, but Rust is still performing them.

**Evidence**:
1. JavaScript logs show "after crit=19" for Sky Drop, suggesting crit was determined earlier
2. Sky Drop has base accuracy 100, but on turn 2 it should use cached values
3. Two-turn moves may store accuracy/crit results from turn 1 and reuse them on turn 2

**Hypothesis**:
In JavaScript, two-turn moves like Sky Drop store the accuracy hit result and crit result from turn 1 (the charging turn) and don't re-roll them on turn 2 (the execution turn). This prevents the target from dodging or the crit status from changing between turns.

**Files to Investigate**:
- `battle-actions.ts`: Check if willCrit or similar properties are set for two-turn moves
- `hit_step_accuracy.rs`: May need to skip accuracy check for two-turn moves on turn 2
- `get_damage.rs`: May need to skip crit check for two-turn moves on turn 2

**Potential Fix**:
Add logic to skip accuracy and crit checks when:
1. Pokemon has "twoturnmove" volatile
2. Move is a two-turn move (has charge flag)
3. Duration indicates this is turn 2

**Next Steps**:
1. Verify JavaScript behavior by checking if two-turn moves set willCrit/willHit on turn 1
2. Implement similar logic in Rust to cache and reuse these values
3. Test with seed 6 to confirm fix
