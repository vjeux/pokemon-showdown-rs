//! Hospitality Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect, hp_fraction};
use crate::event::EventResult;

/// onStart(pokemon) {
///     for (const ally of pokemon.adjacentAllies()) {
///         this.heal(ally.baseMaxhp / 4, ally, pokemon);
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    // for (const ally of pokemon.adjacentAllies())
    let adjacent_allies = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.adjacent_allies(battle)
    };

    // Heal each ally for 1/4 of their max HP
    for ally_pos in adjacent_allies {
        let heal_amount = {
            let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            hp_fraction(ally.base_maxhp, 4)
        };

        // this.heal(ally.baseMaxhp / 4, ally, pokemon);
        battle.heal(heal_amount, Some(ally_pos), Some(pokemon_pos), None);
    }

    EventResult::Continue
}

