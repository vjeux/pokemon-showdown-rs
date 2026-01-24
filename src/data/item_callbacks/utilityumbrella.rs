//! Utility Umbrella Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (!pokemon.ignoringItem()) return;
///     if (['sunnyday', 'raindance', 'desolateland', 'primordialsea'].includes(this.field.effectiveWeather())) {
///         this.runEvent('WeatherChange', pokemon, pokemon, this.effect);
///     }
/// }
pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // if (!pokemon.ignoringItem()) return;
    let pokemon_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let is_ignoring_item = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        !pokemon.ignoring_item(battle, false)
    };

    if !is_ignoring_item {
        return EventResult::Continue;
    }

    // if (['sunnyday', 'raindance', 'desolateland', 'primordialsea'].includes(this.field.effectiveWeather()))
    let weather = battle.effective_weather();
    let weather_str = weather.as_str();

    if weather_str == "sunnyday"
        || weather_str == "raindance"
        || weather_str == "desolateland"
        || weather_str == "primordialsea"
    {
        // this.runEvent('WeatherChange', pokemon, pokemon, this.effect);
        let effect = battle.current_effect_id().map(|id| battle.make_item_effect(id));
        battle.run_event("WeatherChange", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), Some(pokemon_pos), effect.as_ref(), EventResult::Continue, false, false);
    }

    EventResult::Continue
}

/// onUpdate(pokemon) {
///     if (!this.effectState.inactive) return;
///     this.effectState.inactive = false;
///     if (['sunnyday', 'raindance', 'desolateland', 'primordialsea'].includes(this.field.effectiveWeather())) {
///         this.runEvent('WeatherChange', pokemon, pokemon, this.effect);
///     }
/// }
pub fn on_update(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (!this.effectState.inactive) return;
    let is_inactive = battle.with_effect_state_ref(|state| {
        state.inactive.unwrap_or(false)
    }).unwrap_or(false);

    if !is_inactive {
        return EventResult::Continue;
    }

    // this.effectState.inactive = false;
    battle.with_effect_state(|state| {
        state.inactive = Some(false);
    });

    // if (['sunnyday', 'raindance', 'desolateland', 'primordialsea'].includes(this.field.effectiveWeather()))
    let weather = battle.effective_weather();
    let weather_str = weather.as_str();

    if weather_str == "sunnyday"
        || weather_str == "raindance"
        || weather_str == "desolateland"
        || weather_str == "primordialsea"
    {
        // this.runEvent('WeatherChange', pokemon, pokemon, this.effect);
        let effect = battle.current_effect_id().map(|id| battle.make_item_effect(id));
        battle.run_event("WeatherChange", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), Some(pokemon_pos), effect.as_ref(), EventResult::Continue, false, false);
    }

    EventResult::Continue
}

/// onEnd(pokemon) {
///     if (['sunnyday', 'raindance', 'desolateland', 'primordialsea'].includes(this.field.effectiveWeather())) {
///         this.runEvent('WeatherChange', pokemon, pokemon, this.effect);
///     }
///     this.effectState.inactive = true;
/// }
pub fn on_end(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (['sunnyday', 'raindance', 'desolateland', 'primordialsea'].includes(this.field.effectiveWeather()))
    let weather = battle.effective_weather();
    let weather_str = weather.as_str();

    if weather_str == "sunnyday"
        || weather_str == "raindance"
        || weather_str == "desolateland"
        || weather_str == "primordialsea"
    {
        // this.runEvent('WeatherChange', pokemon, pokemon, this.effect);
        let effect = battle.current_effect_id().map(|id| battle.make_item_effect(id));
        battle.run_event("WeatherChange", Some(crate::event::EventTarget::Pokemon(pokemon_pos)), Some(pokemon_pos), effect.as_ref(), EventResult::Continue, false, false);
    }

    // this.effectState.inactive = true;
    battle.with_effect_state(|state| {
        state.inactive = Some(true);
    });

    EventResult::Continue
}
