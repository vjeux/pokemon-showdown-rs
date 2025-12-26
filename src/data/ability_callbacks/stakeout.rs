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
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

/// onModifyAtk(atk, attacker, defender)
/// Doubles Attack against Pokemon that just switched in
pub fn on_modify_atk(_atk: u32, _attacker: &Pokemon, defender: &Pokemon, _move: &MoveDef) -> AbilityHandlerResult {
    // if (!defender.activeTurns)
    if defender.active_turns == 0 {
        // this.debug('Stakeout boost');
        // return this.chainModify(2);
        return AbilityHandlerResult::ChainModify(8192, 4096); // 2x
    }
    AbilityHandlerResult::Undefined
}

pub const ON_MODIFY_SP_A_PRIORITY: i32 = 5;

/// onModifySpA(atk, attacker, defender)
/// Doubles Special Attack against Pokemon that just switched in
pub fn on_modify_sp_a(_spa: u32, _attacker: &Pokemon, defender: &Pokemon, _move: &MoveDef) -> AbilityHandlerResult {
    // if (!defender.activeTurns)
    if defender.active_turns == 0 {
        // this.debug('Stakeout boost');
        // return this.chainModify(2);
        return AbilityHandlerResult::ChainModify(8192, 4096); // 2x
    }
    AbilityHandlerResult::Undefined
}

