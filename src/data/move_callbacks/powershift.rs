//! Power Shift Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-start', pokemon, 'Power Shift');
    ///     const newatk = pokemon.storedStats.def;
    ///     const newdef = pokemon.storedStats.atk;
    ///     pokemon.storedStats.atk = newatk;
    ///     pokemon.storedStats.def = newdef;
    /// }
    pub fn on_start(
        battle: &mut Battle,
        pokemon_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
        _effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-start', pokemon, 'Power Shift');
        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_slot()
        };

        battle.add("-start", &[pokemon_arg.into(), "Power Shift".into()]);

        // const newatk = pokemon.storedStats.def;
        // const newdef = pokemon.storedStats.atk;
        // pokemon.storedStats.atk = newatk;
        // pokemon.storedStats.def = newdef;
        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let newatk = pokemon_pokemon.stored_stats.def;
        let newdef = pokemon_pokemon.stored_stats.atk;
        pokemon_pokemon.stored_stats.atk = newatk;
        pokemon_pokemon.stored_stats.def = newdef;

        EventResult::Continue
    }

    /// onCopy(pokemon) {
    ///     const newatk = pokemon.storedStats.def;
    ///     const newdef = pokemon.storedStats.atk;
    ///     pokemon.storedStats.atk = newatk;
    ///     pokemon.storedStats.def = newdef;
    /// }
    pub fn on_copy(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // const newatk = pokemon.storedStats.def;
        // const newdef = pokemon.storedStats.atk;
        // pokemon.storedStats.atk = newatk;
        // pokemon.storedStats.def = newdef;
        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let newatk = pokemon_pokemon.stored_stats.def;
        let newdef = pokemon_pokemon.stored_stats.atk;
        pokemon_pokemon.stored_stats.atk = newatk;
        pokemon_pokemon.stored_stats.def = newdef;

        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Power Shift');
    ///     const newatk = pokemon.storedStats.def;
    ///     const newdef = pokemon.storedStats.atk;
    ///     pokemon.storedStats.atk = newatk;
    ///     pokemon.storedStats.def = newdef;
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-end', pokemon, 'Power Shift');
        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_slot()
        };

        battle.add("-end", &[pokemon_arg.into(), "Power Shift".into()]);

        // const newatk = pokemon.storedStats.def;
        // const newdef = pokemon.storedStats.atk;
        // pokemon.storedStats.atk = newatk;
        // pokemon.storedStats.def = newdef;
        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let newatk = pokemon_pokemon.stored_stats.def;
        let newdef = pokemon_pokemon.stored_stats.atk;
        pokemon_pokemon.stored_stats.atk = newatk;
        pokemon_pokemon.stored_stats.def = newdef;

        EventResult::Continue
    }

    /// onRestart(pokemon) {
    ///     pokemon.removeVolatile('Power Shift');
    /// }
    pub fn on_restart(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        use crate::dex_data::ID;

        let pokemon = pokemon_pos;

        // pokemon.removeVolatile('Power Shift');
        {
            Pokemon::remove_volatile(battle, pokemon, &ID::from("powershift"));
        }

        EventResult::Continue
    }
}
