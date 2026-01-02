//! Berserk Gene Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onUpdate(pokemon) {
///     if (pokemon.useItem()) {
///         pokemon.addVolatile('confusion');
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.useItem()) {
    //     pokemon.addVolatile('confusion');
    // }

    let used_item = {
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_mut.use_item().is_some()
    };

    if used_item {
        Pokemon::add_volatile(battle, pokemon_pos, ID::new("confusion"), None);
    }

    EventResult::Continue
}
