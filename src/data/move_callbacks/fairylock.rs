//! Fairy Lock Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

pub mod condition {
    use super::*;

    /// onFieldStart(target) {
    ///     this.add('-fieldactivate', 'move: Fairy Lock');
    /// }
    pub fn on_field_start(battle: &mut Battle, _target_pos: Option<(usize, usize)>) -> EventResult {
        // this.add('-fieldactivate', 'move: Fairy Lock');
        battle.add("-fieldactivate", &["move: Fairy Lock".into()]);

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
}
