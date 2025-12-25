//! Splintered Stormshards Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	splinteredstormshards: {
//! 		num: 727,
//! 		accuracy: true,
//! 		basePower: 190,
//! 		category: "Physical",
//! 		isNonstandard: "Past",
//! 		name: "Splintered Stormshards",
//! 		pp: 1,
//! 		priority: 0,
//! 		flags: {},
//! 		onHit() {
//! 			this.field.clearTerrain();
//! 		},
//! 		onAfterSubDamage() {
//! 			this.field.clearTerrain();
//! 		},
//! 		isZ: "lycaniumz",
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Rock",
//! 		contestType: "Cool",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit(...)
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

/// onAfterSubDamage(...)
pub fn on_after_sub_damage(battle: &mut Battle, /* TODO: Add parameters */) -> MoveHandlerResult {
    // TODO: Implement 1-to-1 from JS
    MoveHandlerResult::Undefined
}

