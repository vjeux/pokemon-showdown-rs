//! Justified Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	justified: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (move.type === 'Dark') {
//! 				this.boost({ atk: 1 });
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Justified",
//! 		rating: 2.5,
//! 		num: 154,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, move)
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, _source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    let target_ref = (target.side_index, target.position);
    // if (move.type === 'Dark')
    if move_.move_type == "Dark" {
        // this.boost({ atk: 1 });
        battle.boost(&[("atk", 1)], target_ref, Some(target_ref), None);
    }
    AbilityHandlerResult::Undefined
}
