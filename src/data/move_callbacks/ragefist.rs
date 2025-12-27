//! Rage Fist Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use crate::event::EventResult;
use super::{Status, Effect};

/// basePowerCallback(pokemon) {
///     return Math.min(350, 50 + 50 * pokemon.timesAttacked);
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

