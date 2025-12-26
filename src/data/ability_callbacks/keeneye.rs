//! Keen Eye Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	keeneye: {
//! 		onTryBoost(boost, target, source, effect) {
//! 			if (source && target === source) return;
//! 			if (boost.accuracy && boost.accuracy < 0) {
//! 				delete boost.accuracy;
//! 				if (!(effect as ActiveMove).secondaries) {
//! 					this.add("-fail", target, "unboost", "accuracy", "[from] ability: Keen Eye", `[of] ${target}`);
//! 				}
//! 			}
//! 		},
//! 		onModifyMove(move) {
//! 			move.ignoreEvasion = true;
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Keen Eye",
//! 		rating: 0.5,
//! 		num: 51,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryBoost(boost, target, source, effect)
/// Prevents accuracy reduction
pub fn on_try_boost(battle: &mut Battle, boost: &mut std::collections::HashMap<String, i8>, target: &Pokemon, source: Option<&Pokemon>, effect_id: &str, has_secondaries: bool) -> AbilityHandlerResult {
    // if (source && target === source) return;
    if let Some(src) = source {
        if (target.side_index, target.position) == (src.side_index, src.position) {
            return AbilityHandlerResult::Undefined;
        }
    }
    // if (boost.accuracy && boost.accuracy < 0)
    if let Some(&acc_boost) = boost.get("accuracy") {
        if acc_boost < 0 {
            // delete boost.accuracy;
            boost.remove("accuracy");
            // if (!(effect as ActiveMove).secondaries)
            if !has_secondaries {
                // this.add("-fail", target, "unboost", "accuracy", "[from] ability: Keen Eye", `[of] ${target}`);
                battle.add("-fail", &[
                    Arg::Pokemon(target),
                    Arg::Str("unboost"),
                    Arg::Str("accuracy"),
                    Arg::Str("[from] ability: Keen Eye"),
                    Arg::Str(&format!("[of] {}", target.name))
                ]);
            }
        }
    }
    AbilityHandlerResult::Undefined
}

/// onModifyMove(move)
/// Makes moves ignore evasion
pub fn on_modify_move(move_: &mut MoveDef, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // move.ignoreEvasion = true;
    move_.ignores_evasion = true;
    AbilityHandlerResult::Undefined
}

