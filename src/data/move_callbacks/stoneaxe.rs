//! Stone Axe Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	stoneaxe: {
//! 		num: 830,
//! 		accuracy: 90,
//! 		basePower: 65,
//! 		category: "Physical",
//! 		name: "Stone Axe",
//! 		pp: 15,
//! 		priority: 0,
//! 		flags: { contact: 1, protect: 1, mirror: 1, metronome: 1, slicing: 1 },
//! 		onAfterHit(target, source, move) {
//! 			if (!move.hasSheerForce && source.hp) {
//! 				for (const side of source.side.foeSidesWithConditions()) {
//! 					side.addSideCondition('stealthrock');
//! 				}
//! 			}
//! 		},
//! 		onAfterSubDamage(damage, target, source, move) {
//! 			if (!move.hasSheerForce && source.hp) {
//! 				for (const side of source.side.foeSidesWithConditions()) {
//! 					side.addSideCondition('stealthrock');
//! 				}
//! 			}
//! 		},
//! 		secondary: {}, // Sheer Force-boosted
//! 		target: "normal",
//! 		type: "Rock",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onAfterHit(...)
pub fn on_after_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onAfterSubDamage(...)
pub fn on_after_sub_damage(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

