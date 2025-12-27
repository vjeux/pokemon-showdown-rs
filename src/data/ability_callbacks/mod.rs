//! Ability Callback Handlers
//!
//! This module provides dispatch functions for ability callbacks.
//! Individual ability implementations will be added as needed.

use crate::battle::Battle;
use crate::event::EventResult;

// =========================================================================
// DISPATCH FUNCTIONS
//
// These functions route ability events to specific ability implementations.
// They return EventResult directly, with EventResult::Continue for no match.
// =========================================================================

/// Dispatch onModifySTAB callbacks (read-only)
pub fn dispatch_on_modify_stab(
    _battle: &Battle,
    ability_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "adaptability" => {
            // Adaptability changes STAB from 1.5x to 2x
            // Modifier: 2.0/1.5 = 4/3 = 1.333...
            // In 4096 basis: (4096 * 4) / 3 = 5461
            EventResult::Number(5461)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onSwitchIn callbacks (mutates battle)
pub fn dispatch_on_switch_in(
    _battle: &mut Battle,
    ability_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        // TODO: Add weather setters (drizzle, drought, sandstream, snowwarning)
        // TODO: Add terrain setters (electricsurge, grassysurge, mistysurge, psychicsurge)
        // TODO: Add stat modifiers (intimidate, download, etc.)
        _ => EventResult::Continue,
    }
}

/// Dispatch onSourceModifyDamage callbacks (read-only)
pub fn dispatch_on_source_modify_damage(
    _battle: &Battle,
    ability_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "multiscale" => {
            // TODO: Check if pokemon HP is full
            // If full HP: return EventResult::Number(2048) // 0.5x damage
            EventResult::Continue
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifyDamage callbacks (read-only)
pub fn dispatch_on_modify_damage(
    _battle: &Battle,
    ability_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        _ => EventResult::Continue,
    }
}

/// Dispatch onResidual callbacks (mutates battle)
pub fn dispatch_on_residual(
    _battle: &mut Battle,
    ability_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifyAtk callbacks (read-only)
pub fn dispatch_on_modify_atk(
    _battle: &Battle,
    ability_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "hugepower" | "purepower" => {
            // Both double Attack: 2.0x = 8192 in 4096 basis
            EventResult::Number(8192)
        }
        "gorillatactics" => {
            // TODO: Check if locked into a move
            // If locked: return EventResult::Number(6144) // 1.5x
            EventResult::Continue
        }
        "defeatist" => {
            // TODO: Check if HP < 50%
            // If HP < 50%: return EventResult::Number(2048) // 0.5x
            EventResult::Continue
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifySpA callbacks (read-only)
pub fn dispatch_on_modify_sp_a(
    _battle: &Battle,
    ability_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "defeatist" => {
            // TODO: Check if HP < 50%
            // If HP < 50%: return EventResult::Number(2048) // 0.5x
            EventResult::Continue
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifyDef callbacks (read-only)
pub fn dispatch_on_modify_def(
    _battle: &Battle,
    ability_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "furcoat" => {
            // Doubles Defense: 2.0x = 8192 in 4096 basis
            EventResult::Number(8192)
        }
        "marvelscale" => {
            // TODO: Check if pokemon has status
            // If has status: return EventResult::Number(6144) // 1.5x
            EventResult::Continue
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifySpD callbacks (read-only)
pub fn dispatch_on_modify_sp_d(
    _battle: &Battle,
    ability_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifySpe callbacks (read-only)
pub fn dispatch_on_modify_spe(
    _battle: &Battle,
    ability_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "quickfeet" => {
            // TODO: Check if pokemon has status
            // If has status: return EventResult::Number(6144) // 1.5x
            EventResult::Continue
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onBasePower callbacks (read-only)
pub fn dispatch_on_base_power(
    _battle: &Battle,
    ability_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifyAccuracy callbacks (read-only)
pub fn dispatch_on_modify_accuracy(
    _battle: &Battle,
    ability_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "compoundeyes" => {
            // Compound Eyes: 1.3x accuracy
            // In 4096 basis: (4096 * 13) / 10 = 5325
            EventResult::Number(5325)
        }
        _ => EventResult::Continue,
    }
}

/// Dispatch onModifyCritRatio callbacks (read-only)
pub fn dispatch_on_modify_crit_ratio(
    _battle: &Battle,
    ability_id: &str,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    match ability_id {
        "superluck" => {
            // Super Luck adds +1 to crit ratio
            EventResult::Number(1)
        }
        _ => EventResult::Continue,
    }
}
