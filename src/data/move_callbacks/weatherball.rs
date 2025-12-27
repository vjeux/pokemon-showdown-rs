//! Weather Ball Move
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

/// onModifyType(move, pokemon) {
///     switch (pokemon.effectiveWeather()) {
///     case 'sunnyday':
///     case 'desolateland':
///         move.type = 'Fire';
///         break;
///     case 'raindance':
///     case 'primordialsea':
///         move.type = 'Water';
///         break;
///     case 'sandstorm':
///         move.type = 'Rock';
///         break;
///     case 'hail':
///     case 'snowscape':
///         move.type = 'Ice';
///         break;
///     }
/// }
pub fn on_modify_type(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyMove(move, pokemon) {
///     switch (pokemon.effectiveWeather()) {
///     case 'sunnyday':
///     case 'desolateland':
///         move.basePower *= 2;
///         break;
///     case 'raindance':
///     case 'primordialsea':
///         move.basePower *= 2;
///         break;
///     case 'sandstorm':
///         move.basePower *= 2;
///         break;
///     case 'hail':
///     case 'snowscape':
///         move.basePower *= 2;
///         break;
///     }
///     this.debug(`BP: ${move.basePower}`);
/// }
pub fn on_modify_move(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

