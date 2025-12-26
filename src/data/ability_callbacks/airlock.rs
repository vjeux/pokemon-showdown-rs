//! Air Lock Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	airlock: {
//! 		onSwitchIn(pokemon) {
//! 			// Air Lock does not activate when Skill Swapped or when Neutralizing Gas leaves the field
//! 			this.add('-ability', pokemon, 'Air Lock');
//! 			((this.effect as any).onStart as (p: Pokemon) => void).call(this, pokemon);
//! 		},
//! 		onStart(pokemon) {
//! 			pokemon.abilityState.ending = false; // Clear the ending flag
//! 			this.eachEvent('WeatherChange', this.effect);
//! 		},
//! 		onEnd(pokemon) {
//! 			pokemon.abilityState.ending = true;
//! 			this.eachEvent('WeatherChange', this.effect);
//! 		},
//! 		suppressWeather: true,
//! 		flags: {},
//! 		name: "Air Lock",
//! 		rating: 1.5,
//! 		num: 76,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const SUPPRESS_WEATHER: bool = true;

    /// onSwitchIn(pokemon)
    pub fn on_switch_in(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Air Lock")]);
        AbilityHandlerResult::Undefined
    }

    /// onStart(pokemon)
    pub fn on_start(_battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
        pokemon.ability_state.data.insert("ending".to_string(), serde_json::Value::Bool(false));
        // Weather suppression is handled by the SUPPRESS_WEATHER flag
        AbilityHandlerResult::Undefined
    }

    /// onEnd(pokemon)
    pub fn on_end(_battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
        pokemon.ability_state.data.insert("ending".to_string(), serde_json::Value::Bool(true));
        AbilityHandlerResult::Undefined
    }
