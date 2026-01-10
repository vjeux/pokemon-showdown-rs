//! Captivate Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::Gender;
use crate::event::EventResult;

/// onTryImmunity(pokemon, source) {
///     return (pokemon.gender === 'M' && source.gender === 'F') || (pokemon.gender === 'F' && source.gender === 'M');
/// }
pub fn on_try_immunity(
    battle: &mut Battle,
    pokemon_pos: Option<(usize, usize)>,
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the pokemon
    let pokemon = match pokemon_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get the source
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // return (pokemon.gender === 'M' && source.gender === 'F') || (pokemon.gender === 'F' && source.gender === 'M');
    let pokemon_gender = {
        let pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.gender
    };

    let source_gender = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.gender
    };

    let result = (pokemon_gender == Gender::Male && source_gender == Gender::Female)
        || (pokemon_gender == Gender::Female && source_gender == Gender::Male);

    EventResult::Boolean(result)
}
