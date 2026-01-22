//! Ingrain Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, hp_fraction};
use crate::event::EventResult;
use crate::Pokemon;

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-start', pokemon, 'move: Ingrain');
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-start', pokemon, 'move: Ingrain');
        let pokemon_ident = {
            let poke = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            poke.get_slot()
        };
        battle.add(
            "-start",
            &[pokemon_ident.as_str().into(), "move: Ingrain".into()],
        );

        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     this.heal(pokemon.baseMaxhp / 16);
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.heal(pokemon.baseMaxhp / 16);
        let (heal_amount, _pokemon_name) = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            (hp_fraction(pokemon_pokemon.base_maxhp, 16), pokemon_pokemon.name.clone())
        };

        debug_elog!("[INGRAIN RESIDUAL] turn={}, pokemon={}, healing {} HP",
            battle.turn, _pokemon_name, heal_amount);
        battle.heal(heal_amount, Some(pokemon), None, None);

        EventResult::Continue
    }

    /// onTrapPokemon(pokemon) {
    ///     pokemon.tryTrap();
    /// }
    pub fn on_trap_pokemon(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // pokemon.tryTrap();
        Pokemon::try_trap(battle, pokemon, false);

        EventResult::Continue
    }

    /// onDragOut(pokemon) {
    ///     this.add('-activate', pokemon, 'move: Ingrain');
    ///     return null;
    /// }
    pub fn on_drag_out(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-activate', pokemon, 'move: Ingrain');
        let pokemon_ident = {
            let poke = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            poke.get_slot()
        };
        battle.add(
            "-activate",
            &[pokemon_ident.as_str().into(), "move: Ingrain".into()],
        );

        // return null;
        // EventResult::Null prevents the Pokemon from being dragged out
        EventResult::Null
    }
}
