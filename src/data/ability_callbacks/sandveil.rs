//! Sand Veil Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	sandveil: {
//! 		onImmunity(type, pokemon) {
//! 			if (type === 'sandstorm') return false;
//! 		},
//! 		onModifyAccuracyPriority: -1,
//! 		onModifyAccuracy(accuracy) {
//! 			if (typeof accuracy !== 'number') return;
//! 			if (this.field.isWeather('sandstorm')) {
//! 				this.debug('Sand Veil - decreasing accuracy');
//! 				return this.chainModify([3277, 4096]);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Sand Veil",
//! 		rating: 1.5,
//! 		num: 8,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_ACCURACY_PRIORITY: i32 = -1;

/// onImmunity(type, pokemon)
/// Immune to sandstorm damage
pub fn on_immunity(_battle: &mut Battle, immunity_type: &str, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (type === 'sandstorm') return false;
    if immunity_type == "sandstorm" {
        return AbilityHandlerResult::False;
    }
    AbilityHandlerResult::Undefined
}

/// onModifyAccuracy(accuracy)
/// Lowers opponent's accuracy by ~20% in sandstorm
pub fn on_modify_accuracy(battle: &mut Battle, _accuracy: u32) -> AbilityHandlerResult {
    // if (typeof accuracy !== 'number') return;
    // Note: In Rust, accuracy is always u32, so this check is implicit

    // if (this.field.isWeather('sandstorm'))
    if battle.field.is_weather("sandstorm") {
        // this.debug('Sand Veil - decreasing accuracy');
        // return this.chainModify([3277, 4096]);
        return AbilityHandlerResult::ChainModify(3277, 4096); // ~0.8x
    }
    AbilityHandlerResult::Undefined
}

