//! Facade Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, pokemon) {
///     if (pokemon.status && pokemon.status !== 'slp') {
///         return this.chainModify(2);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (pokemon.status && pokemon.status !== 'slp') {
    let status = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.status.clone()
    };

    if status != ID::from("") && status != ID::from("slp") {
        // return this.chainModify(2);
        return EventResult::Number(battle.chain_modify(2.0 as f32));
    }

    EventResult::Continue
}

