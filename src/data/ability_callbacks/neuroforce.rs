//! Neuroforce Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	neuroforce: {
//! 		onModifyDamage(damage, source, target, move) {
//! 			if (move && target.getMoveHitData(move).typeMod > 0) {
//! 				return this.chainModify([5120, 4096]);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Neuroforce",
//! 		rating: 2.5,
//! 		num: 233,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyDamage(damage, source, target, move)
/// Boosts super-effective damage by 1.25x
pub fn on_modify_damage(_damage: u32, _source: &Pokemon, target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move && target.getMoveHitData(move).typeMod > 0)
    let hit_data = target.get_move_hit_data(&move_.id);
    if hit_data.type_mod > 0 {
        // return this.chainModify([5120, 4096]);
        return AbilityHandlerResult::ChainModify(5120, 4096); // 1.25x
    }
    AbilityHandlerResult::Undefined
}

