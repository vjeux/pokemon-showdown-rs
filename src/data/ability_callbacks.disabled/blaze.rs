//! Blaze Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	blaze: {
//! 		onModifyAtkPriority: 5,
//! 		onModifyAtk(atk, attacker, defender, move) {
//! 			if (move.type === 'Fire' && attacker.hp <= attacker.maxhp / 3) {
//! 				this.debug('Blaze boost');
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		onModifySpAPriority: 5,
//! 		onModifySpA(atk, attacker, defender, move) {
//! 			if (move.type === 'Fire' && attacker.hp <= attacker.maxhp / 3) {
//! 				this.debug('Blaze boost');
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Blaze",
//! 		rating: 2,
//! 		num: 66,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

// onModifyAtkPriority: 5,
    pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;

    /// onModifyAtk(atk, attacker, defender, move)
    pub fn on_modify_atk(_atk: u32, attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        // if (move.type === 'Fire' && attacker.hp <= attacker.maxhp / 3)
        if move_.move_type == "Fire" && attacker.hp <= attacker.maxhp / 3 {
            // this.debug('Blaze boost');
            // return this.chainModify(1.5);
            return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
        }
        AbilityHandlerResult::Undefined
    }

    // onModifySpAPriority: 5,
    pub const ON_MODIFY_SPA_PRIORITY: i32 = 5;

    /// onModifySpA(atk, attacker, defender, move)
    pub fn on_modify_spa(_atk: u32, attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        // if (move.type === 'Fire' && attacker.hp <= attacker.maxhp / 3)
        if move_.move_type == "Fire" && attacker.hp <= attacker.maxhp / 3 {
            // this.debug('Blaze boost');
            // return this.chainModify(1.5);
            return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
        }
        AbilityHandlerResult::Undefined
    }
