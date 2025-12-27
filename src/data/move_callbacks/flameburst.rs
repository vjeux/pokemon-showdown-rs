//! Flame Burst Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit(...)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(target, source, move) {			for (const ally of target.adjacentAllies()) {
/// 				this.damage(ally.baseMaxhp / 16, ally, source, this.dex.conditions.get('Flame Burst'));
/// 			}
/// 		}
/// ```
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onAfterSubDamage(...)
///
/// ```text
/// JS Source (data/moves.ts):
/// onAfterSubDamage(damage, target, source, move) {			for (const ally of target.adjacentAllies()) {
/// 				this.damage(ally.baseMaxhp / 16, ally, source, this.dex.conditions.get('Flame Burst'));
/// 			}
/// 		}
/// ```
pub fn on_after_sub_damage(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

