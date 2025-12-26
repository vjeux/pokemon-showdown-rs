//! Leftovers Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	leftovers: {
//! 		name: "Leftovers",
//! 		spritenum: 242,
//! 		fling: {
//! 			basePower: 10,
//! 		},
//! 		onResidualOrder: 5,
//! 		onResidualSubOrder: 4,
//! 		onResidual(pokemon) {
//! 			this.heal(pokemon.baseMaxhp / 16);
//! 		},
//! 		num: 234,
//! 		gen: 2,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{ItemHandlerResult, ItemDef};

/// onResidualOrder(...)
pub fn on_residual_order(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}

/// onResidualSubOrder(...)
pub fn on_residual_sub_order(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}

/// onResidual(...)
pub fn on_residual(battle: &mut Battle, /* TODO: Add parameters */) -> ItemHandlerResult {
    // TODO: Implement 1-to-1 from JS
    ItemHandlerResult::Undefined
}
