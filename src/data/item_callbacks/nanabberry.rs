//! Nanab Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	nanabberry: {
//! 		name: "Nanab Berry",
//! 		spritenum: 302,
//! 		isBerry: true,
//! 		naturalGift: {
//! 			basePower: 90,
//! 			type: "Water",
//! 		},
//! 		onEat: false,
//! 		num: 166,
//! 		gen: 3,
//! 		isNonstandard: "Past",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::ItemHandlerResult;

/// onEat(...)
pub fn on_eat(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}
