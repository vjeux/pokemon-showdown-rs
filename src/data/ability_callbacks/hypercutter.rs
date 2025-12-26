//! Hyper Cutter Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	hypercutter: {
//! 		onTryBoost(boost, target, source, effect) {
//! 			if (source && target === source) return;
//! 			if (boost.atk && boost.atk < 0) {
//! 				delete boost.atk;
//! 				if (!(effect as ActiveMove).secondaries) {
//! 					this.add("-fail", target, "unboost", "Attack", "[from] ability: Hyper Cutter", `[of] ${target}`);
//! 				}
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Hyper Cutter",
//! 		rating: 1.5,
//! 		num: 52,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryBoost(boost, target, source, effect)
/// Prevents Attack stat from being lowered by opponents
pub fn on_try_boost(battle: &mut Battle, boost: &mut std::collections::HashMap<String, i8>, target: &Pokemon, source: Option<&Pokemon>, effect_id: &str, has_secondaries: bool) -> AbilityHandlerResult {
    // if (source && target === source) return;
    if let Some(src) = source {
        if (target.side_index, target.position) == (src.side_index, src.position) {
            return AbilityHandlerResult::Undefined;
        }
    }
    // if (boost.atk && boost.atk < 0)
    if let Some(&atk_boost) = boost.get("atk") {
        if atk_boost < 0 {
            // delete boost.atk;
            boost.remove("atk");
            // if (!(effect as ActiveMove).secondaries)
            if !has_secondaries {
                // this.add("-fail", target, "unboost", "Attack", "[from] ability: Hyper Cutter", `[of] ${target}`);
                battle.add("-fail", &[
                    Arg::Pokemon(target),
                    Arg::Str("unboost"),
                    Arg::Str("Attack"),
                    Arg::Str("[from] ability: Hyper Cutter"),
                    Arg::Str(&format!("[of] {}", target.name))
                ]);
            }
        }
    }
    AbilityHandlerResult::Undefined
}

