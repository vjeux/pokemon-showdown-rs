//! Refresh Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(pokemon) {
///     if (['', 'slp', 'frz'].includes(pokemon.status)) return false;
///     pokemon.cureStatus();
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (['', 'slp', 'frz'].includes(pokemon.status)) return false;
    let status = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.status.clone()
    };

    if status == ID::from("") || status == ID::from("slp") || status == ID::from("frz") {
        return EventResult::Boolean(false);
    }

    // pokemon.cureStatus();
    battle.cure_status(pokemon, None, None);

    EventResult::Continue
}

