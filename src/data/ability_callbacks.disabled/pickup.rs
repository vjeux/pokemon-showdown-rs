//! Pickup Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	pickup: {
//! 		onResidualOrder: 28,
//! 		onResidualSubOrder: 2,
//! 		onResidual(pokemon) {
//! 			if (pokemon.item) return;
//! 			const pickupTargets = this.getAllActive().filter(target => (
//! 				target.lastItem && target.usedItemThisTurn && pokemon.isAdjacent(target)
//! 			));
//! 			if (!pickupTargets.length) return;
//! 			const randomTarget = this.sample(pickupTargets);
//! 			const item = randomTarget.lastItem;
//! 			randomTarget.lastItem = '';
//! 			this.add('-item', pokemon, this.dex.items.get(item), '[from] ability: Pickup');
//! 			pokemon.setItem(item);
//! 		},
//! 		flags: {},
//! 		name: "Pickup",
//! 		rating: 0.5,
//! 		num: 53,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_RESIDUAL_ORDER: i32 = 28;
pub const ON_RESIDUAL_SUB_ORDER: i32 = 2;

/// onResidual(pokemon)
/// Picks up items used by adjacent Pokemon this turn
///
/// TODO: onResidual handler not yet implemented
/// TODO: Needs pokemon.item, getAllActive(), target.lastItem, target.usedItemThisTurn, pokemon.isAdjacent(), sample(), setItem()
/// When implemented, should:
/// 1. Skip if pokemon already has an item
/// 2. Filter all active Pokemon for those with lastItem, usedItemThisTurn, and adjacent to this Pokemon
/// 3. If no targets, return
/// 4. Randomly sample one target
/// 5. Take the target's lastItem and clear it
/// 6. Add item message and set the item on this Pokemon
pub fn on_residual(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

