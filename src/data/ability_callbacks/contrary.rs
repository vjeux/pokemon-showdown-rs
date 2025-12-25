//! Contrary Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	contrary: {
//! 		onChangeBoost(boost, target, source, effect) {
//! 			if (effect && effect.id === 'zpower') return;
//! 			let i: BoostID;
//! 			for (i in boost) {
//! 				boost[i]! *= -1;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Contrary",
//! 		rating: 4.5,
//! 		num: 126,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onChangeBoost(boost, target, source, effect)
    /// Inverts all stat changes
    pub fn on_change_boost(boost: &mut std::collections::HashMap<String, i8>, effect_id: Option<&str>) -> AbilityHandlerResult {
        // if (effect && effect.id === 'zpower') return;
        if let Some(id) = effect_id {
            if id == "zpower" {
                return AbilityHandlerResult::Undefined;
            }
        }
        // for (i in boost) { boost[i]! *= -1; }
        for (_, val) in boost.iter_mut() {
            *val *= -1;
        }
        AbilityHandlerResult::Undefined
    }
