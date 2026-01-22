//! Regenerator Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect, hp_fraction};
use crate::event::EventResult;

/// onSwitchOut(pokemon) {
///     pokemon.heal(pokemon.baseMaxhp / 3);
/// }
pub fn on_switch_out(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Heal 1/3 of max HP when switching out
    let heal_amount = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        hp_fraction(pokemon.base_maxhp, 3)
    };

    battle.heal(heal_amount, Some(pokemon_pos), None, None);
    EventResult::Continue
}

