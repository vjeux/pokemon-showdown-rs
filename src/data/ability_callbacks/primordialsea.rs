//! Primordial Sea Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(source) {
///     this.field.setWeather('primordialsea');
/// }
pub fn on_start(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // this.field.setWeather('primordialsea');
    battle.set_weather(ID::from("primordialsea"), None, None);

    EventResult::Continue
}

/// onAnySetWeather(target, source, weather) {
///     const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
///     if (this.field.getWeather().id === 'primordialsea' && !strongWeathers.includes(weather.id)) return false;
/// }
pub fn on_any_set_weather(battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>) -> EventResult {
    // const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
    let strong_weathers = ["desolateland", "primordialsea", "deltastream"];

    // Get current weather
    let current_weather = battle.field.get_weather().as_str();

    // Get the incoming weather from the event (relay_var_type contains the weather ID)
    let incoming_weather = if let Some(ref event) = battle.current_event {
        if let Some(ref weather_id) = match &event.relay_var { Some(crate::event::EventResult::String(ref s)) => Some(s), _ => None } {
            weather_id.as_str()
        } else {
            return EventResult::Continue;
        }
    } else {
        return EventResult::Continue;
    };

    // if (this.field.getWeather().id === 'primordialsea' && !strongWeathers.includes(weather.id)) return false;
    if current_weather == "primordialsea" && !strong_weathers.contains(&incoming_weather) {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onEnd(pokemon) {
///     if (this.field.weatherState.source !== pokemon) return;
///     for (const target of this.getAllActive()) {
///         if (target === pokemon) continue;
///         if (target.hasAbility('primordialsea')) {
///             this.field.weatherState.source = target;
///             return;
///         }
///     }
///     this.field.clearWeather();
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // if (this.field.weatherState.source !== pokemon) return;
    let weather_source = &battle.field.weather_state.source;
    if weather_source != &Some(pokemon_pos) {
        return EventResult::Continue;
    }

    // for (const target of this.getAllActive())
    let all_active = battle.get_all_active(false);

    for active_pos in all_active {
        // if (target === pokemon) continue;
        if active_pos == pokemon_pos {
            continue;
        }

        // if (target.hasAbility('primordialsea'))
        let has_primordial_sea = {
            let pokemon = match battle.pokemon_at(active_pos.0, active_pos.1) {
                Some(p) => p,
                None => continue,
            };
            pokemon.base_ability == ID::from("primordialsea") || pokemon.ability == ID::from("primordialsea")
        };

        if has_primordial_sea {
            // this.field.weatherState.source = target;
            battle.field.weather_state.source = Some(active_pos);
            return EventResult::Continue;
        }
    }

    // this.field.clearWeather();
    battle.clear_weather();

    EventResult::Continue
}

