//! Scrappy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	scrappy: {
//! 		onModifyMovePriority: -5,
//! 		onModifyMove(move) {
//! 			if (!move.ignoreImmunity) move.ignoreImmunity = {};
//! 			if (move.ignoreImmunity !== true) {
//! 				move.ignoreImmunity['Fighting'] = true;
//! 				move.ignoreImmunity['Normal'] = true;
//! 			}
//! 		},
//! 		onTryBoost(boost, target, source, effect) {
//! 			if (effect.name === 'Intimidate' && boost.atk) {
//! 				delete boost.atk;
//! 				this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Scrappy', `[of] ${target}`);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Scrappy",
//! 		rating: 3,
//! 		num: 113,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_MOVE_PRIORITY: i32 = -5;

/// onModifyMove(move)
/// Allows Fighting and Normal moves to hit Ghost types
pub fn on_modify_move(_battle: &mut Battle, move_: &mut MoveDef) -> AbilityHandlerResult {
    // if (!move.ignoreImmunity) move.ignoreImmunity = {};
    // if (move.ignoreImmunity !== true)
    if !move_.ignore_immunity {
        // move.ignoreImmunity['Fighting'] = true;
        // move.ignoreImmunity['Normal'] = true;
        if !move_.ignore_immunity_types.contains(&"Fighting".to_string()) {
            move_.ignore_immunity_types.push("Fighting".to_string());
        }
        if !move_.ignore_immunity_types.contains(&"Normal".to_string()) {
            move_.ignore_immunity_types.push("Normal".to_string());
        }
    }
    AbilityHandlerResult::Undefined
}

/// onTryBoost(boost, target, source, effect)
/// Blocks Intimidate's Attack drop
pub fn on_try_boost(battle: &mut Battle, boost: &mut std::collections::HashMap<String, i8>, target: &Pokemon, _source: Option<&Pokemon>, effect_id: &str, _has_secondaries: bool) -> AbilityHandlerResult {
    // if (effect.name === 'Intimidate' && boost.atk)
    if effect_id == "intimidate" && boost.contains_key("atk") {
        // delete boost.atk;
        boost.remove("atk");
        // this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Scrappy', `[of] ${target}`);
        battle.add("-fail", &[
            Arg::Pokemon(target),
            Arg::Str("unboost"),
            Arg::Str("Attack"),
            Arg::Str("[from] ability: Scrappy"),
            Arg::Str(&format!("[of] {}", target.position))
        ]);
    }
    AbilityHandlerResult::Undefined
}

