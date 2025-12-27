//! Compound Eyes Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	compoundeyes: {
//! 		onSourceModifyAccuracyPriority: -1,
//! 		onSourceModifyAccuracy(accuracy) {
//! 			if (typeof accuracy !== 'number') return;
//! 			this.debug('compoundeyes - enhancing accuracy');
//! 			return this.chainModify([5325, 4096]);
//! 		},
//! 		flags: {},
//! 		name: "Compound Eyes",
//! 		rating: 3,
//! 		num: 14,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceModifyAccuracyPriority: -1
    pub const ON_SOURCE_MODIFY_ACCURACY_PRIORITY: i32 = -1;

    /// onSourceModifyAccuracy(accuracy)
    /// Boosts accuracy by ~1.3x (5325/4096)
    pub fn on_source_modify_accuracy(accuracy: Option<i32>) -> AbilityHandlerResult {
        // if (typeof accuracy !== 'number') return;
        if accuracy.is_none() {
            return AbilityHandlerResult::Undefined;
        }
        // return this.chainModify([5325, 4096]);
        AbilityHandlerResult::ChainModify(5325, 4096)
    }
