//! Chlorophyll Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	chlorophyll: {
//! 		onModifySpe(spe, pokemon) {
//! 			if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather())) {
//! 				return this.chainModify(2);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Chlorophyll",
//! 		rating: 3,
//! 		num: 34,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifySpe(spe, pokemon)
    pub fn on_modify_spe(battle: &Battle, _spe: u32, _pokemon: &Pokemon) -> AbilityHandlerResult {
        // if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather()))
        let weather = battle.field.effective_weather();
        if *weather == ID::new("sunnyday") || *weather == ID::new("desolateland") {
            // return this.chainModify(2);
            return AbilityHandlerResult::ChainModify(8192, 4096); // 2x
        }
        AbilityHandlerResult::Undefined
    }
