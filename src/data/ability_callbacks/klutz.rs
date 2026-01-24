//! Klutz Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onStart(pokemon) {
///     this.singleEvent('End', pokemon.getItem(), pokemon.itemState, pokemon);
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
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
        let item_effect = battle.make_item_effect(&ID::from(item_id.as_str()));
        battle.single_event("End", &item_effect, None, Some(pokemon_pos), None, None, None);
    }

    EventResult::Continue
}

