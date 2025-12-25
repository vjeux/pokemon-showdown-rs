//! Sappy Seed Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	sappyseed: {
//! 		num: 738,
//! 		accuracy: 90,
//! 		basePower: 100,
//! 		category: "Physical",
//! 		isNonstandard: "LGPE",
//! 		name: "Sappy Seed",
//! 		pp: 10,
//! 		priority: 0,
//! 		flags: { protect: 1, reflectable: 1, mirror: 1 },
//! 		onHit(target, source) {
//! 			if (target.hasType('Grass')) return null;
//! 			target.addVolatile('leechseed', source);
//! 		},
//! 		secondary: null,
//! 		target: "normal",
//! 		type: "Grass",
//! 		contestType: "Clever",
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

