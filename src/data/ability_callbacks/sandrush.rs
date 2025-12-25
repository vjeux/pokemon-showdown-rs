//! Sand Rush Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	sandrush: {
//! 		onModifySpe(spe, pokemon) {
//! 			if (this.field.isWeather('sandstorm')) {
//! 				return this.chainModify(2);
//! 			}
//! 		},
//! 		onImmunity(type, pokemon) {
//! 			if (type === 'sandstorm') return false;
//! 		},
//! 		flags: {},
//! 		name: "Sand Rush",
//! 		rating: 3,
//! 		num: 146,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifySpe(spe, pokemon)
pub fn on_modify_spe(battle: &Battle, _spe: u32, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (this.field.isWeather('sandstorm'))
    if *battle.field.get_weather() == ID::new("sandstorm") {
        // return this.chainModify(2);
        return AbilityHandlerResult::ChainModify(2, 1);
    }
    AbilityHandlerResult::Undefined
}

/// onImmunity(type, pokemon)
pub fn on_immunity(_battle: &Battle, immunity_type: &str, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (type === 'sandstorm') return false;
    if immunity_type == "sandstorm" {
        return AbilityHandlerResult::False;
    }
    AbilityHandlerResult::Undefined
}
