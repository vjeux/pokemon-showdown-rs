//! Spit Up Move
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
///     if (!pokemon.volatiles['stockpile']?.layers) return false;
///     return pokemon.volatiles['stockpile'].layers * 100;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTry(source) {
///     return !!source.volatiles['stockpile'];
/// }
pub fn on_try(battle: &mut Battle, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAfterMove(pokemon) {
///     pokemon.removeVolatile('stockpile');
/// }
pub fn on_after_move(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

