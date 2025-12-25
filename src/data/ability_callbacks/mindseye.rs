//! Mind's Eye Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	mindseye: {
//! 		onTryBoost(boost, target, source, effect) {
//! 			if (source && target === source) return;
//! 			if (boost.accuracy && boost.accuracy < 0) {
//! 				delete boost.accuracy;
//! 				if (!(effect as ActiveMove).secondaries) {
//! 					this.add("-fail", target, "unboost", "accuracy", "[from] ability: Mind's Eye", `[of] ${target}`);
//! 				}
//! 			}
//! 		},
//! 		onModifyMovePriority: -5,
//! 		onModifyMove(move) {
//! 			move.ignoreEvasion = true;
//! 			if (!move.ignoreImmunity) move.ignoreImmunity = {};
//! 			if (move.ignoreImmunity !== true) {
//! 				move.ignoreImmunity['Fighting'] = true;
//! 				move.ignoreImmunity['Normal'] = true;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Mind's Eye",
//! 		rating: 0,
//! 		num: 300,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryBoost(boost, target, source, effect)
/// Prevents accuracy drops
pub fn on_try_boost(battle: &mut Battle, boost: &mut std::collections::HashMap<String, i8>, target: &Pokemon, source: Option<&Pokemon>, _effect_id: &str, has_secondaries: bool) -> AbilityHandlerResult {
    // if (source && target === source) return;
    if let Some(src) = source {
        if (target.side_index, target.position) == (src.side_index, src.position) {
            return AbilityHandlerResult::Undefined;
        }
    }
    // if (boost.accuracy && boost.accuracy < 0)
    if let Some(&accuracy_boost) = boost.get("accuracy") {
        if accuracy_boost < 0 {
            // delete boost.accuracy;
            boost.remove("accuracy");
            // if (!(effect as ActiveMove).secondaries)
            if !has_secondaries {
                // this.add("-fail", target, "unboost", "accuracy", "[from] ability: Mind's Eye", `[of] ${target}`);
                battle.add("-fail", &[
                    Arg::Pokemon(target),
                    Arg::Str("unboost"),
                    Arg::Str("accuracy"),
                    Arg::Str("[from] ability: Mind's Eye"),
                    Arg::Str(&format!("[of] {}", target.position))
                ]);
            }
        }
    }
    AbilityHandlerResult::Undefined
}

pub const ON_MODIFY_MOVE_PRIORITY: i32 = -5;

/// onModifyMove(move)
/// Ignores evasion and Ghost immunity for Normal/Fighting moves
///
/// TODO: onModifyMove handler not yet implemented
/// TODO: Needs move.ignoreEvasion, move.ignoreImmunity fields
/// When implemented, should:
/// 1. Set move.ignoreEvasion = true
/// 2. Set move.ignoreImmunity['Fighting'] = true
/// 3. Set move.ignoreImmunity['Normal'] = true
pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

