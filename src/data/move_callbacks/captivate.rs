//! Captivate Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::dex_data::{ID, Gender};

/// onTryImmunity(pokemon, source) {
///     return (pokemon.gender === 'M' && source.gender === 'F') || (pokemon.gender === 'F' && source.gender === 'M');
/// }
pub fn on_try_immunity(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // Get the source
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return (pokemon.gender === 'M' && source.gender === 'F') || (pokemon.gender === 'F' && source.gender === 'M');
    let pokemon_gender = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.gender.clone()
    };

    let source_gender = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.gender.clone()
    };

    let result = (pokemon_gender == Gender::Male && source_gender == Gender::Female) ||
                 (pokemon_gender == Gender::Female && source_gender == Gender::Male);

    EventResult::Boolean(result)
}
