//! Steam Engine Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	steamengine: {
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (['Water', 'Fire'].includes(move.type)) {
//! 				this.boost({ spe: 6 });
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Steam Engine",
//! 		rating: 2,
//! 		num: 243,
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
    // if (['Water', 'Fire'].includes(move.type))
    if move_.move_type == "Water" || move_.move_type == "Fire" {
        // this.boost({ spe: 6 });
        battle.boost(&[("spe", 6)], target_ref, Some(target_ref), None);
    }
    AbilityHandlerResult::Undefined
}
