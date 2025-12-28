//! Embargo Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

pub mod condition {
    use super::*;

    /// onStart(pokemon) {
    ///     this.add('-start', pokemon, 'Embargo');
    ///     this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
    /// }
    pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-start', pokemon, 'Embargo');
        let pokemon_ident = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_slot()
        };

        battle.add("-start", &[pokemon_ident.as_str().into(), "Embargo".into()]);

        // this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
        let item_id = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.item.clone()
        };

        battle.single_event("End", &item_id, Some(pokemon), None, None);

        EventResult::Continue
    }

    /// onEnd(pokemon) {
    ///     this.add('-end', pokemon, 'Embargo');
    /// }
    pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        let pokemon = pokemon_pos;

        // this.add('-end', pokemon, 'Embargo');
        let pokemon_ident = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_slot()
        };

        battle.add("-end", &[pokemon_ident.as_str().into(), "Embargo".into()]);

        EventResult::Continue
    }
}
