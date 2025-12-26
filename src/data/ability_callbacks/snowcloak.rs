//! Snow Cloak Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	snowcloak: {
//! 		onImmunity(type, pokemon) {
//! 			if (type === 'hail') return false;
//! 		},
//! 		onModifyAccuracyPriority: -1,
//! 		onModifyAccuracy(accuracy) {
//! 			if (typeof accuracy !== 'number') return;
//! 			if (this.field.isWeather(['hail', 'snowscape'])) {
//! 				this.debug('Snow Cloak - decreasing accuracy');
//! 				return this.chainModify([3277, 4096]);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Snow Cloak",
//! 		rating: 1.5,
//! 		num: 81,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onImmunity(type, pokemon)
/// Immune to hail damage
pub fn on_immunity(_battle: &mut Battle, immunity_type: &str, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (type === 'hail') return false;
    if immunity_type == "hail" {
        return AbilityHandlerResult::False;
    }
    AbilityHandlerResult::Undefined
}

/// onModifyAccuracyPriority: -1
pub const ON_MODIFY_ACCURACY_PRIORITY: i32 = -1;

/// onModifyAccuracy(accuracy)
/// Lowers opponent's accuracy by ~20% in hail/snowscape
pub fn on_modify_accuracy(battle: &mut Battle, _accuracy: u32) -> AbilityHandlerResult {
    // if (typeof accuracy !== 'number') return;
    // Note: In Rust, accuracy is always u32, so this check is implicit

    // if (this.field.isWeather(['hail', 'snowscape']))
    let weather = battle.field.get_weather();
    if *weather == ID::new("hail") || *weather == ID::new("snowscape") {
        // this.debug('Snow Cloak - decreasing accuracy');
        // return this.chainModify([3277, 4096]);
        return AbilityHandlerResult::ChainModify(3277, 4096); // ~0.8x
    }
    AbilityHandlerResult::Undefined
}

