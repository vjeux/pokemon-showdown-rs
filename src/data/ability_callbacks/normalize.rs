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
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_TYPE_PRIORITY: i32 = 1;

/// Moves that don't get type-changed
const NO_MODIFY_TYPE: &[&str] = &[
    "hiddenpower", "judgment", "multiattack", "naturalgift", "revelationdance",
    "struggle", "technoblast", "terrainpulse", "weatherball",
];

/// onModifyType(move, pokemon)
/// Changes most moves to Normal-type
pub fn on_modify_type(move_: &mut MoveDef, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (!(move.isZ && move.category !== 'Status') &&
    //     (!noModifyType.includes(move.id) || this.activeMove?.isMax) &&
    //     !(move.name === 'Tera Blast' && pokemon.terastallized))
    if !(move_.is_z && move_.category != MoveCategory::Status)
        && (!NO_MODIFY_TYPE.contains(&move_.id.as_str()) || move_.is_max)
        && !(move_.name == "Tera Blast" && pokemon.terastallized.is_some())
    {
        // move.type = 'Normal';
        move_.move_type = "Normal".to_string();
        // move.typeChangerBoosted = this.effect;
        move_.type_changer_boosted = true;
    }
    AbilityHandlerResult::Undefined
}

pub const ON_BASE_POWER_PRIORITY: i32 = 23;

/// onBasePower(basePower, pokemon, target, move)
/// Boosts normalized moves by ~1.2x
pub fn on_base_power(_base_power: u32, _pokemon: &Pokemon, _target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.typeChangerBoosted === this.effect) return this.chainModify([4915, 4096]);
    if move_.type_changer_boosted {
        return AbilityHandlerResult::ChainModify(4915, 4096); // ~1.2x
    }
    AbilityHandlerResult::Undefined
}

