//! Big Pecks Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	bigpecks: {
//! 		onTryBoost(boost, target, source, effect) {
//! 			if (source && target === source) return;
//! 			if (boost.def && boost.def < 0) {
//! 				delete boost.def;
//! 				if (!(effect as ActiveMove).secondaries && effect.id !== 'octolock') {
//! 					this.add("-fail", target, "unboost", "Defense", "[from] ability: Big Pecks", `[of] ${target}`);
//! 				}
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Big Pecks",
//! 		rating: 0.5,
//! 		num: 145,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryBoost(boost, target, source, effect)
    /// Prevents Defense stat from being lowered
    pub fn on_try_boost(battle: &mut Battle, boost: &mut std::collections::HashMap<String, i8>, target: &Pokemon, source: Option<&Pokemon>, effect_id: &str, has_secondaries: bool) -> AbilityHandlerResult {
        // If source exists and target is the source, do nothing
        if let Some(src) = source {
            if (target.side_index, target.position) == (src.side_index, src.position) {
                return AbilityHandlerResult::Undefined;
            }
        }
        // Check if Defense is being lowered
        if let Some(&def_boost) = boost.get("def") {
            if def_boost < 0 {
                boost.remove("def");
                if !has_secondaries && effect_id != "octolock" {
                    battle.add("-fail", &[Arg::Pokemon(target), Arg::Str("unboost"), Arg::Str("Defense"), Arg::Str("[from] ability: Big Pecks"), Arg::Str(&format!("[of] {}", target.name))]);
                }
            }
        }
        AbilityHandlerResult::Undefined
    }
