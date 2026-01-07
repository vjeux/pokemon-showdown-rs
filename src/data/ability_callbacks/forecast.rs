//! Forecast Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     this.singleEvent('WeatherChange', this.effect, this.effectState, pokemon);
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    // this.singleEvent('WeatherChange', this.effect, this.effectState, pokemon);
    // This just calls the WeatherChange event, which we've implemented as on_weather_change
    on_weather_change(battle, pokemon_pos, None, None)
}

/// onWeatherChange(pokemon) {
///     if (pokemon.baseSpecies.baseSpecies !== 'Castform' || pokemon.transformed) return;
///     let forme = null;
///     switch (pokemon.effectiveWeather()) {
///     case 'sunnyday':
///     case 'desolateland':
///         if (pokemon.species.id !== 'castformsunny') forme = 'Castform-Sunny';
///         break;
///     case 'raindance':
///     case 'primordialsea':
///         if (pokemon.species.id !== 'castformrainy') forme = 'Castform-Rainy';
///         break;
///     case 'hail':
///     case 'snowscape':
///         if (pokemon.species.id !== 'castformsnowy') forme = 'Castform-Snowy';
///         break;
///     default:
///         if (pokemon.species.id !== 'castform') forme = 'Castform';
///         break;
///     }
///     if (pokemon.isActive && forme) {
///         pokemon.formeChange(forme, this.effect, false, '0', '[msg]');
///     }
/// }
pub fn on_weather_change(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::dex_data::ID;
    

    // if (pokemon.baseSpecies.baseSpecies !== 'Castform' || pokemon.transformed) return;
    let (is_active, base_species_base_species, transformed, species_id, effective_weather) = {
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
            pokemon.species_id.clone(),
            effective_weather.to_string(),
        )
    };

    if base_species_base_species.as_deref() != Some("Castform") || transformed {
        return EventResult::Continue;
    }

    // let forme = null;
    // switch (pokemon.effectiveWeather()) {
    let forme = match effective_weather.as_str() {
        // case 'sunnyday':
        // case 'desolateland':
        //     if (pokemon.species.id !== 'castformsunny') forme = 'Castform-Sunny';
        "sunnyday" | "desolateland" => {
            if species_id.as_str() != "castformsunny" {
                Some("castformsunny")
            } else {
                None
            }
        }
        // case 'raindance':
        // case 'primordialsea':
        //     if (pokemon.species.id !== 'castformrainy') forme = 'Castform-Rainy';
        "raindance" | "primordialsea" => {
            if species_id.as_str() != "castformrainy" {
                Some("castformrainy")
            } else {
                None
            }
        }
        // case 'hail':
        // case 'snowscape':
        //     if (pokemon.species.id !== 'castformsnowy') forme = 'Castform-Snowy';
        "hail" | "snowscape" => {
            if species_id.as_str() != "castformsnowy" {
                Some("castformsnowy")
            } else {
                None
            }
        }
        // default:
        //     if (pokemon.species.id !== 'castform') forme = 'Castform';
        _ => {
            if species_id.as_str() != "castform" {
                Some("castform")
            } else {
                None
            }
        }
    };

    // if (pokemon.isActive && forme) {
    //     pokemon.formeChange(forme, this.effect, false, '0', '[msg]');
    // }
    if is_active {
        if let Some(forme_id) = forme {
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
                        crate::pokemon::Pokemon::forme_change(
                            battle_ref2,
                            (pokemon_pos.0, pokemon_index),
                            ID::from(forme_id),
                            Some(ID::from("forecast")),
                            false,
                            "0",
                            Some("[msg]")
                        );
                    }
                }
            }
        }
    }

    EventResult::Continue
}

