//! Solid Rock Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	solidrock: {
//! 		onSourceModifyDamage(damage, source, target, move) {
//! 			if (target.getMoveHitData(move).typeMod > 0) {
//! 				this.debug('Solid Rock neutralize');
//! 				return this.chainModify(0.75);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Solid Rock",
//! 		rating: 3,
//! 		num: 116,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceModifyDamage(damage, source, target, move)
/// Reduces super-effective damage by 25%
pub fn on_source_modify_damage(_damage: i32, _source: &Pokemon, target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (target.getMoveHitData(move).typeMod > 0)
    let hit_data = target.get_move_hit_data(&move_.id);
    if hit_data.type_mod > 0 {
        // return this.chainModify(0.75);
        return AbilityHandlerResult::ChainModify(3072, 4096); // 0.75x
    }
    AbilityHandlerResult::Undefined
}

