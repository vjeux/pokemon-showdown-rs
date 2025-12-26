//! Filter Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	filter: {
//! 		onSourceModifyDamage(damage, source, target, move) {
//! 			if (target.getMoveHitData(move).typeMod > 0) {
//! 				this.debug('Filter neutralize');
//! 				return this.chainModify(0.75);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Filter",
//! 		rating: 3,
//! 		num: 111,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceModifyDamage(damage, source, target, move)
    /// Reduces damage from super-effective moves by 25%
    pub fn on_source_modify_damage(_damage: u32, _source: &Pokemon, target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        // Check if move is super effective (typeMod > 0 means super effective)
        let hit_data = target.get_move_hit_data(&move_.id);
        if hit_data.type_mod > 0 {
            // 0.75x = 3072/4096
            return AbilityHandlerResult::ChainModify(3072, 4096);
        }
        AbilityHandlerResult::Undefined
    }
