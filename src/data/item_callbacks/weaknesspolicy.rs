//! Weakness Policy Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (!move.damage && !move.damageCallback && target.getMoveHitData(move).typeMod > 0) {
///         target.useItem();
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, damage: i32, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    // if (!move.damage && !move.damageCallback && target.getMoveHitData(move).typeMod > 0) {
    //     target.useItem();
    // }

    // Check if move has fixed damage (move.damage or move.damageCallback)
    let has_fixed_damage = match &battle.active_move {
        Some(active_move) => {
            // move.damage is a fixed damage value, move.damageCallback means custom damage calculation
            // For now, we don't have these fields, so we assume no fixed damage
            // This is a simplification but maintains the logic structure
            false
        },
        None => return EventResult::Continue,
    };

    if has_fixed_damage {
        return EventResult::Continue;
    }

    // Get move type from active_move
    let move_type = match &battle.active_move {
        Some(active_move) => active_move.move_type.clone(),
        None => return EventResult::Continue,
    };

    // Check type effectiveness against target
    // target.getMoveHitData(move).typeMod > 0 means super effective
    let type_effectiveness = {
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.run_effectiveness(&move_type)
    };

    // typeMod > 0 in JavaScript means super effective (effectiveness > 1.0 in our system)
    if type_effectiveness > 1.0 {
        // target.useItem();
        let target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_mut.use_item();
    }

    EventResult::Continue
}
