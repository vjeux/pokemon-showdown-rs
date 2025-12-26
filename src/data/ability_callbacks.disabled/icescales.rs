//! Ice Scales Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	icescales: {
//! 		onSourceModifyDamage(damage, source, target, move) {
//! 			if (move.category === 'Special') {
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Ice Scales",
//! 		rating: 4,
//! 		num: 246,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceModifyDamage(damage, source, target, move)
/// Halves damage from Special moves
pub fn on_source_modify_damage(_damage: u32, _source: &Pokemon, _target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.category === 'Special')
    if move_.category == MoveCategory::Special {
        // return this.chainModify(0.5);
        return AbilityHandlerResult::ChainModify(2048, 4096); // 0.5x
    }
    AbilityHandlerResult::Undefined
}

