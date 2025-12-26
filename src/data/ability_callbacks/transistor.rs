//! Transistor Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	transistor: {
//! 		onModifyAtkPriority: 5,
//! 		onModifyAtk(atk, attacker, defender, move) {
//! 			if (move.type === 'Electric') {
//! 				this.debug('Transistor boost');
//! 				return this.chainModify([5325, 4096]);
//! 			}
//! 		},
//! 		onModifySpAPriority: 5,
//! 		onModifySpA(atk, attacker, defender, move) {
//! 			if (move.type === 'Electric') {
//! 				this.debug('Transistor boost');
//! 				return this.chainModify([5325, 4096]);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Transistor",
//! 		rating: 3.5,
//! 		num: 262,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

/// onModifyAtk(atk, attacker, defender, move)
/// Boosts Electric-type moves by 1.3x
pub fn on_modify_atk(_atk: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Electric')
    if move_.move_type == "Electric" {
        // return this.chainModify([5325, 4096]);
        return AbilityHandlerResult::ChainModify(5325, 4096); // ~1.3x
    }
    AbilityHandlerResult::Undefined
}

pub const ON_MODIFY_SP_A_PRIORITY: i32 = 5;

/// onModifySpA(atk, attacker, defender, move)
/// Boosts Electric-type moves by 1.3x
pub fn on_modify_sp_a(_atk: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Electric')
    if move_.move_type == "Electric" {
        // return this.chainModify([5325, 4096]);
        return AbilityHandlerResult::ChainModify(5325, 4096); // ~1.3x
    }
    AbilityHandlerResult::Undefined
}

