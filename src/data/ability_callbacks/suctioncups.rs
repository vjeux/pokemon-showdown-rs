//! Suction Cups Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	suctioncups: {
//! 		onDragOutPriority: 1,
//! 		onDragOut(pokemon) {
//! 			this.add('-activate', pokemon, 'ability: Suction Cups');
//! 			return null;
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Suction Cups",
//! 		rating: 1,
//! 		num: 21,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDragOutPriority(...)
pub fn on_drag_out_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onDragOut(...)
pub fn on_drag_out(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

