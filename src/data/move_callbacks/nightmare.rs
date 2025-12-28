//! Nightmare Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;


pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     if (pokemon.status !== 'slp' && !pokemon.hasAbility('comatose')) {
    ///         return false;
    ///     }
    ///     this.add('-start', pokemon, 'Nightmare');
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // if (pokemon.status !== 'slp' && !pokemon.hasAbility('comatose')) {
        let (status, has_comatose) = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Boolean(false),
            };
            let status = pokemon_pokemon.status.clone();
            let has_comatose = pokemon_pokemon.has_ability(&["comatose"]);
            (status, has_comatose)
        };

        if status != ID::from("slp") && !has_comatose {
            // return false;
            return EventResult::Boolean(false);
        }

        // this.add('-start', pokemon, 'Nightmare');
        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            crate::battle::Arg::from(pokemon_pokemon)
        };

        battle.add("-start", &[pokemon_arg, "Nightmare".into()]);

        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     this.damage(pokemon.baseMaxhp / 4);
    /// }
    pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.damage(pokemon.baseMaxhp / 4);
        let damage = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.base_maxhp / 4
        };

        battle.damage(damage, Some(pokemon), None, None, false);

        EventResult::Continue
    }
}
