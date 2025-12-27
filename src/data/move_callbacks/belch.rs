//! Belch Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDisableMove(pokemon) {
///     if (!pokemon.ateBerry) pokemon.disableMove('belch');
/// }
pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if !pokemon.ate_berry {
        // TODO: pokemon.disableMove('belch');
    }

    EventResult::Continue
}

