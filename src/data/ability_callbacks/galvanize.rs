//! Galvanize Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	galvanize: {
//! 		onModifyTypePriority: -1,
//! 		onModifyType(move, pokemon) {
//! 			const noModifyType = [
//! 				'judgment', 'multiattack', 'naturalgift', 'revelationdance', 'technoblast', 'terrainpulse', 'weatherball',
//! 			];
//! 			if (move.type === 'Normal' && (!noModifyType.includes(move.id) || this.activeMove?.isMax) &&
//! 				!(move.isZ && move.category !== 'Status') && !(move.name === 'Tera Blast' && pokemon.terastallized)) {
//! 				move.type = 'Electric';
//! 				move.typeChangerBoosted = this.effect;
//! 			}
//! 		},
//! 		onBasePowerPriority: 23,
//! 		onBasePower(basePower, pokemon, target, move) {
//! 			if (move.typeChangerBoosted === this.effect) return this.chainModify([4915, 4096]);
//! 		},
//! 		flags: {},
//! 		name: "Galvanize",
//! 		rating: 4,
//! 		num: 206,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_TYPE_PRIORITY: i32 = -1;

/// Moves that don't get type-changed
const NO_MODIFY_TYPE: &[&str] = &[
    "judgment", "multiattack", "naturalgift", "revelationdance", "technoblast", "terrainpulse", "weatherball",
];

/// onModifyType(move, pokemon)
/// Changes Normal-type moves to Electric-type
pub fn on_modify_type(move_: &mut MoveDef, pokemon: &Pokemon) -> AbilityHandlerResult {
    if move_.move_type == "Normal"
        && !NO_MODIFY_TYPE.contains(&move_.id.as_str())
        && !(move_.is_z && move_.category != crate::data::moves::MoveCategory::Status)
        && !(move_.name == "Tera Blast" && pokemon.terastallized.is_some())
    {
        move_.move_type = "Electric".to_string();
        move_.type_changer_boosted = true;
    }
    AbilityHandlerResult::Undefined
}

pub const ON_BASE_POWER_PRIORITY: i32 = 23;

/// onBasePower(basePower, pokemon, target, move)
pub fn on_base_power(_base_power: u32, _pokemon: &Pokemon, _target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    if move_.type_changer_boosted {
        return AbilityHandlerResult::ChainModify(4915, 4096); // ~1.2x
    }
    AbilityHandlerResult::Undefined
}

