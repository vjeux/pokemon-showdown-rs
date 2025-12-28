//! Ingrain Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-start', pokemon, 'move: Ingrain');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-start', pokemon, 'move: Ingrain');
        let pokemon_arg = {
            let poke = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(poke)
        };
        battle.add("-start", &[pokemon_arg, "move: Ingrain".into()]);

        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     this.heal(pokemon.baseMaxhp / 16);
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.heal(pokemon.baseMaxhp / 16);
        let heal_amount = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.base_maxhp / 16
        };

        battle.heal(heal_amount, Some(pokemon), None, None);

        EventResult::Continue
    }

    /// onTrapPokemon(pokemon) {
    ///     pokemon.tryTrap();
    /// }
    pub fn on_trap_pokemon(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // pokemon.tryTrap();
        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.try_trap(false);

        EventResult::Continue
    }

    /// onDragOut(pokemon) {
    ///     this.add('-activate', pokemon, 'move: Ingrain');
    ///     return null;
    /// }
    pub fn on_drag_out(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-activate', pokemon, 'move: Ingrain');
        let pokemon_arg = {
            let poke = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(poke)
        };
        battle.add("-activate", &[pokemon_arg, "move: Ingrain".into()]);

        // return null;
        EventResult::Stop
    }
}
