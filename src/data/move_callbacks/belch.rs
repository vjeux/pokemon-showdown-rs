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

    // Only allow belch if pokemon has eaten a berry this battle
    if !pokemon.ate_berry {
        // Need to get mutable reference to disable the move
        drop(pokemon);
        if let Some(pokemon_mut) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            pokemon_mut.disable_move("belch", None);
        }
    }

    EventResult::Continue
}

