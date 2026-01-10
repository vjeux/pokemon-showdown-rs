//! Stone Axe Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterHit(target, source, move) {
///     if (!move.hasSheerForce && source.hp) {
///         for (const side of source.side.foeSidesWithConditions()) {
///             side.addSideCondition('stealthrock');
///         }
///     }
/// }
pub fn on_after_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),  // JavaScript: onAfterHit(target, source) - target first
    source_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    let source = source_pos;

    // if (!move.hasSheerForce && source.hp) {
    let has_sheer_force = match &battle.active_move {
        Some(active_move) => active_move.has_sheer_force,
        None => return EventResult::Continue,
    };

    if has_sheer_force {
        return EventResult::Continue;
    }

    let (source_side, source_hp) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source_pokemon.side_index, source_pokemon.hp)
    };

    if source_hp == 0 {
        return EventResult::Continue;
    }

    // for (const side of source.side.foeSidesWithConditions()) {
    //     side.addSideCondition('stealthrock');
    // }
    let foe_side_indices: Vec<usize> = battle.sides[source_side]
        .foe_sides_with_conditions(&battle.sides)
        .iter()
        .map(|s| s.n)
        .collect();

    for foe_side_idx in foe_side_indices {
        let _ = battle.sides[foe_side_idx].add_side_condition(ID::from("stealthrock"), None);
    }

    EventResult::Continue
}

/// onAfterSubDamage(damage, target, source, move) {
///     if (!move.hasSheerForce && source.hp) {
///         for (const side of source.side.foeSidesWithConditions()) {
///             side.addSideCondition('stealthrock');
///         }
///     }
/// }
pub fn on_after_sub_damage(
    battle: &mut Battle,
    _damage: i32,
    _target_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
    active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    use crate::dex_data::ID;

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get active_move from parameter
    let active_move_ref = match active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (!move.hasSheerForce && source.hp) {
    if active_move_ref.has_sheer_force {
        return EventResult::Continue;
    }

    let (source_side, source_hp) = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (source_pokemon.side_index, source_pokemon.hp)
    };

    if source_hp == 0 {
        return EventResult::Continue;
    }

    // for (const side of source.side.foeSidesWithConditions()) {
    //     side.addSideCondition('stealthrock');
    // }
    let foe_side_indices: Vec<usize> = battle.sides[source_side]
        .foe_sides_with_conditions(&battle.sides)
        .iter()
        .map(|s| s.n)
        .collect();

    for foe_side_idx in foe_side_indices {
        let _ = battle.sides[foe_side_idx].add_side_condition(ID::from("stealthrock"), None);
    }

    EventResult::Continue
}
