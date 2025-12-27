//! Purify Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit(target, source) {
/// if (!target.cureStatus()) {
///     this.add('-fail', source);
///     this.attrLastMove('[still]');
///     return this.NOT_FAIL;
/// }
/// this.heal(Math.ceil(source.maxhp * 0.5), source);
/// }
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

