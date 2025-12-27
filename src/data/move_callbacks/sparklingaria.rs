//! Sparkling Aria Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onAfterMove(source, target, move) {
///     if (source.fainted || !move.hitTargets || move.hasSheerForce) {
///         // make sure the volatiles are cleared
///         for (const pokemon of this.getAllActive()) delete pokemon.volatiles['sparklingaria'];
///         return;
///     }
///     const numberTargets = move.hitTargets.length;
///     for (const pokemon of move.hitTargets) {
///         // bypasses Shield Dust when hitting multiple targets
///         if (pokemon !== source && pokemon.isActive && (pokemon.removeVolatile('sparklingaria') || numberTargets > 1) &&
///             pokemon.status === 'brn') {
///             pokemon.cureStatus();
///         }
///     }
/// }
pub fn on_after_move(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

