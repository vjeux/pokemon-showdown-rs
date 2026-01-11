//! Ceaseless Edge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onAfterHit(target, source, move) {
///     if (!move.hasSheerForce && source.hp) {
///         for (const side of source.side.foeSidesWithConditions()) {
///             side.addSideCondition('spikes');
///         }
///     }
/// }
/// JavaScript signature: onAfterHit(target, source, move) - target first, source second
pub fn on_after_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),
    source_pos: (usize, usize),
) -> EventResult {
    // if (!move.hasSheerForce && source.hp) {
    let has_sheer_force = battle
        .active_move
        .as_ref()
        .map(|m| m.has_sheer_force)
        .unwrap_or(false);

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
                // Use Battle::add_side_condition which properly handles SideRestart for stacking
                battle.add_side_condition(
                    side_idx,
                    ID::from("spikes"),
                    Some(source_pos),
                    Some(&crate::battle::Effect::move_("ceaselessedge")),
                );
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
pub fn on_after_sub_damage(
    battle: &mut Battle,
    _damage: i32,
    _target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // Get the source
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!move.hasSheerForce && source.hp) {
    let has_sheer_force = battle
        .active_move
        .as_ref()
        .map(|m| m.has_sheer_force)
        .unwrap_or(false);

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
                // Use Battle::add_side_condition which properly handles SideRestart for stacking
                battle.add_side_condition(
                    side_idx,
                    ID::from("spikes"),
                    Some(source),
                    Some(&crate::battle::Effect::move_("ceaselessedge")),
                );
            }
        }
    }

    EventResult::Continue
}
