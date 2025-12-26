//! Primordial Sea Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	primordialsea: {
//! 		onStart(source) {
//! 			this.field.setWeather('primordialsea');
//! 		},
//! 		onAnySetWeather(target, source, weather) {
//! 			const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
//! 			if (this.field.getWeather().id === 'primordialsea' && !strongWeathers.includes(weather.id)) return false;
//! 		},
//! 		onEnd(pokemon) {
//! 			if (this.field.weatherState.source !== pokemon) return;
//! 			for (const target of this.getAllActive()) {
//! 				if (target === pokemon) continue;
//! 				if (target.hasAbility('primordialsea')) {
//! 					this.field.weatherState.source = target;
//! 					return;
//! 				}
//! 			}
//! 			this.field.clearWeather();
//! 		},
//! 		flags: {},
//! 		name: "Primordial Sea",
//! 		rating: 4.5,
//! 		num: 189,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// Strong weathers that can override each other
const STRONG_WEATHERS: &[&str] = &["desolateland", "primordialsea", "deltastream"];

/// onStart(source)
/// Sets Primordial Sea weather
pub fn on_start(battle: &mut Battle, _source: &Pokemon) -> AbilityHandlerResult {
    // this.field.setWeather('primordialsea');
    battle.field.set_weather(ID::from("primordialsea"), None);
    AbilityHandlerResult::Undefined
}

/// onAnySetWeather(target, source, weather)
/// Prevents other weather from replacing strong weathers
pub fn on_any_set_weather(battle: &Battle, weather_id: &str) -> AbilityHandlerResult {
    // const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
    // if (this.field.getWeather().id === 'primordialsea' && !strongWeathers.includes(weather.id)) return false;
    if *battle.field.get_weather() == ID::from("primordialsea") && !STRONG_WEATHERS.contains(&weather_id) {
        return AbilityHandlerResult::False;
    }
    AbilityHandlerResult::Undefined
}

/// onEnd(pokemon)
/// Clears weather when ability holder faints (unless another has the ability)
pub fn on_end(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (this.field.weatherState.source !== pokemon) return;
    let weather_state = battle.field.get_weather_state();
    if let Some(ref source_slot) = weather_state.source_slot {
        if source_slot != &pokemon.get_slot() {
            return AbilityHandlerResult::Undefined;
        }
    } else {
        return AbilityHandlerResult::Undefined;
    }

    // for (const target of this.getAllActive())
    for (side_idx, slot, target) in battle.get_all_active(false) {
        // if (target === pokemon) continue;
        if (side_idx, slot) == (pokemon.side_index, pokemon.position) {
            continue;
        }

        // if (target.hasAbility('primordialsea'))
        if target.ability.as_str() == "primordialsea" {
            // this.field.weatherState.source = target;
            let target_slot = target.get_slot();
            battle.field.get_weather_state_mut().source_slot = Some(target_slot);
            // return;
            return AbilityHandlerResult::Undefined;
        }
    }

    // this.field.clearWeather();
    battle.field.clear_weather();
    AbilityHandlerResult::Undefined
}

