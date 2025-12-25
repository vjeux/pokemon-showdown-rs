//! Stakeout Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	stakeout: {
//! 		onModifyAtkPriority: 5,
//! 		onModifyAtk(atk, attacker, defender) {
//! 			if (!defender.activeTurns) {
//! 				this.debug('Stakeout boost');
//! 				return this.chainModify(2);
//! 			}
//! 		},
//! 		onModifySpAPriority: 5,
//! 		onModifySpA(atk, attacker, defender) {
//! 			if (!defender.activeTurns) {
//! 				this.debug('Stakeout boost');
//! 				return this.chainModify(2);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Stakeout",
//! 		rating: 4.5,
//! 		num: 198,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

/// onModifyAtk(atk, attacker, defender)
/// Doubles Attack against Pokemon that just switched in
///
/// TODO: onModifyAtk handler not yet implemented
/// TODO: Needs defender.activeTurns
/// When implemented, should:
/// 1. Check if defender.activeTurns is 0 (just switched in)
/// 2. Multiply Attack by 2x (ChainModify(2, 1))
pub fn on_modify_atk(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

pub const ON_MODIFY_SP_A_PRIORITY: i32 = 5;

/// onModifySpA(atk, attacker, defender)
/// Doubles Special Attack against Pokemon that just switched in
///
/// TODO: onModifySpA handler not yet implemented
/// TODO: Needs defender.activeTurns
/// When implemented, should:
/// 1. Check if defender.activeTurns is 0 (just switched in)
/// 2. Multiply Special Attack by 2x (ChainModify(2, 1))
pub fn on_modify_sp_a(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

