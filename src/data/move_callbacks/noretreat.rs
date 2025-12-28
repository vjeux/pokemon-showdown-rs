//! No Retreat Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source, target, move) {
///     if (source.volatiles['noretreat']) return false;
///     if (source.volatiles['trapped']) {
///         delete move.volatileStatus;
///     }
/// }
pub fn on_try(battle: &mut Battle, source_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let source = source_pos;

    // if (source.volatiles['noretreat']) return false;
    let has_noretreat = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.volatiles.contains_key(&ID::from("noretreat"))
    };

    if has_noretreat {
        return EventResult::Boolean(false);
    }

    // if (source.volatiles['trapped']) {
    let has_trapped = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.volatiles.contains_key(&ID::from("trapped"))
    };

    if has_trapped {
        // delete move.volatileStatus;
        if let Some(ref mut active_move) = battle.active_move {
            active_move.volatile_status = None;
        }
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-start', pokemon, 'move: No Retreat');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-start', pokemon, 'move: No Retreat');
        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(pokemon_pokemon)
        };

        battle.add("-start", &[pokemon_arg.into(), "move: No Retreat".into()]);

        EventResult::Continue
    }

    /// onTrapPokemon(pokemon) {
    ///     pokemon.tryTrap();
    /// }
    pub fn on_trap_pokemon(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // pokemon.tryTrap();
        {
            let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.try_trap(false);
        }

        EventResult::Continue
    }
}
