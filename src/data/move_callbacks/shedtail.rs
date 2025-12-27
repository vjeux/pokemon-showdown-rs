//! Shed Tail Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onTryHit(source) {            if (!this.canSwitch(source.side) || source.volatiles['commanded']) {
///                 this.add('-fail', source);
///                 return this.NOT_FAIL;
///             }
///             if (source.volatiles['substitute']) {
///                 this.add('-fail', source, 'move: Shed Tail');
///                 return this.NOT_FAIL;
///             }
///             if (source.hp <= Math.ceil(source.maxhp / 2)) {
///                 this.add('-fail', source, 'move: Shed Tail', '[weak]');
///                 return this.NOT_FAIL;
///             }
///         }
pub fn on_try_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onHit(target) {            this.directDamage(Math.ceil(target.maxhp / 2));
///         }
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

