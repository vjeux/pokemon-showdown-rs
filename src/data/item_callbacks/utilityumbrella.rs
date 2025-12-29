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
        !pokemon.ignoring_item()
    };

    if !is_ignoring_item {
        return EventResult::Continue;
    }

    // if (['sunnyday', 'raindance', 'desolateland', 'primordialsea'].includes(this.field.effectiveWeather()))
    let weather = battle.field.effective_weather();
    let weather_str = weather.as_str();

    if weather_str == "sunnyday"
        || weather_str == "raindance"
        || weather_str == "desolateland"
        || weather_str == "primordialsea"
    {
        // this.runEvent('WeatherChange', pokemon, pokemon, this.effect);
        let effect = battle.current_effect.clone();
        battle.run_event(
            "WeatherChange",
            Some(pokemon_pos),
            Some(pokemon_pos),
            effect.as_ref(),
            None,
        );
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
    let is_inactive = {
        if let Some(ref effect_state) = battle.current_effect_state {
            effect_state
                .data
                .get("inactive")
                .and_then(|v| v.as_bool())
                .unwrap_or(false)
        } else {
            false
        }
    };

    if !is_inactive {
        return EventResult::Continue;
    }

    // this.effectState.inactive = false;
    if let Some(ref mut effect_state) = battle.current_effect_state {
        effect_state
            .data
            .insert("inactive".to_string(), serde_json::json!(false));
    }

    // if (['sunnyday', 'raindance', 'desolateland', 'primordialsea'].includes(this.field.effectiveWeather()))
    let weather = battle.field.effective_weather();
    let weather_str = weather.as_str();

    if weather_str == "sunnyday"
        || weather_str == "raindance"
        || weather_str == "desolateland"
        || weather_str == "primordialsea"
    {
        // this.runEvent('WeatherChange', pokemon, pokemon, this.effect);
        let effect = battle.current_effect.clone();
        battle.run_event(
            "WeatherChange",
            Some(pokemon_pos),
            Some(pokemon_pos),
            effect.as_ref(),
            None,
        );
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
    let weather = battle.field.effective_weather();
    let weather_str = weather.as_str();

    if weather_str == "sunnyday"
        || weather_str == "raindance"
        || weather_str == "desolateland"
        || weather_str == "primordialsea"
    {
        // this.runEvent('WeatherChange', pokemon, pokemon, this.effect);
        let effect = battle.current_effect.clone();
        battle.run_event(
            "WeatherChange",
            Some(pokemon_pos),
            Some(pokemon_pos),
            effect.as_ref(),
            None,
        );
    }

    // this.effectState.inactive = true;
    if let Some(ref mut effect_state) = battle.current_effect_state {
        effect_state
            .data
            .insert("inactive".to_string(), serde_json::json!(true));
    }

    EventResult::Continue
}
