//! Salt Cure Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-start', pokemon, 'Salt Cure');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // this.add('-start', pokemon, 'Salt Cure');
        let pokemon_arg = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(pokemon)
        };

        battle.add("-start", &[
            pokemon_arg,
            "Salt Cure".into(),
        ]);

        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     this.damage(pokemon.baseMaxhp / (pokemon.hasType(['Water', 'Steel']) ? 4 : 8));
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.damage(pokemon.baseMaxhp / (pokemon.hasType(['Water', 'Steel']) ? 4 : 8));
        let has_water_or_steel = battle.has_type(pokemon, "Water") || battle.has_type(pokemon, "Steel");

        let divisor = if has_water_or_steel { 4 } else { 8 };

        let damage = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.base_maxhp / divisor
        };

        battle.damage(damage, pokemon, None, None, false);

        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Salt Cure');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // this.add('-end', pokemon, 'Salt Cure');
        let pokemon_arg = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(pokemon)
        };

        battle.add("-end", &[
            pokemon_arg,
            "Salt Cure".into(),
        ]);

        EventResult::Continue
    }
}
