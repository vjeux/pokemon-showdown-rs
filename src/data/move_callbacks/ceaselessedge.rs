//! Ceaseless Edge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::dex_data::ID;

/// onAfterHit(target, source, move) {
///     if (!move.hasSheerForce && source.hp) {
///         for (const side of source.side.foeSidesWithConditions()) {
///             side.addSideCondition('spikes');
///         }
///     }
/// }
pub fn on_after_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    // if (!move.hasSheerForce && source.hp) {
    let has_sheer_force = battle.active_move_has_sheer_force();

    let source_hp = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.hp
    };

    if !has_sheer_force && source_hp > 0 {
        // for (const side of source.side.foeSidesWithConditions()) {
        //     side.addSideCondition('spikes');
        // }
        let source_side_idx = source_pos.0;

        // Get foe sides (opposite side in a 2-player battle)
        for side_idx in 0..battle.sides.len() {
            if side_idx != source_side_idx {
                battle.sides[side_idx].add_side_condition(&ID::from("spikes"));
            }
        }
    }

    EventResult::Continue
}

/// onAfterSubDamage(damage, target, source, move) {
///     if (!move.hasSheerForce && source.hp) {
///         for (const side of source.side.foeSidesWithConditions()) {
///             side.addSideCondition('spikes');
///         }
///     }
/// }
pub fn on_after_sub_damage(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // Get the source
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!move.hasSheerForce && source.hp) {
    let has_sheer_force = battle.active_move_has_sheer_force();

    let source_hp = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.hp
    };

    if !has_sheer_force && source_hp > 0 {
        // for (const side of source.side.foeSidesWithConditions()) {
        //     side.addSideCondition('spikes');
        // }
        let source_side_idx = source.0;

        // Get foe sides (opposite side in a 2-player battle)
        for side_idx in 0..battle.sides.len() {
            if side_idx != source_side_idx {
                battle.sides[side_idx].add_side_condition(&ID::from("spikes"));
            }
        }
    }

    EventResult::Continue
}
