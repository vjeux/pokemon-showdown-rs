//! Soul-Heart Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	soulheart: {
//! 		onAnyFaintPriority: 1,
//! 		onAnyFaint() {
//! 			this.boost({ spa: 1 }, this.effectState.target);
//! 		},
//! 		flags: {},
//! 		name: "Soul-Heart",
//! 		rating: 3.5,
//! 		num: 220,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_ANY_FAINT_PRIORITY: i32 = 1;

/// onAnyFaint()
pub fn on_any_faint(battle: &mut Battle, target: &Pokemon) -> AbilityHandlerResult {
    let target_ref = (target.side_index, target.position);
    // this.boost({ spa: 1 }, this.effectState.target);
    battle.boost(&[("spa", 1)], target_ref, Some(target_ref), None);
    AbilityHandlerResult::Undefined
}
