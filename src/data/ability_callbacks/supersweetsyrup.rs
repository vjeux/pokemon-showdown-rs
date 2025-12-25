//! Supersweet Syrup Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	supersweetsyrup: {
//! 		onStart(pokemon) {
//! 			if (pokemon.syrupTriggered) return;
//! 			pokemon.syrupTriggered = true;
//! 			this.add('-ability', pokemon, 'Supersweet Syrup');
//! 			for (const target of pokemon.adjacentFoes()) {
//! 				if (target.volatiles['substitute']) {
//! 					this.add('-immune', target);
//! 				} else {
//! 					this.boost({ evasion: -1 }, target, pokemon, null, true);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Supersweet Syrup",
//! 		rating: 1.5,
//! 		num: 306,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Lowers Evasion of adjacent foes by 1 on switch-in (once per switch)
///
/// TODO: onStart handler not yet implemented
/// TODO: Needs pokemon.syrupTriggered state tracking, pokemon.adjacentFoes(), target.volatiles['substitute'], battle.boost()
/// When implemented, should:
/// 1. Check if pokemon.syrupTriggered is true (already triggered this switch-in)
/// 2. Set pokemon.syrupTriggered = true
/// 3. Add ability message: battle.add('-ability', pokemon, 'Supersweet Syrup')
/// 4. Loop through pokemon.adjacentFoes()
/// 5. For each target: if has substitute, show immune, else boost evasion -1
pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

