//! Desolate Land Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(source) {
///     this.field.setWeather('desolateland');
/// }
pub fn on_start(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;

    // this.field.setWeather('desolateland');
    battle.field.set_weather(ID::from("desolateland"), None);

    EventResult::Continue
}

/// onAnySetWeather(target, source, weather) {
///     const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
///     if (this.field.getWeather().id === 'desolateland' && !strongWeathers.includes(weather.id)) return false;
/// }
pub fn on_any_set_weather(battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>) -> EventResult {
    // const strongWeathers = ['desolateland', 'primordialsea', 'deltastream'];
    let strong_weathers = ["desolateland", "primordialsea", "deltastream"];

    // Get current weather
    let current_weather = battle.field.get_weather().as_str();

    // Get the incoming weather from the event (relay_var_type contains the weather ID)
    let incoming_weather = if let Some(ref event) = battle.current_event {
        if let Some(ref weather_id) = event.relay_var_type {
            weather_id.as_str()
        } else {
            return EventResult::Continue;
        }
    } else {
        return EventResult::Continue;
    };

    // if (this.field.getWeather().id === 'desolateland' && !strongWeathers.includes(weather.id)) return false;
    if current_weather == "desolateland" && !strong_weathers.contains(&incoming_weather) {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onEnd(pokemon) {
///     if (this.field.weatherState.source !== pokemon) return;
///     for (const target of this.getAllActive()) {
///         if (target === pokemon) continue;
///         if (target.hasAbility('desolateland')) {
///             this.field.weatherState.source = target;
///             return;
///         }
///     }
///     this.field.clearWeather();
/// }
pub fn on_end(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    // This requires EffectState to have a source field tracking the weather source Pokemon
    // Currently EffectState only has id, effectOrder, duration, layers, and source_slot
    // JavaScript's EffectState.source stores the Pokemon that set the weather
    // Need to either:
    // 1. Add source: Option<(usize, usize)> to EffectState
    // 2. Use a different mechanism to track weather source
    EventResult::Continue
}

