//! Shadow Shield Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	shadowshield: {
//! 		onSourceModifyDamage(damage, source, target, move) {
//! 			if (target.hp >= target.maxhp) {
//! 				this.debug('Shadow Shield weaken');
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Shadow Shield",
//! 		rating: 3.5,
//! 		num: 231,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceModifyDamage(damage, source, target, move)
pub fn on_source_modify_damage(_damage: u32, _source: &Pokemon, target: &Pokemon, _move: &MoveDef) -> AbilityHandlerResult {
    // if (target.hp >= target.maxhp)
    if target.hp >= target.maxhp {
        // this.debug('Shadow Shield weaken');
        // return this.chainModify(0.5);
        return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
    }
    AbilityHandlerResult::Undefined
}
