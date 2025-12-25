//! Schooling Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	schooling: {
//! 		onSwitchInPriority: -1,
//! 		onStart(pokemon) {
//! 			if (pokemon.baseSpecies.baseSpecies !== 'Wishiwashi' || pokemon.level < 20 || pokemon.transformed) return;
//! 			if (pokemon.hp > pokemon.maxhp / 4) {
//! 				if (pokemon.species.id === 'wishiwashi') {
//! 					pokemon.formeChange('Wishiwashi-School');
//! 				}
//! 			} else {
//! 				if (pokemon.species.id === 'wishiwashischool') {
//! 					pokemon.formeChange('Wishiwashi');
//! 				}
//! 			}
//! 		},
//! 		onResidualOrder: 29,
//! 		onResidual(pokemon) {
//! 			if (
//! 				pokemon.baseSpecies.baseSpecies !== 'Wishiwashi' || pokemon.level < 20 ||
//! 				pokemon.transformed || !pokemon.hp
//! 			) return;
//! 			if (pokemon.hp > pokemon.maxhp / 4) {
//! 				if (pokemon.species.id === 'wishiwashi') {
//! 					pokemon.formeChange('Wishiwashi-School');
//! 				}
//! 			} else {
//! 				if (pokemon.species.id === 'wishiwashischool') {
//! 					pokemon.formeChange('Wishiwashi');
//! 				}
//! 			}
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, cantsuppress: 1 },
//! 		name: "Schooling",
//! 		rating: 3,
//! 		num: 208,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_SWITCH_IN_PRIORITY: i32 = -1;

/// onStart(pokemon)
/// Wishiwashi forme changes based on HP and level
///
/// TODO: onStart handler not yet implemented
/// TODO: Needs pokemon.baseSpecies, pokemon.level, pokemon.transformed, pokemon.hp, pokemon.maxhp, pokemon.species.id, pokemon.formeChange()
/// When implemented, should:
/// 1. Check if Wishiwashi, level >= 20, and not transformed
/// 2. If HP > 25%, change to School forme
/// 3. If HP <= 25%, change to Solo forme
pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

pub const ON_RESIDUAL_ORDER: i32 = 29;

/// onResidual(pokemon)
/// Checks HP and changes forme each turn
///
/// TODO: onResidual handler not yet implemented
/// TODO: Needs pokemon.baseSpecies, pokemon.level, pokemon.transformed, pokemon.hp, pokemon.maxhp, pokemon.species.id, pokemon.formeChange()
/// When implemented, should:
/// 1. Check if Wishiwashi, level >= 20, not transformed, and alive
/// 2. If HP > 25%, change to School forme
/// 3. If HP <= 25%, change to Solo forme
pub fn on_residual(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

