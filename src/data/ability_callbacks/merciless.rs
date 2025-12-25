//! Merciless Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	merciless: {
//! 		onModifyCritRatio(critRatio, source, target) {
//! 			if (target && ['psn', 'tox'].includes(target.status)) return 5;
//! 		},
//! 		flags: {},
//! 		name: "Merciless",
//! 		rating: 1.5,
//! 		num: 196,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyCritRatio(critRatio, source, target)
/// Always crits poisoned foes
pub fn on_modify_crit_ratio(_crit_ratio: i32, _source: &Pokemon, target: Option<&Pokemon>) -> AbilityHandlerResult {
    // if (target && ['psn', 'tox'].includes(target.status)) return 5;
    if let Some(target) = target {
        let status = target.status.as_str();
        if status == "psn" || status == "tox" {
            return AbilityHandlerResult::Number(5); // Guaranteed crit
        }
    }
    AbilityHandlerResult::Undefined
}

