//! Competitive Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	competitive: {
//! 		onAfterEachBoost(boost, target, source, effect) {
//! 			if (!source || target.isAlly(source)) {
//! 				return;
//! 			}
//! 			let statsLowered = false;
//! 			let i: BoostID;
//! 			for (i in boost) {
//! 				if (boost[i]! < 0) {
//! 					statsLowered = true;
//! 				}
//! 			}
//! 			if (statsLowered) {
//! 				this.boost({ spa: 2 }, target, target, null, false, true);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Competitive",
//! 		rating: 2.5,
//! 		num: 172,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onAfterEachBoost(boost, target, source, effect)
    /// boost is a map of stat changes that were applied
    pub fn on_after_each_boost(battle: &mut Battle, boosts: &[(&str, i8)], target: &Pokemon, source: Option<&Pokemon>) -> AbilityHandlerResult {
        // if (!source || target.isAlly(source))
        let source = match source {
            Some(s) => s,
            None => return AbilityHandlerResult::Undefined,
        };
        if target.side_index == source.side_index {
            return AbilityHandlerResult::Undefined;
        }
        // let statsLowered = false;
        let mut stats_lowered = false;
        // for (i in boost) { if (boost[i]! < 0) { statsLowered = true; } }
        for (_, change) in boosts {
            if *change < 0 {
                stats_lowered = true;
                break;
            }
        }
        // if (statsLowered)
        if stats_lowered {
            // this.boost({ spa: 2 }, target, target, null, false, true);
            let target_ref = (target.side_index, target.position);
            battle.boost(&[("spa", 2)], target_ref, Some(target_ref), None);
        }
        AbilityHandlerResult::Undefined
    }
