//! Hondew Berry Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	hondewberry: {
//! 		name: "Hondew Berry",
//! 		spritenum: 213,
//! 		isBerry: true,
//! 		naturalGift: {
//! 			basePower: 90,
//! 			type: "Ground",
//! 		},
//! 		onEat: false,
//! 		num: 172,
//! 		gen: 3,
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
