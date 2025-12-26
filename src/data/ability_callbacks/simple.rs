//! Simple Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	simple: {
//! 		onChangeBoost(boost, target, source, effect) {
//! 			if (effect && effect.id === 'zpower') return;
//! 			let i: BoostID;
//! 			for (i in boost) {
//! 				boost[i]! *= 2;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Simple",
//! 		rating: 4,
//! 		num: 86,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onChangeBoost(boost, target, source, effect)
/// Doubles all stat changes (both positive and negative)
///
/// TODO: onChangeBoost handler not yet implemented in battle system
/// TODO: Needs boost object manipulation, effect.id checking
/// When implemented, should:
/// 1. Skip if effect is 'zpower' (Z-Move stat boosts aren't doubled)
/// 2. Loop through all stat changes in boost object
/// 3. Multiply each boost value by 2
pub fn on_change_boost(boost: &mut std::collections::HashMap<String, i8>, _target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (effect && effect.id === 'zpower') return;
    if effect.id == "zpower" {
        return AbilityHandlerResult::Undefined;
    }

    // let i: BoostID;
    // for (i in boost) {
    //     boost[i]! *= 2;
    // }
    for (_stat, value) in boost.iter_mut() {
        *value = (*value * 2).clamp(-6, 6);
    }

    AbilityHandlerResult::Undefined
}

