//! Aqua Ring Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-start', pokemon, 'Aqua Ring');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // this.add('-start', pokemon, 'Aqua Ring');
        let pokemon_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            format!("p{}{}", pokemon_pos.0 + 1, pokemon.ident)
        };
        battle.add("-start", &[pokemon_ident.into(), "Aqua Ring".into()]);
        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     this.heal(pokemon.baseMaxhp / 16);
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // Get the pokemon to access its base max hp
        let base_maxhp = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p.base_maxhp,
            None => return EventResult::Continue,
        };

        // this.heal(pokemon.baseMaxhp / 16);
        battle.heal(base_maxhp / 16, Some(pokemon_pos), None, None);

        EventResult::Continue
    }
}
