//! Metronome Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onStart(pokemon) {
///     pokemon.addVolatile('metronome');
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // pokemon.addVolatile('metronome');
    let pokemon_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    Pokemon::add_volatile(battle, pokemon_pos, ID::new("metronome"), None);

    EventResult::Continue
}
