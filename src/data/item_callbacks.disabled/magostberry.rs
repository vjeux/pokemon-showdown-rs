//! Magost Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	magostberry: {
//! 		name: "Magost Berry",
//! 		spritenum: 275,
//! 		isBerry: true,
//! 		naturalGift: {
//! 			basePower: 90,
//! 			type: "Rock",
//! 		},
//! 		onEat: false,
//! 		num: 176,
//! 		gen: 3,
//! 		isNonstandard: "Past",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{ItemHandlerResult, ItemDef};

/// onEat(...)
pub fn on_eat(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}
