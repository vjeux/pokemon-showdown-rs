//! Full Incense Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	fullincense: {
//! 		name: "Full Incense",
//! 		spritenum: 155,
//! 		fling: {
//! 			basePower: 10,
//! 		},
//! 		onFractionalPriority: -0.1,
//! 		num: 316,
//! 		gen: 4,
//! 		isNonstandard: "Past",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{ItemHandlerResult, ItemDef};

/// onFractionalPriority(...)
pub fn on_fractional_priority(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}
