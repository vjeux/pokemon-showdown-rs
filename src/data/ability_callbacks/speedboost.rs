//! Speed Boost Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	speedboost: {
//! 		onResidualOrder: 28,
//! 		onResidualSubOrder: 2,
//! 		onResidual(pokemon) {
//! 			if (pokemon.activeTurns) {
//! 				this.boost({ spe: 1 });
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Speed Boost",
//! 		rating: 4.5,
//! 		num: 3,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onResidualOrder: 28
pub const ON_RESIDUAL_ORDER: i32 = 28;
/// onResidualSubOrder: 2
pub const ON_RESIDUAL_SUB_ORDER: i32 = 2;

/// onResidual(pokemon)
pub fn on_residual(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (pokemon.activeTurns)
    if pokemon.active_turns > 0 {
        // this.boost({ spe: 1 });
        let pokemon_ref = (pokemon.side_index, pokemon.position);
        battle.boost(&[("spe", 1)], pokemon_ref, Some(pokemon_ref), None);
    }
    AbilityHandlerResult::Undefined
}
