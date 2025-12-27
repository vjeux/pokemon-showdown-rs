//! Punishment Move
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

/// basePowerCallback(pokemon, target) {
///     let power = 60 + 20 * target.positiveBoosts();
///     if (power > 200) power = 200;
///     this.debug(`BP: ${power}`);
///     return power;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

