//! Water Compaction Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	watercompaction: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (move.type === 'Water') {
//! 				this.boost({ def: 2 });
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Water Compaction",
//! 		rating: 1.5,
//! 		num: 195,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, _source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    let target_ref = (target.side_index, target.position);
    // if (move.type === 'Water')
    if move_.move_type == "Water" {
        // this.boost({ def: 2 });
        battle.boost(&[("def", 2)], target_ref, Some(target_ref), None);
    }
    AbilityHandlerResult::Undefined
}
