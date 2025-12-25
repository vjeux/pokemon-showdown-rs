//! Regenerator Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	regenerator: {
//! 		onSwitchOut(pokemon) {
//! 			pokemon.heal(pokemon.baseMaxhp / 3);
//! 		},
//! 		flags: {},
//! 		name: "Regenerator",
//! 		rating: 4.5,
//! 		num: 144,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSwitchOut(pokemon)
pub fn on_switch_out(_battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // pokemon.heal(pokemon.baseMaxhp / 3);
    let heal_amount = pokemon.base_maxhp / 3;
    pokemon.heal(heal_amount);
    AbilityHandlerResult::Undefined
}
