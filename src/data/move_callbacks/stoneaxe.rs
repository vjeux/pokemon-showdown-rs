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
pub fn on_after_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    let source_hp = if let Some(pokemon) = battle.pokemon_at(source_pos.0, source_pos.1) {
        pokemon.hp
    } else {
        return EventResult::Continue;
    };

    // Only set up Stealth Rock if source is alive
    // TODO: Check move.hasSheerForce
    if source_hp > 0 {
        // Get foe sides that have conditions and add Stealth Rock to them
        let source_side_idx = source_pos.0;

        // Get indices of foe sides with conditions
        let foe_indices: Vec<usize> = if let Some(source_side) = battle.sides.get(source_side_idx) {
            source_side.foe_sides_with_conditions(&battle.sides)
                .into_iter()
                .filter_map(|side| battle.sides.iter().position(|s| std::ptr::eq(s, side)))
                .collect()
        } else {
            return EventResult::Continue;
        };

        // Add Stealth Rock to each foe side
        for foe_idx in foe_indices {
            if let Some(side) = battle.sides.get_mut(foe_idx) {
                side.add_side_condition(crate::dex_data::ID::new("stealthrock"), None);
            }
        }
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
pub fn on_after_sub_damage(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_hp = if let Some(pokemon) = battle.pokemon_at(source_pos.0, source_pos.1) {
        pokemon.hp
    } else {
        return EventResult::Continue;
    };

    // Only set up Stealth Rock if source is alive
    // TODO: Check move.hasSheerForce
    if source_hp > 0 {
        // Get foe sides that have conditions and add Stealth Rock to them
        let source_side_idx = source_pos.0;

        // Get indices of foe sides with conditions
        let foe_indices: Vec<usize> = if let Some(source_side) = battle.sides.get(source_side_idx) {
            source_side.foe_sides_with_conditions(&battle.sides)
                .into_iter()
                .filter_map(|side| battle.sides.iter().position(|s| std::ptr::eq(s, side)))
                .collect()
        } else {
            return EventResult::Continue;
        };

        // Add Stealth Rock to each foe side
        for foe_idx in foe_indices {
            if let Some(side) = battle.sides.get_mut(foe_idx) {
                side.add_side_condition(crate::dex_data::ID::new("stealthrock"), None);
            }
        }
    }

    EventResult::Continue
}

