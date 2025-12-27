//! Metal Burst Move
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

/// damageCallback(pokemon) {
///     const lastDamagedBy = pokemon.getLastDamagedBy(true);
///     if (lastDamagedBy !== undefined) {
///         return (lastDamagedBy.damage * 1.5) || 1;
///     }
///     return 0;
/// }
pub fn damage_callback(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTry(source) {
///     const lastDamagedBy = source.getLastDamagedBy(true);
///     if (!lastDamagedBy?.thisTurn) return false;
/// }
pub fn on_try(battle: &mut Battle, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyTarget(targetRelayVar, source, target, move) {
///     const lastDamagedBy = source.getLastDamagedBy(true);
///     if (lastDamagedBy) {
///         targetRelayVar.target = this.getAtSlot(lastDamagedBy.slot);
///     }
/// }
pub fn on_modify_target(battle: &mut Battle, source_pos: Option<(usize, usize)>, target_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

