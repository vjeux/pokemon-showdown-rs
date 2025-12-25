//! Sniper Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	sniper: {
//! 		onModifyDamage(damage, source, target, move) {
//! 			if (target.getMoveHitData(move).crit) {
//! 				this.debug('Sniper boost');
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Sniper",
//! 		rating: 2,
//! 		num: 97,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyDamage(damage, source, target, move)
/// Boosts damage by 1.5x on critical hits
///
/// TODO: This handler is not yet registered in the ability system.
/// The onModifyDamage handler needs to be added to the battle engine alongside
/// onSourceModifyDamage. When implemented, this should:
/// 1. Check if target.getMoveHitData(move).crit is true
/// 2. Return chainModify(1.5) which is ChainModify(6144, 4096)
///
/// Expected signature once infrastructure is ready:
/// pub fn on_modify_damage(_battle: &mut Battle, _damage: u32, _source: &Pokemon, target: &Pokemon, move_: &MoveDef, was_crit: bool) -> AbilityHandlerResult {
///     if was_crit {
///         return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
///     }
///     AbilityHandlerResult::Undefined
/// }
pub fn on_modify_damage(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement when onModifyDamage handler infrastructure is added
    AbilityHandlerResult::Undefined
}

