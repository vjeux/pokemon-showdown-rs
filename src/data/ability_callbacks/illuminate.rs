//! Illuminate Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	illuminate: {
//! 		onTryBoost(boost, target, source, effect) {
//! 			if (source && target === source) return;
//! 			if (boost.accuracy && boost.accuracy < 0) {
//! 				delete boost.accuracy;
//! 				if (!(effect as ActiveMove).secondaries) {
//! 					this.add("-fail", target, "unboost", "accuracy", "[from] ability: Illuminate", `[of] ${target}`);
//! 				}
//! 			}
//! 		},
//! 		onModifyMove(move) {
//! 			move.ignoreEvasion = true;
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Illuminate",
//! 		rating: 0.5,
//! 		num: 35,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryBoost(boost, target, source, effect)
/// Prevents accuracy reduction from opponents
pub fn on_try_boost(battle: &mut Battle, boost: &mut std::collections::HashMap<String, i8>, target: &Pokemon, source: Option<&Pokemon>, effect_id: &str, has_secondaries: bool) -> AbilityHandlerResult {
    // Return early if source && target === source (self-inflicted)
    if let Some(src) = source {
        if (target.side_index, target.position) == (src.side_index, src.position) {
            return AbilityHandlerResult::Undefined;
        }
    }
    // Check if boost.accuracy exists and is negative
    if let Some(&acc_boost) = boost.get("accuracy") {
        if acc_boost < 0 {
            // Delete boost.accuracy to prevent the reduction
            boost.remove("accuracy");
            // Add fail message if not from a secondary effect
            if !has_secondaries {
                battle.add("-fail", &[
                    Arg::Pokemon(target),
                    Arg::Str("unboost"),
                    Arg::Str("accuracy"),
                    Arg::Str("[from] ability: Illuminate"),
                    Arg::Str(&format!("[of] {}", target.name))
                ]);
            }
        }
    }
    AbilityHandlerResult::Undefined
}

/// onModifyMove(move)
/// Makes all moves ignore evasion
pub fn on_modify_move(move_: &mut MoveDef, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // Set move.ignoreEvasion = true
    move_.ignores_evasion = true;
    AbilityHandlerResult::Undefined
}

