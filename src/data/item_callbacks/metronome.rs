//! Metronome Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     pokemon.addVolatile('metronome');
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // pokemon.addVolatile('metronome');
    let pokemon_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    pokemon_mut.add_volatile(crate::dex_data::ID::new("metronome"));

    EventResult::Continue
}
