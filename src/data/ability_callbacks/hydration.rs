//! Hydration Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	hydration: {
//! 		onResidualOrder: 5,
//! 		onResidualSubOrder: 3,
//! 		onResidual(pokemon) {
//! 			if (pokemon.status && ['raindance', 'primordialsea'].includes(pokemon.effectiveWeather())) {
//! 				this.debug('hydration');
//! 				this.add('-activate', pokemon, 'ability: Hydration');
//! 				pokemon.cureStatus();
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Hydration",
//! 		rating: 1.5,
//! 		num: 93,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_RESIDUAL_ORDER: i32 = 5;
pub const ON_RESIDUAL_SUB_ORDER: i32 = 3;

/// onResidual(pokemon)
/// Cures status conditions in rain (raindance or primordialsea weather)
pub fn on_residual(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // if (pokemon.status && ['raindance', 'primordialsea'].includes(pokemon.effectiveWeather()))
    if !pokemon.status.is_empty() {
        let weather = battle.field.effective_weather();
        if *weather == ID::new("raindance") || *weather == ID::new("primordialsea") {
            // this.debug('hydration');
            // this.add('-activate', pokemon, 'ability: Hydration');
            battle.add("-activate", &[
                Arg::Pokemon(pokemon),
                Arg::Str("ability: Hydration")
            ]);
            // pokemon.cureStatus();
            pokemon.cure_status();
        }
    }
    AbilityHandlerResult::Undefined
}

