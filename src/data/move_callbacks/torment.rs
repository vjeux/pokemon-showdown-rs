//! Torment Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(pokemon, source, effect) {
    ///     if (pokemon.volatiles['dynamax']) {
    ///         delete pokemon.volatiles['torment'];
    ///         return false;
    ///     }
    ///     if (effect?.id === 'gmaxmeltdown') this.effectState.duration = 3;
    ///     this.add('-start', pokemon, 'Torment');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // TODO: Check for dynamax volatile
        // TODO: Check effect_id for gmaxmeltdown
        // TODO: battle.add('-start', pokemon, 'Torment');
        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Torment');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: battle.add('-end', pokemon, 'Torment');
        EventResult::Continue
    }

    /// onDisableMove(pokemon) {
    ///     if (pokemon.lastMove && pokemon.lastMove.id !== 'struggle') pokemon.disableMove(pokemon.lastMove.id);
    /// }
    pub fn on_disable_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Torment prevents using the same move twice in a row
        if let Some(ref last_move) = pokemon.last_move {
            if last_move.as_str() != "struggle" {
                // Temporarily store last_move to avoid borrow checker issues
                let last_move_id = last_move.clone();
                drop(pokemon); // Release borrow

                if let Some(pokemon_mut) = battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                    pokemon_mut.disable_move(&last_move_id.as_str(), None);
                }
            }
        }

        EventResult::Continue
    }
}
