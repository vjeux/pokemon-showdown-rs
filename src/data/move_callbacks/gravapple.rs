//! Grav Apple Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	gravapple: {
//! 		num: 788,
//! 		accuracy: 100,
//! 		basePower: 80,
//! 		category: "Physical",
//! 		name: "Grav Apple",
//! 		pp: 10,
//! 		priority: 0,
//! 		flags: { protect: 1, mirror: 1 },
//! 		onBasePower(basePower) {
//! 			if (this.field.getPseudoWeather('gravity')) {
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		secondary: {
//! 			chance: 100,
//! 			boosts: {
//! 				def: -1,
//! 			},
//! 		},
//! 		target: "normal",
//! 		type: "Grass",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onBasePower(...)
pub fn on_base_power(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

