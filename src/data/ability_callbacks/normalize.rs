//! Normalize Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	normalize: {
//! 		onModifyTypePriority: 1,
//! 		onModifyType(move, pokemon) {
//! 			const noModifyType = [
//! 				'hiddenpower', 'judgment', 'multiattack', 'naturalgift', 'revelationdance', 'struggle', 'technoblast', 'terrainpulse', 'weatherball',
//! 			];
//! 			if (!(move.isZ && move.category !== 'Status') &&
//! 				// TODO: Figure out actual interaction
//! 				(!noModifyType.includes(move.id) || this.activeMove?.isMax) && !(move.name === 'Tera Blast' && pokemon.terastallized)) {
//! 				move.type = 'Normal';
//! 				move.typeChangerBoosted = this.effect;
//! 			}
//! 		},
//! 		onBasePowerPriority: 23,
//! 		onBasePower(basePower, pokemon, target, move) {
//! 			if (move.typeChangerBoosted === this.effect) return this.chainModify([4915, 4096]);
//! 		},
//! 		flags: {},
//! 		name: "Normalize",
//! 		rating: 0,
//! 		num: 96,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_TYPE_PRIORITY: i32 = 1;

/// onModifyType(move, pokemon)
/// Changes most moves to Normal-type
///
/// TODO: onModifyType handler not yet implemented
/// TODO: Needs move.isZ, move.category, move.typeChangerBoosted
/// TODO: Needs pokemon.terastallized, activeMove.isMax
/// When implemented, should:
/// 1. Skip if Z-move (non-status), or certain special moves
/// 2. Skip Hidden Power, Judgment, Multi-Attack, Natural Gift, etc.
/// 3. Skip Tera Blast when terastallized
/// 4. Set move.type = 'Normal'
/// 5. Set move.typeChangerBoosted = this.effect
pub fn on_modify_type(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

pub const ON_BASE_POWER_PRIORITY: i32 = 23;

/// onBasePower(basePower, pokemon, target, move)
/// Boosts normalized moves by ~1.2x
///
/// TODO: onBasePower handler not yet implemented
/// TODO: Needs move.typeChangerBoosted check
/// When implemented, should:
/// 1. Check if move.typeChangerBoosted === this.effect
/// 2. Return chainModify(4915, 4096) for ~1.2x boost
pub fn on_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

