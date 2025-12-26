//! Prism Armor Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	prismarmor: {
//! 		onSourceModifyDamage(damage, source, target, move) {
//! 			if (target.getMoveHitData(move).typeMod > 0) {
//! 				this.debug('Prism Armor neutralize');
//! 				return this.chainModify(0.75);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Prism Armor",
//! 		rating: 3,
//! 		num: 232,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceModifyDamage(damage, source, target, move)
/// Reduces super-effective damage by 25%
pub fn on_source_modify_damage(_damage: u32, _source: &Pokemon, target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (target.getMoveHitData(move).typeMod > 0)
    let hit_data = target.get_move_hit_data(&move_.id);
    if hit_data.type_mod > 0 {
        // return this.chainModify(0.75);
        return AbilityHandlerResult::ChainModify(3072, 4096); // 0.75x
    }
    AbilityHandlerResult::Undefined
}

