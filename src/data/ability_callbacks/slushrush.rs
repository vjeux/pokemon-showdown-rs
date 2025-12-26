//! Slush Rush Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	slushrush: {
//! 		onModifySpe(spe, pokemon) {
//! 			if (this.field.isWeather(['hail', 'snowscape'])) {
//! 				return this.chainModify(2);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Slush Rush",
//! 		rating: 3,
//! 		num: 202,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifySpe(spe, pokemon)
pub fn on_modify_spe(battle: &Battle, _spe: u32, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (this.field.isWeather(['hail', 'snowscape']))
    let weather = battle.field.get_weather();
    if *weather == ID::new("hail") || *weather == ID::new("snowscape") {
        // return this.chainModify(2);
        return AbilityHandlerResult::ChainModify(2, 1);
    }
    AbilityHandlerResult::Undefined
}
