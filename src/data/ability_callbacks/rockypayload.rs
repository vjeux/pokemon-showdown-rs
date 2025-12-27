//! Rocky Payload Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	rockypayload: {
//! 		onModifyAtkPriority: 5,
//! 		onModifyAtk(atk, attacker, defender, move) {
//! 			if (move.type === 'Rock') {
//! 				this.debug('Rocky Payload boost');
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		onModifySpAPriority: 5,
//! 		onModifySpA(atk, attacker, defender, move) {
//! 			if (move.type === 'Rock') {
//! 				this.debug('Rocky Payload boost');
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Rocky Payload",
//! 		rating: 3.5,
//! 		num: 276,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

/// onModifyAtk(atk, attacker, defender, move)
/// Boosts Rock-type moves by 1.5x
pub fn on_modify_atk(_atk: i32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Rock')
    if move_.move_type == "Rock" {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}

pub const ON_MODIFY_SP_A_PRIORITY: i32 = 5;

/// onModifySpA(atk, attacker, defender, move)
/// Boosts Rock-type moves by 1.5x
pub fn on_modify_sp_a(_atk: i32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Rock')
    if move_.move_type == "Rock" {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}

