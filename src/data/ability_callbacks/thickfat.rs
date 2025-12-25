//! Thick Fat Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	thickfat: {
//! 		onSourceModifyAtkPriority: 6,
//! 		onSourceModifyAtk(atk, attacker, defender, move) {
//! 			if (move.type === 'Ice' || move.type === 'Fire') {
//! 				this.debug('Thick Fat weaken');
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		onSourceModifySpAPriority: 5,
//! 		onSourceModifySpA(atk, attacker, defender, move) {
//! 			if (move.type === 'Ice' || move.type === 'Fire') {
//! 				this.debug('Thick Fat weaken');
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Thick Fat",
//! 		rating: 3.5,
//! 		num: 47,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceModifyAtkPriority: 6
pub const ON_SOURCE_MODIFY_ATK_PRIORITY: i32 = 6;
/// onSourceModifySpAPriority: 5
pub const ON_SOURCE_MODIFY_SPA_PRIORITY: i32 = 5;

/// onSourceModifyAtk(atk, attacker, defender, move)
/// Halves Ice-type and Fire-type attack damage
pub fn on_source_modify_atk(_atk: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Ice' || move.type === 'Fire')
    if move_.move_type == "Ice" || move_.move_type == "Fire" {
        // this.debug('Thick Fat weaken');
        // return this.chainModify(0.5);
        return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
    }
    AbilityHandlerResult::Undefined
}

/// onSourceModifySpA(atk, attacker, defender, move)
/// Halves Ice-type and Fire-type special attack damage
pub fn on_source_modify_sp_a(_spa: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.type === 'Ice' || move.type === 'Fire')
    if move_.move_type == "Ice" || move_.move_type == "Fire" {
        // this.debug('Thick Fat weaken');
        // return this.chainModify(0.5);
        return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
    }
    AbilityHandlerResult::Undefined
}

