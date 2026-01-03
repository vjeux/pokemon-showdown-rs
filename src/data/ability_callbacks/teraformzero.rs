//! Teraform Zero Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterTerastallization(pokemon) {
///     if (pokemon.baseSpecies.name !== 'Terapagos-Stellar') return;
///     if (this.field.weather || this.field.terrain) {
///         this.add('-ability', pokemon, 'Teraform Zero');
///         this.field.clearWeather();
///         this.field.clearTerrain();
///     }
/// }
pub fn on_after_terastallization(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;

    // if (pokemon.baseSpecies.name !== 'Terapagos-Stellar') return;
    let base_species_name = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_base_species_name(&battle.dex)
    };

    if base_species_name.as_deref() != Some("Terapagos-Stellar") {
        return EventResult::Continue;
    }

    // if (this.field.weather || this.field.terrain)
    let has_weather_or_terrain = !battle.field.weather.is_empty() || !battle.field.terrain.is_empty();

    if has_weather_or_terrain {
        // this.add('-ability', pokemon, 'Teraform Zero');
        let pokemon_slot = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add("-ability", &[
            Arg::String(pokemon_slot),
            Arg::Str("Teraform Zero"),
        ]);

        // this.field.clearWeather();
        battle.field.clear_weather();

        // this.field.clearTerrain();
        battle.field.clear_terrain();
    }

    EventResult::Continue
}

