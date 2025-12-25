//! Magician Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	magician: {
//! 		onAfterMoveSecondarySelf(source, target, move) {
//! 			if (!move || source.switchFlag === true || !move.hitTargets || source.item || source.volatiles['gem'] ||
//! 				move.id === 'fling' || move.category === 'Status') return;
//! 			const hitTargets = move.hitTargets;
//! 			this.speedSort(hitTargets);
//! 			for (const pokemon of hitTargets) {
//! 				if (pokemon !== source) {
//! 					const yourItem = pokemon.takeItem(source);
//! 					if (!yourItem) continue;
//! 					if (!source.setItem(yourItem)) {
//! 						pokemon.item = yourItem.id; // bypass setItem so we don't break choicelock or anything
//! 						continue;
//! 					}
//! 					this.add('-item', source, yourItem, '[from] ability: Magician', `[of] ${pokemon}`);
//! 					return;
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Magician",
//! 		rating: 1,
//! 		num: 170,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onAfterMoveSecondarySelf(source, target, move)
/// Steals target's item after attacking
///
/// TODO: onAfterMoveSecondarySelf handler not yet implemented
/// TODO: Needs item system (pokemon.item, takeItem(), setItem())
/// TODO: Needs source.switchFlag, move.hitTargets, volatiles['gem']
/// When implemented, should:
/// 1. Skip if no move, switching, no hitTargets, has item, gem used, fling, or status move
/// 2. Sort hitTargets by speed
/// 3. Loop through targets, skip source
/// 4. Call pokemon.takeItem(source) and source.setItem(yourItem)
/// 5. Add item message and return
pub fn on_after_move_secondary_self(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

