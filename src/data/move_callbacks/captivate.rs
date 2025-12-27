//! Captivate Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryImmunity(pokemon, source) {
///     return (pokemon.gender === 'M' && source.gender === 'F') || (pokemon.gender === 'F' && source.gender === 'M');
/// }
pub fn on_try_immunity(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    use crate::dex_data::Gender;

    // Only works if pokemon and source have opposite genders
    let opposite_genders = (pokemon.gender == Gender::Male && source.gender == Gender::Female) ||
                          (pokemon.gender == Gender::Female && source.gender == Gender::Male);

    EventResult::Bool(opposite_genders)
}

