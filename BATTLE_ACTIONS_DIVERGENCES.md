# Battle Actions Divergences - JavaScript vs Rust

This document tracks divergences between the JavaScript implementation in `pokemon-showdown/sim/battle-actions.ts` and the Rust implementation in `pokemon-showdown-rs/src/battle_actions/`.

## Status Summary

**Total files in battle_actions/**: 43
**Total TODOs/NOTEs found**: 74
**Completed implementations**: 1
**Remaining stubs**: 15

## Files with Stubs (Not Implemented)

These files are completely unimplemented stubs:

1. ~~`can_mega_evo.rs` - canMegaEvo~~ ✅ IMPLEMENTED
2. `after_move_secondary_event.rs` - afterMoveSecondaryEvent
3. `can_z_move.rs` - canZMove
4. `force_switch.rs` - forceSwitch
5. `get_active_max_move.rs` - getActiveMaxMove
6. `get_active_z_move.rs` - getActiveZMove
7. `get_max_move.rs` - getMaxMove
8. `get_z_move.rs` - getZMove
9. `hit_step_break_protect.rs` - hitStepBreakProtect
10. `hit_step_invulnerability_event.rs` - hitStepInvulnerabilityEvent
11. `hit_step_move_hit_loop.rs` - hitStepMoveHitLoop
12. `hit_step_steal_boosts.rs` - hitStepStealBoosts
13. `hit_step_try_hit_event.rs` - hitStepTryHitEvent
14. `hit_step_try_immunity.rs` - hitStepTryImmunity
15. `hit_step_type_immunity.rs` - hitStepTypeImmunity
16. `run_mega_evo.rs` - runMegaEvo

## Files with Partial Implementation

These files have implementations but with TODOs for missing functionality:

### hit_step_accuracy.rs
- Line 118: Full OHKO logic missing
- Line 173: move.target === 'self' and toxic special cases
- Line 195: Miss message and Blunder Policy handling

### get_damage.rs
- Line 358: Second condition check for move.isMax
- Line 364: NOTE about not returning early if base_power == 0

### run_move.rs
Multiple missing features:
- Line 30: activeMoveActions tracking
- Line 45: pranksterBoosted implementation
- Line 51: OverrideAction event
- Line 75: moveThisTurnResult tracking
- Line 81: cantusetwice handling
- Line 85: beforeMoveCallback
- Line 100: LockMove event
- Line 104: PP deduction verification
- Line 107: moveUsed tracking
- Line 159: lastSuccessfulMoveThisTurn
- Line 170: cantusetwice hint

## Rust-Specific Files (Not in JavaScript)

These files exist only in Rust and should be evaluated:

1. `get_boost_modifier.rs` - Helper for boost calculations
2. `new.rs` - BattleActions constructor

## Fixes Applied

### 2026-01-02 - Commit 275d04d0
**Implemented: can_mega_evo**
- Added `isMega`, `requiredMove`, and `requiredAbility` fields to SpeciesData struct
- Implemented 1:1 port of canMegaEvo from battle-actions.ts
- Exported can_mega_evo from battle_actions module
- Handles all Mega Evolution cases:
  - Mega Rayquaza (requires Dragon Ascent move)
  - Floette/Zygarde special cases
  - Array-based mega stones (Charizard X/Y)
  - Standard mega stone evolution
- Matches JavaScript implementation line by line

---

## Next Steps

1. Implement canUltraBurst and runMegaEvo to complete Mega Evolution functionality
2. Implement remaining stubbed files by porting from JavaScript battle-actions.ts
3. Complete partial implementations with missing TODOs
4. Verify Rust-specific files are necessary or can be removed
5. Ensure all implementations match JavaScript line-by-line
6. Run battle tests to verify correctness

