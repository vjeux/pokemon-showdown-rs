//! Weak Armor Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	weakarmor: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (move.category === 'Physical') {
//! 				this.boost({ def: -1, spe: 2 }, target, target);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Weak Armor",
//! 		rating: 1,
//! 		num: 133,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target: &Pokemon, _source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    let target_ref = (target.side_index, target.position);
    // if (move.category === 'Physical')
    if move_.category == MoveCategory::Physical {
        // this.boost({ def: -1, spe: 2 }, target, target);
        battle.boost(&[("def", -1), ("spe", 2)], target_ref, Some(target_ref), None);
    }
    AbilityHandlerResult::Undefined
}
