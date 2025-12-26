//! Shed Skin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	shedskin: {
//! 		onResidualOrder: 5,
//! 		onResidualSubOrder: 3,
//! 		onResidual(pokemon) {
//! 			if (pokemon.hp && pokemon.status && this.randomChance(33, 100)) {
//! 				this.debug('shed skin');
//! 				this.add('-activate', pokemon, 'ability: Shed Skin');
//! 				pokemon.cureStatus();
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Shed Skin",
//! 		rating: 3,
//! 		num: 61,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onResidualOrder: 5
pub const ON_RESIDUAL_ORDER: i32 = 5;
/// onResidualSubOrder: 3
pub const ON_RESIDUAL_SUB_ORDER: i32 = 3;

/// onResidual(pokemon)
pub fn on_residual(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // if (pokemon.hp && pokemon.status && this.randomChance(33, 100))
    if pokemon.hp > 0 && !pokemon.status.is_empty() && battle.random_chance(33, 100) {
        // this.debug('shed skin');
        // this.add('-activate', pokemon, 'ability: Shed Skin');
        battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Shed Skin")]);
        // pokemon.cureStatus();
        pokemon.cure_status();
    }
    AbilityHandlerResult::Undefined
}

