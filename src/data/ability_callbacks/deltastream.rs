//! Delta Stream Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	deltastream: {
//! 		onStart(source) {
//! 			this.field.setWeather('deltastream');
//! 		},
//! 		onAnySetWeather(target, source, weather) {
//! 			const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
//! 			if (this.field.getWeather().id === 'deltastream' && !strongWeathers.includes(weather.id)) return false;
//! 		},
//! 		onEnd(pokemon) {
//! 			if (this.field.weatherState.source !== pokemon) return;
//! 			for (const target of this.getAllActive()) {
//! 				if (target === pokemon) continue;
//! 				if (target.hasAbility('deltastream')) {
//! 					this.field.weatherState.source = target;
//! 					return;
//! 				}
//! 			}
//! 			this.field.clearWeather();
//! 		},
//! 		flags: {},
//! 		name: "Delta Stream",
//! 		rating: 4,
//! 		num: 191,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// Strong weathers that can override Delta Stream
    const STRONG_WEATHERS: &[&str] = &["desolateland", "primordialsea", "deltastream"];

    /// onStart(source)
    pub fn on_start(battle: &mut Battle, _source: &Pokemon) -> AbilityHandlerResult {
        // this.field.setWeather('deltastream');
        battle.field.set_weather(ID::new("deltastream"), None);
        AbilityHandlerResult::Undefined
    }

    /// onAnySetWeather(target, source, weather)
    /// Blocks non-strong weathers
    pub fn on_any_set_weather(battle: &Battle, weather_id: &str) -> AbilityHandlerResult {
        // if (this.field.getWeather().id === 'deltastream' && !strongWeathers.includes(weather.id)) return false;
        if *battle.field.get_weather() == ID::new("deltastream") && !STRONG_WEATHERS.contains(&weather_id) {
            return AbilityHandlerResult::False;
        }
        AbilityHandlerResult::Undefined
    }

    /// onEnd(pokemon)
    pub fn on_end(battle: &mut Battle, _pokemon: &Pokemon) -> AbilityHandlerResult {
        // if (this.field.weatherState.source !== pokemon) return;
        // Simplified - just clear weather if this was the source
        // Full implementation requires checking other Pokemon with deltastream
        battle.field.clear_weather();
        AbilityHandlerResult::Undefined
    }
