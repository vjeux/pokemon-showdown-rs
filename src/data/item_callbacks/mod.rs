//! Item Callback Handlers
//!
//! This module provides dispatch functions for item callbacks.
//! Individual item implementations will be added as needed.

use crate::battle::Battle;
use crate::event::EventResult;

// =========================================================================
// DISPATCH FUNCTIONS
//
// These functions route item events to specific item implementations.
// They return EventResult directly, with EventResult::Continue for no match.
// =========================================================================

/// Dispatch onModifyAtk callbacks (read-only)
pub fn dispatch_on_modify_atk(
    _battle: &Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "choiceband" => {
            // TODO: Check if not dynamaxed
            // If not dynamaxed: return EventResult::Number(6144) // 1.5x
            EventResult::Continue
        }
        // TODO: Add lightball (Pikachu only, 2x Atk)
        // TODO: Add thickclub (Cubone/Marowak only, 2x Atk)
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifySpA callbacks (read-only)
pub fn dispatch_on_modify_sp_a(
    _battle: &Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "choicespecs" => {
            // TODO: Check if not dynamaxed
            // If not dynamaxed: return EventResult::Number(6144) // 1.5x
            EventResult::Continue
        }
        // TODO: Add deepseatooth (Clamperl only, 2x SpA)
        // TODO: Add lightball (Pikachu only, 2x SpA)
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifyDef callbacks (read-only)
pub fn dispatch_on_modify_def(
    _battle: &Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "eviolite" => {
            // TODO: Check if pokemon can still evolve (nfe = not fully evolved)
            // If can evolve: return EventResult::Number(6144) // 1.5x
            EventResult::Continue
        }
        // TODO: Add metalpowder (Ditto only, 2x Def)
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifySpD callbacks (read-only)
pub fn dispatch_on_modify_sp_d(
    _battle: &Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "assaultvest" => {
            // Assault Vest: 1.5x SpD
            EventResult::Number(6144)
        }
        "eviolite" => {
            // TODO: Check if pokemon can still evolve
            // If can evolve: return EventResult::Number(6144) // 1.5x
            EventResult::Continue
        }
        // TODO: Add deepseascale (Clamperl only, 2x SpD)
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifySpe callbacks (read-only)
pub fn dispatch_on_modify_spe(
    _battle: &Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "choicescarf" => {
            // TODO: Check if not dynamaxed
            // If not dynamaxed: return EventResult::Number(6144) // 1.5x
            EventResult::Continue
        }
        // TODO: Add quickpowder (Ditto only, 2x Spe)
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifyDamage callbacks (read-only)
pub fn dispatch_on_modify_damage(
    _battle: &Battle,
    item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match item_id {
        "lifeorb" => {
            // Life Orb: 1.3x damage
            // In 4096 basis: (4096 * 5324) / 4096 = 5324
            EventResult::Number(5324)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onSourceModifyDamage callbacks (read-only)
pub fn dispatch_on_source_modify_damage(
    _battle: &Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onBasePower callbacks (read-only)
pub fn dispatch_on_base_power(
    _battle: &Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyCritRatio callbacks (read-only)
pub fn dispatch_on_modify_crit_ratio(
    _battle: &Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onModifyAccuracy callbacks (read-only)
pub fn dispatch_on_modify_accuracy(
    _battle: &Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onAfterMoveSecondarySelf callbacks (mutates battle)
pub fn dispatch_on_after_move_secondary_self(
    _battle: &mut Battle,
    item_id: &str,
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    match item_id {
        "lifeorb" => {
            // TODO: Damage the holder by 10% of max HP
            // Check if move was damaging and source != target
            EventResult::Continue
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onResidual callbacks (mutates battle)
pub fn dispatch_on_residual(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onUpdate callbacks (mutates battle)
pub fn dispatch_on_update(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onStart callbacks (mutates battle)
pub fn dispatch_on_start(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onDamagingHit callbacks (mutates battle)
pub fn dispatch_on_damaging_hit(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onTakeItem callbacks (mutates battle)
pub fn dispatch_on_take_item(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onEat callbacks (mutates battle)
pub fn dispatch_on_eat(
    _battle: &mut Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}

/// Dispatch onSourceTryPrimaryHit callbacks (read-only)
pub fn dispatch_on_source_try_primary_hit(
    _battle: &Battle,
    _item_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    EventResult::Continue
}
