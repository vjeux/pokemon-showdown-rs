//! Delta Stream Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(source) {
///     this.field.setWeather('deltastream');
/// }
pub fn on_start(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // this.field.setWeather('deltastream');
    battle.set_weather(ID::from("deltastream"), None, None);

    EventResult::Continue
}

/// onAnySetWeather(target, source, weather) {
///     const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
///     if (this.field.getWeather().id === 'deltastream' && !strongWeathers.includes(weather.id)) return false;
/// }
pub fn on_any_set_weather(battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, weather_id: &str) -> EventResult {
    // const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
    let strong_weathers = ["desolateland", "primordialsea", "deltastream"];

    // Get current weather
    let current_weather = battle.field.get_weather().as_str();

    // if (this.field.getWeather().id === 'deltastream' && !strongWeathers.includes(weather.id)) return false;
    if current_weather == "deltastream" && !strong_weathers.contains(&weather_id) {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onEnd(pokemon) {
///     if (this.field.weatherState.source !== pokemon) return;
///     for (const target of this.getAllActive()) {
///         if (target === pokemon) continue;
///         if (target.hasAbility('deltastream')) {
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

        // if (target.hasAbility('deltastream'))
        let has_delta_stream = {
            let pokemon = match battle.pokemon_at(active_pos.0, active_pos.1) {
                Some(p) => p,
                None => continue,
            };
            pokemon.base_ability == ID::from("deltastream") || pokemon.ability == ID::from("deltastream")
        };

        if has_delta_stream {
            // this.field.weatherState.source = target;
            battle.field.weather_state.source = Some(active_pos);
            return EventResult::Continue;
        }
    }

    // this.field.clearWeather();
    battle.clear_weather();

    EventResult::Continue
}

