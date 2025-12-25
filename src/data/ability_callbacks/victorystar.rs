//! Victory Star Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	victorystar: {
//! 		onAnyModifyAccuracyPriority: -1,
//! 		onAnyModifyAccuracy(accuracy, target, source) {
//! 			if (source.isAlly(this.effectState.target) && typeof accuracy === 'number') {
//! 				return this.chainModify([4506, 4096]);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Victory Star",
//! 		rating: 2,
//! 		num: 162,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onAnyModifyAccuracyPriority: -1
pub const ON_ANY_MODIFY_ACCURACY_PRIORITY: i32 = -1;

/// onAnyModifyAccuracy(accuracy, target, source)
pub fn on_any_modify_accuracy(battle: &mut Battle, accuracy: u8, _target: &Pokemon, source: &Pokemon, ability_holder: &Pokemon) -> AbilityHandlerResult {
    // if (source.isAlly(this.effectState.target) && typeof accuracy === 'number')
    if source.is_ally(ability_holder.side_index) && accuracy > 0 {
        // return this.chainModify([4506, 4096]);
        // Increases accuracy by ~10% (4506/4096 ≈ 1.10)
        return AbilityHandlerResult::ChainModify(4506, 4096);
    }
    AbilityHandlerResult::Undefined
}

