//! Flower Gift Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     this.singleEvent('WeatherChange', this.effect, this.effectState, pokemon);
/// }
pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onWeatherChange(pokemon) {
///     if (!pokemon.isActive || pokemon.baseSpecies.baseSpecies !== 'Cherrim' || pokemon.transformed) return;
///     if (!pokemon.hp) return;
///     if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather())) {
///         if (pokemon.species.id !== 'cherrimsunshine') {
///             pokemon.formeChange('Cherrim-Sunshine', this.effect, false, '0', '[msg]');
///         }
///     } else {
///         if (pokemon.species.id === 'cherrimsunshine') {
///             pokemon.formeChange('Cherrim', this.effect, false, '0', '[msg]');
///         }
///     }
/// }
pub fn on_weather_change(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onAllyModifyAtk(atk, pokemon) {
///     if (this.effectState.target.baseSpecies.baseSpecies !== 'Cherrim') return;
///     if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather())) {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_ally_modify_atk(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // if (this.effectState.target.baseSpecies.baseSpecies !== 'Cherrim') return;
    let ability_holder_is_cherrim = {
        let ability_holder_pos = match battle.effect_state.target {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let ability_holder = match battle.pokemon_at(ability_holder_pos.0, ability_holder_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        ability_holder.get_base_species_base_species(&battle.dex).as_deref() == Some("Cherrim")
    };

    if !ability_holder_is_cherrim {
        return EventResult::Continue;
    }

    // if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather()))
    let ally_pos = match &battle.current_event {
        Some(event) => event.target.unwrap_or((0, 0)),
        None => return EventResult::Continue,
    };

    let is_sunny = {
        let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let field_weather = battle.field.get_weather();
        let effective_weather = ally.effective_weather(battle, field_weather.as_str());

        effective_weather == "sunnyday" || effective_weather == "desolateland"
    };

    if is_sunny {
        // return this.chainModify(1.5);
        return EventResult::Number(battle.chain_modify(1.5));
    }

    EventResult::Continue
}

/// onAllyModifySpD(spd, pokemon) {
///     if (this.effectState.target.baseSpecies.baseSpecies !== 'Cherrim') return;
///     if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather())) {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_ally_modify_sp_d(battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // if (this.effectState.target.baseSpecies.baseSpecies !== 'Cherrim') return;
    let ability_holder_is_cherrim = {
        let ability_holder_pos = match battle.effect_state.target {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let ability_holder = match battle.pokemon_at(ability_holder_pos.0, ability_holder_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        ability_holder.get_base_species_base_species(&battle.dex).as_deref() == Some("Cherrim")
    };

    if !ability_holder_is_cherrim {
        return EventResult::Continue;
    }

    // if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather()))
    let ally_pos = match &battle.current_event {
        Some(event) => event.target.unwrap_or((0, 0)),
        None => return EventResult::Continue,
    };

    let is_sunny = {
        let ally = match battle.pokemon_at(ally_pos.0, ally_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let field_weather = battle.field.get_weather();
        let effective_weather = ally.effective_weather(battle, field_weather.as_str());

        effective_weather == "sunnyday" || effective_weather == "desolateland"
    };

    if is_sunny {
        // return this.chainModify(1.5);
        return EventResult::Number(battle.chain_modify(1.5));
    }

    EventResult::Continue
}

