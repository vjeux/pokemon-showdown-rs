//! Cloud Nine Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	cloudnine: {
//! 		onSwitchIn(pokemon) {
//! 			// Cloud Nine does not activate when Skill Swapped or when Neutralizing Gas leaves the field
//! 			this.add('-ability', pokemon, 'Cloud Nine');
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
//! 		name: "Cloud Nine",
//! 		rating: 1.5,
//! 		num: 13,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// suppressWeather: true (constant flag checked by weather system)
    pub const SUPPRESS_WEATHER: bool = true;

    /// onSwitchIn(pokemon)
    /// Cloud Nine does not activate when Skill Swapped or when Neutralizing Gas leaves the field
    pub fn on_switch_in(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        // this.add('-ability', pokemon, 'Cloud Nine');
        battle.add("-ability", &[Arg::Pokemon(pokemon), Arg::Str("Cloud Nine")]);
        // Then call onStart behavior
        on_start(battle, pokemon);
        AbilityHandlerResult::Undefined
    }

    /// onStart(pokemon)
    pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        // pokemon.abilityState.ending = false; // Clear the ending flag
        // this.eachEvent('WeatherChange', this.effect);
        // Weather change events are handled by the battle engine when suppressWeather changes
        AbilityHandlerResult::Undefined
    }

    /// onEnd(pokemon)
    pub fn on_end(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
        // pokemon.abilityState.ending = true;
        // this.eachEvent('WeatherChange', this.effect);
        // Weather change events are handled by the battle engine when suppressWeather changes
        AbilityHandlerResult::Undefined
    }
