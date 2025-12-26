//! Stamina Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	stamina: {
//! 		onDamagingHit(damage, target, source, effect) {
//! 			this.boost({ def: 1 });
//! 		},
//! 		flags: {},
//! 		name: "Stamina",
//! 		rating: 4,
//! 		num: 192,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamagingHit(damage, target, source, effect)
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, _source: &Pokemon, _effect: &Effect) -> AbilityHandlerResult {
    let target_ref = (target.side_index, target.position);
    // this.boost({ def: 1 });
    battle.boost(&[("def", 1)], target_ref, Some(target_ref), None);
    AbilityHandlerResult::Undefined
}
