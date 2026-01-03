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
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // this.singleEvent('WeatherChange', this.effect, this.effectState, pokemon);
    // This just calls the WeatherChange event, which we've implemented as on_weather_change
    on_weather_change(battle, pokemon_pos)
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
pub fn on_weather_change(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::dex_data::ID;
    use crate::Pokemon;

    // if (!pokemon.isActive || pokemon.baseSpecies.baseSpecies !== 'Cherrim' || pokemon.transformed) return;
    let (is_active, base_species_base_species, transformed, hp, species_id, effective_weather) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_species_base_species = pokemon.get_base_species_base_species(&battle.dex);
        let field_weather = battle.field.get_weather();
        let effective_weather = pokemon.effective_weather(battle, field_weather.as_str());

        (
            pokemon.is_active,
            base_species_base_species,
            pokemon.transformed,
            pokemon.hp,
            pokemon.species_id.clone(),
            effective_weather.to_string(),
        )
    };

    if !is_active || base_species_base_species.as_deref() != Some("Cherrim") || transformed {
        return EventResult::Continue;
    }

    // if (!pokemon.hp) return;
    if hp == 0 {
        return EventResult::Continue;
    }

    // if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather()))
    if effective_weather == "sunnyday" || effective_weather == "desolateland" {
        // if (pokemon.species.id !== 'cherrimsunshine')
        if species_id.as_str() != "cherrimsunshine" {
            // pokemon.formeChange('Cherrim-Sunshine', this.effect, false, '0', '[msg]');
            unsafe {
                // SAFETY: We're creating two mutable references to battle.
                // This is safe because we're accessing different parts of the battle structure.
                let battle_ptr = battle as *mut Battle;
                let battle_ref1 = &mut *battle_ptr;
                let battle_ref2 = &mut *battle_ptr;

                // Get pokemon directly from sides array
                let side = &mut battle_ref1.sides[pokemon_pos.0];
                let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
                if let Some(pokemon_index) = active_slot {
                    if pokemon_index < side.pokemon.len() {
                        let pokemon = &mut side.pokemon[pokemon_index];
                        pokemon.forme_change(battle_ref2, ID::from("cherrimsunshine"), Some(ID::from("flowergift")), false, "0", Some("[msg]"));
                    }
                }
            }
        }
    } else {
        // if (pokemon.species.id === 'cherrimsunshine')
        if species_id.as_str() == "cherrimsunshine" {
            // pokemon.formeChange('Cherrim', this.effect, false, '0', '[msg]');
            unsafe {
                // SAFETY: Same as above
                let battle_ptr = battle as *mut Battle;
                let battle_ref1 = &mut *battle_ptr;
                let battle_ref2 = &mut *battle_ptr;

                // Get pokemon directly from sides array
                let side = &mut battle_ref1.sides[pokemon_pos.0];
                let active_slot = side.active.get(pokemon_pos.1).cloned().flatten();
                if let Some(pokemon_index) = active_slot {
                    if pokemon_index < side.pokemon.len() {
                        let pokemon = &mut side.pokemon[pokemon_index];
                        pokemon.forme_change(battle_ref2, ID::from("cherrim"), Some(ID::from("flowergift")), false, "0", Some("[msg]"));
                    }
                }
            }
        }
    }

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

