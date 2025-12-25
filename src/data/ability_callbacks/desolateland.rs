//! Desolate Land Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	desolateland: {
//! 		onStart(source) {
//! 			this.field.setWeather('desolateland');
//! 		},
//! 		onAnySetWeather(target, source, weather) {
//! 			const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
//! 			if (this.field.getWeather().id === 'desolateland' && !strongWeathers.includes(weather.id)) return false;
//! 		},
//! 		onEnd(pokemon) {
//! 			if (this.field.weatherState.source !== pokemon) return;
//! 			for (const target of this.getAllActive()) {
//! 				if (target === pokemon) continue;
//! 				if (target.hasAbility('desolateland')) {
//! 					this.field.weatherState.source = target;
//! 					return;
//! 				}
//! 			}
//! 			this.field.clearWeather();
//! 		},
//! 		flags: {},
//! 		name: "Desolate Land",
//! 		rating: 4.5,
//! 		num: 190,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// Strong weathers that can override Desolate Land
    const STRONG_WEATHERS: &[&str] = &["desolateland", "primordialsea", "deltastream"];

    /// onStart(source)
    pub fn on_start(battle: &mut Battle, _source: &Pokemon) -> AbilityHandlerResult {
        // this.field.setWeather('desolateland');
        battle.field.set_weather(ID::new("desolateland"), None);
        AbilityHandlerResult::Undefined
    }

    /// onAnySetWeather(target, source, weather)
    /// Blocks non-strong weathers
    pub fn on_any_set_weather(battle: &Battle, weather_id: &str) -> AbilityHandlerResult {
        // if (this.field.getWeather().id === 'desolateland' && !strongWeathers.includes(weather.id)) return false;
        if *battle.field.get_weather() == ID::new("desolateland") && !STRONG_WEATHERS.contains(&weather_id) {
            return AbilityHandlerResult::False;
        }
        AbilityHandlerResult::Undefined
    }

    /// onEnd(pokemon)
    pub fn on_end(battle: &mut Battle, _pokemon: &Pokemon) -> AbilityHandlerResult {
        // Simplified - just clear weather
        battle.field.clear_weather();
        AbilityHandlerResult::Undefined
    }
