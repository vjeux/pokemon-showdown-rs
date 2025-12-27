//! Jungle Healing Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(pokemon) {
///     const success = !!this.heal(this.modify(pokemon.maxhp, 0.25));
///     return pokemon.cureStatus() || success;
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;

    // const success = !!this.heal(this.modify(pokemon.maxhp, 0.25));
    let heal_amount = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        battle.modify_value(pokemon_pokemon.maxhp, 0.25)
    };

    let heal_success = battle.heal(heal_amount, pokemon, None, None);
    let success = heal_success != 0;

    // return pokemon.cureStatus() || success;
    let cure_status_result = {
        let pokemon_pokemon = match battle.pokemon_at_mut(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.cure_status()
    };

    EventResult::Bool(cure_status_result || success)
}

