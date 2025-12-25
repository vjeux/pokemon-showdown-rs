//! Sheer Force Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	sheerforce: {
//! 		onModifyMove(move, pokemon) {
//! 			if (move.secondaries) {
//! 				delete move.secondaries;
//! 				// Technically not a secondary effect, but it is negated
//! 				delete move.self;
//! 				if (move.id === 'clangoroussoulblaze') delete move.selfBoost;
//! 				// Actual negation of `AfterMoveSecondary` effects implemented in scripts.js
//! 				move.hasSheerForce = true;
//! 			}
//! 		},
//! 		onBasePowerPriority: 21,
//! 		onBasePower(basePower, pokemon, target, move) {
//! 			if (move.hasSheerForce) return this.chainModify([5325, 4096]);
//! 		},
//! 		flags: {},
//! 		name: "Sheer Force",
//! 		rating: 3.5,
//! 		num: 125,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyMove(move, pokemon)
/// Removes secondary effects from moves to boost power
///
/// TODO: onModifyMove handler not yet implemented
/// TODO: Needs move.secondaries, delete move.secondaries, delete move.self, move.selfBoost, move.hasSheerForce
/// When implemented, should:
/// 1. If move has secondaries
/// 2. Delete move.secondaries and move.self
/// 3. For Clangor Soul Blaze, also delete move.selfBoost
/// 4. Set move.hasSheerForce = true
pub fn on_modify_move(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

pub const ON_BASE_POWER_PRIORITY: i32 = 21;

/// onBasePower(basePower, pokemon, target, move)
/// Boosts moves with removed secondaries by 1.3x
///
/// TODO: onBasePower handler not yet implemented
/// TODO: Needs move.hasSheerForce
/// When implemented, should:
/// 1. If move.hasSheerForce is true
/// 2. Multiply base power by 5325/4096 (~1.3x)
pub fn on_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

