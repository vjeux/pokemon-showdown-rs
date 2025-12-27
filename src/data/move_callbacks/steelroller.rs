//! Steel Roller Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTry() {
///     return !this.field.isTerrain('');
/// }
pub fn on_try(battle: &mut Battle) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onHit() {
///     this.field.clearTerrain();
/// }
pub fn on_hit(battle: &mut Battle) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onAfterSubDamage() {
///     this.field.clearTerrain();
/// }
pub fn on_after_sub_damage(battle: &mut Battle) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

