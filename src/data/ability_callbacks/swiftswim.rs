//! Swift Swim Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	swiftswim: {
//! 		onModifySpe(spe, pokemon) {
//! 			if (['raindance', 'primordialsea'].includes(pokemon.effectiveWeather())) {
//! 				return this.chainModify(2);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Swift Swim",
//! 		rating: 3,
//! 		num: 33,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifySpe(spe, pokemon)
pub fn on_modify_spe(battle: &Battle, _spe: u32, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (['raindance', 'primordialsea'].includes(pokemon.effectiveWeather()))
    let eff_weather = pokemon.effective_weather(&battle.field.get_weather().to_string());
    if eff_weather == "raindance" || eff_weather == "primordialsea" {
        // return this.chainModify(2);
        return AbilityHandlerResult::ChainModify(2, 1);
    }
    AbilityHandlerResult::Undefined
}
