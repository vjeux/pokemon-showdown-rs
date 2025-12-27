//! Pixilate Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	pixilate: {
//! 		onModifyTypePriority: -1,
//! 		onModifyType(move, pokemon) {
//! 			const noModifyType = [
//! 				'judgment', 'multiattack', 'naturalgift', 'revelationdance', 'technoblast', 'terrainpulse', 'weatherball',
//! 			];
//! 			if (move.type === 'Normal' && (!noModifyType.includes(move.id) || this.activeMove?.isMax) &&
//! 				!(move.isZ && move.category !== 'Status') && !(move.name === 'Tera Blast' && pokemon.terastallized)) {
//! 				move.type = 'Fairy';
//! 				move.typeChangerBoosted = this.effect;
//! 			}
//! 		},
//! 		onBasePowerPriority: 23,
//! 		onBasePower(basePower, pokemon, target, move) {
//! 			if (move.typeChangerBoosted === this.effect) return this.chainModify([4915, 4096]);
//! 		},
//! 		flags: {},
//! 		name: "Pixilate",
//! 		rating: 4,
//! 		num: 182,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_TYPE_PRIORITY: i32 = -1;

const NO_MODIFY_TYPE: &[&str] = &[
    "judgment", "multiattack", "naturalgift", "revelationdance",
    "technoblast", "terrainpulse", "weatherball",
];

/// onModifyType(move, pokemon)
/// Converts Normal-type moves to Fairy-type
pub fn on_modify_type(move_: &mut MoveDef, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (move.type === 'Normal' && (!noModifyType.includes(move.id) || this.activeMove?.isMax) &&
    //     !(move.isZ && move.category !== 'Status') && !(move.name === 'Tera Blast' && pokemon.terastallized))
    if move_.move_type == "Normal"
        && !NO_MODIFY_TYPE.contains(&move_.id.as_str())
        && !(move_.is_z && move_.category != MoveCategory::Status)
        && !(move_.name == "Tera Blast" && pokemon.terastallized.is_some())
    {
        // move.type = 'Fairy';
        move_.move_type = "Fairy".to_string();
        // move.typeChangerBoosted = this.effect;
        move_.type_changer_boosted = true;
    }
    AbilityHandlerResult::Undefined
}

pub const ON_BASE_POWER_PRIORITY: i32 = 23;

/// onBasePower(basePower, pokemon, target, move)
/// Boosts type-changed moves by 1.2x
pub fn on_base_power(_base_power: i32, _pokemon: &Pokemon, _target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.typeChangerBoosted === this.effect)
    if move_.type_changer_boosted {
        // return this.chainModify([4915, 4096]);
        return AbilityHandlerResult::ChainModify(4915, 4096); // ~1.2x
    }
    AbilityHandlerResult::Undefined
}

