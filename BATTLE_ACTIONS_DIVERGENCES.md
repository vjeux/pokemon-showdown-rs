# Battle Actions Divergences - JavaScript vs Rust

This document tracks divergences between the JavaScript implementation in `pokemon-showdown/sim/battle-actions.ts` and the Rust implementation in `pokemon-showdown-rs/src/battle_actions/`.

## Status Summary

**Total files in battle_actions/**: 43
**Total TODOs/NOTEs found**: 74
**Completed implementations**: 6 (can_mega_evo, can_ultra_burst, run_mega_evo, get_z_move, can_z_move, get_active_z_move)
**Remaining stubs**: 11

## Files with Stubs (Not Implemented)

These files are completely unimplemented stubs:

1. ~~`can_mega_evo.rs` - canMegaEvo~~ ✅ IMPLEMENTED
2. ~~`can_ultra_burst.rs` - canUltraBurst~~ ✅ IMPLEMENTED (Converted to standalone function)
3. ~~`run_mega_evo.rs` - runMegaEvo~~ ✅ IMPLEMENTED
4. ~~`can_z_move.rs` - canZMove~~ ✅ IMPLEMENTED
5. ~~`get_z_move.rs` - getZMove~~ ✅ IMPLEMENTED
6. ~~`get_active_z_move.rs` - getActiveZMove~~ ✅ IMPLEMENTED
7. `after_move_secondary_event.rs` - afterMoveSecondaryEvent
8. `force_switch.rs` - forceSwitch
9. `get_active_max_move.rs` - getActiveMaxMove
10. `get_max_move.rs` - getMaxMove
11. `hit_step_break_protect.rs` - hitStepBreakProtect
12. `hit_step_invulnerability_event.rs` - hitStepInvulnerabilityEvent
13. `hit_step_move_hit_loop.rs` - hitStepMoveHitLoop
14. `hit_step_steal_boosts.rs` - hitStepStealBoosts
15. `hit_step_try_hit_event.rs` - hitStepTryHitEvent
16. `hit_step_try_immunity.rs` - hitStepTryImmunity
17. `hit_step_type_immunity.rs` - hitStepTypeImmunity

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

### 2026-01-02 - Commit c8cd8c9f
**Updated: can_ultra_burst**
- Converted from BattleActions method to standalone function
- Matches new function pattern: `(battle: &Battle, side_index: usize, pokemon_index: usize) -> Option<String>`
- Maintains 1:1 equivalence with JavaScript implementation

### 2026-01-02 - Commit 5e72cfc6
**Implemented: run_mega_evo**
- Implemented 1:1 port of runMegaEvo from battle-actions.ts
- Calls can_mega_evo and can_ultra_burst to determine forme
- Performs forme change (using current forme_change method)
- Limits one mega evolution per side
- Fires AfterMega event
- Note: Current forme_change method signature doesn't match JavaScript
  - JavaScript: `formeChange(speciesId, source, isPermanent, abilitySlot, message)`
  - Rust: `forme_change(new_species_id, new_types, new_ability)`
  - Full 1:1 forme_change rewrite needed in future

### 2026-01-02 - Commit 54ce0cd2
**Implemented: get_z_move**
- Implemented 1:1 port of getZMove from JavaScript battle-actions.ts
- Handles Z-crystals for specific moves (e.g., Pikashunium Z)
- Handles generic type-based Z-crystals (e.g., Normalium Z)
- Returns status move name for Status category moves
- Returns typed Z-move name for damaging moves
- Uses item.extra field to access zMoveFrom and zMoveType
- Note: ItemData fields not properly typed (using serde_json::Value)

### 2026-01-02 - Commit 140875e0
**Implemented: can_z_move**
- Implemented 1:1 port of canZMove from JavaScript battle-actions.ts
- Returns Vec<Option<ZMoveOption>> with Z-moves for each move slot
- Checks side.zMoveUsed flag
- Prevents Z-moves for transformed Mega/Primal/Ultra formes
- Verifies item compatibility with itemUser
- Checks move PP availability
- Adds "Z-" prefix for status moves
- Note: Species.isPrimal field doesn't exist - using name pattern check

### 2026-01-02 - Commit bbdb0b7a
**Implemented: get_active_z_move**
- Implemented 1:1 port of getActiveZMove from JavaScript battle-actions.ts
- Returns ActiveMove for Z-Moves with appropriate properties
- Handles specific Z-moves from items (Pikashunium Z, etc.)
- Handles status move Z-moves (returns move with Z-flags)
- Handles damaging Z-moves (creates typed Z-move with custom power)
- Copies category and priority from base move
- Added helper functions:
  - move_data_to_active_move() - Convert MoveData to ActiveMove
  - move_data_flags_to_active_flags() - Convert move flags
- Note: Full dex.getActiveMove() method should be implemented

---

## Next Steps

1. ~~Implement canUltraBurst and runMegaEvo to complete Mega Evolution functionality~~ ✅ COMPLETED
2. ~~Implement Z-Move functions (getZMove, canZMove)~~ ✅ COMPLETED
3. Implement remaining stubbed files by porting from JavaScript battle-actions.ts (12 remaining)
4. Complete partial implementations with missing TODOs
5. Add proper typing for ItemData (z_move, zMoveFrom, zMoveType fields)
6. Add isPrimal field to SpeciesData
7. Rewrite forme_change to match JavaScript 1:1
8. Verify Rust-specific files are necessary or can be removed
9. Ensure all implementations match JavaScript line-by-line
10. Run battle tests to verify correctness

