//! Tangled Feet Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	tangledfeet: {
//! 		onModifyAccuracyPriority: -1,
//! 		onModifyAccuracy(accuracy, target) {
//! 			if (typeof accuracy !== 'number') return;
//! 			if (target?.volatiles['confusion']) {
//! 				this.debug('Tangled Feet - decreasing accuracy');
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Tangled Feet",
//! 		rating: 1,
//! 		num: 77,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyAccuracyPriority: -1
pub const ON_MODIFY_ACCURACY_PRIORITY: i32 = -1;

/// onModifyAccuracy(accuracy, target)
/// Halves opponent accuracy when confused
pub fn on_modify_accuracy(_battle: &mut Battle, _accuracy: u32, target: &Pokemon) -> AbilityHandlerResult {
    // if (typeof accuracy !== 'number') return;
    // Note: In Rust, accuracy is always u32, so this check is implicit

    // if (target?.volatiles['confusion'])
    if target.has_volatile(&ID::new("confusion")) {
        // this.debug('Tangled Feet - decreasing accuracy');
        // return this.chainModify(0.5);
        return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
    }
    AbilityHandlerResult::Undefined
}


