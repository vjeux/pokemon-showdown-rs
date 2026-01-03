//! Curious Medicine Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     for (const ally of pokemon.adjacentAllies()) {
///         ally.clearBoosts();
///         this.add('-clearboost', ally, '[from] ability: Curious Medicine', `[of] ${pokemon}`);
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;

    // for (const ally of pokemon.adjacentAllies())
    let adjacent_allies = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.adjacent_allies(battle)
    };

    // Get pokemon identifier for the "[of] ${pokemon}" part
    let pokemon_id = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    // Clear boosts for each ally
    for ally_pos in adjacent_allies {
        let ally_id = {
            let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            ally.get_slot()
        };

        // ally.clearBoosts();
        {
            let ally = match battle.pokemon_at_mut(ally_pos.0, ally_pos.1) {
                Some(p) => p,
                None => continue,
            };
            ally.clear_boosts();
        }

        // this.add('-clearboost', ally, '[from] ability: Curious Medicine', `[of] ${pokemon}`);
        battle.add("-clearboost", &[
            Arg::String(ally_id),
            Arg::Str("[from] ability: Curious Medicine"),
            Arg::String(format!("[of] {}", pokemon_id)),
        ]);
    }

    EventResult::Continue
}

