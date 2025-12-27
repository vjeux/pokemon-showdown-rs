//! Gyro Ball Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// basePowerCallback(pokemon, target) {
///     let power = Math.floor(25 * target.getStat('spe') / pokemon.getStat('spe')) + 1;
///     if (!isFinite(power)) power = 1;
///     if (power > 150) power = 150;
///     this.debug(`BP: ${power}`);
///     return power;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

