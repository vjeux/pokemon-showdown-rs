//! Weakness Policy Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onDamagingHit(damage, target, source, move) {
///     if (!move.damage && !move.damageCallback && target.getMoveHitData(move).typeMod > 0) {
///         target.useItem();
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: (usize, usize), _source_pos: (usize, usize)) -> EventResult {
    // if (!move.damage && !move.damageCallback && target.getMoveHitData(move).typeMod > 0) {
    //     target.useItem();
    // }

    // Check if move has fixed damage (move.damage or move.damageCallback)
    let has_fixed_damage = match &battle.active_move {
        Some(_active_move) => {
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

    // Get move ID from active_move (clone to avoid borrow issues)
    let move_id = match &battle.active_move {
        Some(active_move) => active_move.id.clone(),
        None => return EventResult::Continue,
    };

    // Check type effectiveness against target
    // target.getMoveHitData(move).typeMod > 0 means super effective
    let type_effectiveness = Pokemon::run_effectiveness(battle, target_pos, &move_id);

    // typeMod > 0 in JavaScript means super effective
    if type_effectiveness > 0 {
        // target.useItem();
        let _target_mut = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        Pokemon::use_item(battle, target_pos, None, None);
    }

    EventResult::Continue
}
