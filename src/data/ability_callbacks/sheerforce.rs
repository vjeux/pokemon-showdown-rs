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
pub fn on_modify_move(move_: &mut MoveDef, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (move.secondaries)
    if !move_.secondaries.is_empty() {
        // delete move.secondaries;
        move_.secondaries.clear();
        // delete move.self;
        move_.self_boosts = None;
        move_.self_drops = None;
        move_.self_volatile = None;
        // if (move.id === 'clangoroussoulblaze') delete move.selfBoost;
        // Note: In Rust we just cleared self_boosts above which handles this

        // move.hasSheerForce = true;
        move_.has_sheer_force = true;
    }
    AbilityHandlerResult::Undefined
}

pub const ON_BASE_POWER_PRIORITY: i32 = 21;

/// onBasePower(basePower, pokemon, target, move)
/// Boosts moves with removed secondaries by 1.3x
pub fn on_base_power(_base_power: u32, _pokemon: &Pokemon, _target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.hasSheerForce) return this.chainModify([5325, 4096]);
    if move_.has_sheer_force {
        return AbilityHandlerResult::ChainModify(5325, 4096); // ~1.3x
    }
    AbilityHandlerResult::Undefined
}

