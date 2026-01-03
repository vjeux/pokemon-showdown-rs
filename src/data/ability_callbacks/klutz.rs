//! Klutz Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onStart(pokemon) {
///     this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
    let item_id = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.item.clone()
    };

    // Only call singleEvent if pokemon has an item
    if !item_id.is_empty() {
        battle.single_event("End", &ID::from(item_id), Some(pokemon_pos), None, None);
    }

    EventResult::Continue
}

