//! Heat Crash Move
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
///     const targetWeight = target.getWeight();
///     const pokemonWeight = pokemon.getWeight();
///     let bp;
///     if (pokemonWeight >= targetWeight * 5) {
///         bp = 120;
///     } else if (pokemonWeight >= targetWeight * 4) {
///         bp = 100;
///     } else if (pokemonWeight >= targetWeight * 3) {
///         bp = 80;
///     } else if (pokemonWeight >= targetWeight * 2) {
///         bp = 60;
///     } else {
///         bp = 40;
///     }
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onTryHit(target, pokemon, move) {
///     if (target.volatiles['dynamax']) {
///         this.add('-fail', pokemon, 'Dynamax');
///         this.attrLastMove('[still]');
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), move_id: &str) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

