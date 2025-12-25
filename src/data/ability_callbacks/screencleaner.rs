//! Screen Cleaner Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	screencleaner: {
//! 		onStart(pokemon) {
//! 			let activated = false;
//! 			for (const sideCondition of ['reflect', 'lightscreen', 'auroraveil']) {
//! 				for (const side of [pokemon.side, ...pokemon.side.foeSidesWithConditions()]) {
//! 					if (side.getSideCondition(sideCondition)) {
//! 						if (!activated) {
//! 							this.add('-activate', pokemon, 'ability: Screen Cleaner');
//! 							activated = true;
//! 						}
//! 						side.removeSideCondition(sideCondition);
//! 					}
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Screen Cleaner",
//! 		rating: 2,
//! 		num: 251,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Removes screens (Reflect, Light Screen, Aurora Veil) from both sides
///
/// TODO: onStart handler not yet implemented
/// TODO: Needs pokemon.side, pokemon.side.foeSidesWithConditions(), side.getSideCondition(), side.removeSideCondition(), battle.add()
/// When implemented, should:
/// 1. Loop through ['reflect', 'lightscreen', 'auroraveil'] side conditions
/// 2. Check both own side and foe sides
/// 3. Remove any active screen conditions
/// 4. Show activate message once if any screens removed
pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

