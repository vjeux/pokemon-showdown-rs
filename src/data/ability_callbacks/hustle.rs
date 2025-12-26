//! Hustle Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	hustle: {
//! 		// This should be applied directly to the stat as opposed to chaining with the others
//! 		onModifyAtkPriority: 5,
//! 		onModifyAtk(atk) {
//! 			return this.modify(atk, 1.5);
//! 		},
//! 		onSourceModifyAccuracyPriority: -1,
//! 		onSourceModifyAccuracy(accuracy, target, source, move) {
//! 			if (move.category === 'Physical' && typeof accuracy === 'number') {
//! 				return this.chainModify([3277, 4096]);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Hustle",
//! 		rating: 3.5,
//! 		num: 55,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyAtkPriority: 5
pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;
/// onSourceModifyAccuracyPriority: -1
pub const ON_SOURCE_MODIFY_ACCURACY_PRIORITY: i32 = -1;

/// onModifyAtk(atk)
/// Multiplies Attack by 1.5
pub fn on_modify_atk(atk: u32) -> AbilityHandlerResult {
    // return this.modify(atk, 1.5);
    // Note: this.modify() directly multiplies, not chainModify
    AbilityHandlerResult::Number((atk as f64 * 1.5) as i32)
}

/// onSourceModifyAccuracy(accuracy, target, source, move)
/// Reduces accuracy of Physical moves
pub fn on_source_modify_accuracy(accuracy: Option<u32>, _target: &Pokemon, _source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.category === 'Physical' && typeof accuracy === 'number')
    if move_.category == MoveCategory::Physical && accuracy.is_some() {
        // return this.chainModify([3277, 4096]);
        return AbilityHandlerResult::ChainModify(3277, 4096); // ~0.8x accuracy
    }
    AbilityHandlerResult::Undefined
}

