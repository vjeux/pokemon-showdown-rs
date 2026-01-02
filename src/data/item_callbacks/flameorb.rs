//! Flame Orb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onResidual(pokemon) {
///     pokemon.trySetStatus('brn', pokemon);
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // pokemon.trySetStatus('brn', pokemon);
    Pokemon::try_set_status(battle, pokemon_pos, crate::dex_data::ID::new("brn"), None);

    EventResult::Continue
}
