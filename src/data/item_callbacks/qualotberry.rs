//! Qualot Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	qualotberry: {
//! 		name: "Qualot Berry",
//! 		spritenum: 371,
//! 		isBerry: true,
//! 		naturalGift: {
//! 			basePower: 90,
//! 			type: "Poison",
//! 		},
//! 		onEat: false,
//! 		num: 171,
//! 		gen: 3,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{ItemHandlerResult, ItemDef};

/// onEat(...)
pub fn on_eat(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}
