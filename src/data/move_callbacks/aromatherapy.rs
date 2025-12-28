//! Aromatherapy Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;
use crate::dex_data::ID;

/// onHit(target, source, move) {
///     this.add('-activate', source, 'move: Aromatherapy');
///     let success = false;
///     const allies = [...target.side.pokemon, ...target.side.allySide?.pokemon || []];
///     for (const ally of allies) {
///         if (ally !== source && !this.suppressingAbility(ally)) {
///             if (ally.hasAbility('sapsipper')) {
///                 this.add('-immune', ally, '[from] ability: Sap Sipper');
///                 continue;
///             }
///             if (ally.hasAbility('goodasgold')) {
///                 this.add('-immune', ally, '[from] ability: Good as Gold');
///                 continue;
///             }
///             if (ally.volatiles['substitute'] && !move.infiltrates) continue;
///         }
///         if (ally.cureStatus()) success = true;
///     }
///     return success;
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // Get the source pokemon
    let source_pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // this.add('-activate', source, 'move: Aromatherapy');
    battle.add("-activate", &[source_pokemon.into(), "move: Aromatherapy".into()]);

    // let success = false;
    let mut success = false;

    // Get the target position
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const allies = [...target.side.pokemon, ...target.side.allySide?.pokemon || []];
    // For now, just iterate through target.side.pokemon (allySide not yet implemented)
    let target_side_idx = target.0;
    let num_pokemon = battle.sides[target_side_idx].pokemon.len();

    // for (const ally of allies) {
    for ally_idx in 0..num_pokemon {
        let ally_pos = (target_side_idx, ally_idx);

        // if (ally !== source && !this.suppressingAbility(ally)) {
        let is_source = ally_pos == pokemon_pos;
        let is_suppressing = battle.suppressing_ability(Some(ally_pos));

        if !is_source && !is_suppressing {
            let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };

            // if (ally.hasAbility('sapsipper')) {
            if ally.has_ability(&["sapsipper"]) {
                // this.add('-immune', ally, '[from] ability: Sap Sipper');
                battle.add("-immune", &[ally.into(), "[from] ability: Sap Sipper".into()]);
                // continue;
                continue;
            }

            // if (ally.hasAbility('goodasgold')) {
            if ally.has_ability(&["goodasgold"]) {
                // this.add('-immune', ally, '[from] ability: Good as Gold');
                battle.add("-immune", &[ally.into(), "[from] ability: Good as Gold".into()]);
                // continue;
                continue;
            }

            // if (ally.volatiles['substitute'] && !move.infiltrates) continue;
            // Note: move.infiltrates not yet implemented, so we check substitute
            if ally.volatiles.contains_key(&ID::from("substitute")) {
                continue;
            }
        }

        // if (ally.cureStatus()) success = true;
        if battle.cure_status(ally_pos) {
            success = true;
        }
    }

    // return success;
    EventResult::Boolean(success)
}

