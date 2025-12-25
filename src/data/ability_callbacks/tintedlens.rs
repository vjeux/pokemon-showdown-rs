//! Tinted Lens Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	tintedlens: {
//! 		onModifyDamage(damage, source, target, move) {
//! 			if (target.getMoveHitData(move).typeMod < 0) {
//! 				this.debug('Tinted Lens boost');
//! 				return this.chainModify(2);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Tinted Lens",
//! 		rating: 4,
//! 		num: 110,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyDamage(damage, source, target, move)
/// Doubles damage of not very effective moves
pub fn on_modify_damage(_damage: u32, _source: &Pokemon, target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (target.getMoveHitData(move).typeMod < 0)
    let hit_data = target.get_move_hit_data(&move_.id);
    if hit_data.type_mod < 0 {
        // this.debug('Tinted Lens boost');
        // return this.chainModify(2);
        return AbilityHandlerResult::ChainModify(8192, 4096); // 2x
    }
    AbilityHandlerResult::Undefined
}

