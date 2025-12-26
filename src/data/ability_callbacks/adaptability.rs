//! Adaptability Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	adaptability: {
//! 		onModifySTAB(stab, source, target, move) {
//! 			if (move.forceSTAB || source.hasType(move.type)) {
//! 				if (stab === 2) {
//! 					return 2.25;
//! 				}
//! 				return 2;
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Adaptability",
//! 		rating: 4,
//! 		num: 91,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifySTAB(stab, source, target, move)
    pub fn on_modify_stab(stab: f64, source: &Pokemon, _target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        // if (move.forceSTAB || source.hasType(move.type))
        if move_.force_stab || source.has_type(&move_.move_type) {
            // if (stab === 2)
            if stab == 2.0 {
                // return 2.25;
                return AbilityHandlerResult::Number(225); // 2.25 represented as 225 (x100)
            }
            // return 2;
            return AbilityHandlerResult::Number(200); // 2.0 represented as 200 (x100)
        }
        AbilityHandlerResult::Undefined
    }
