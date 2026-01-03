//! Embody Aspect (Cornerstone) Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (pokemon.baseSpecies.name === 'Ogerpon-Cornerstone-Tera' && pokemon.terastallized &&
///         !this.effectState.embodied) {
///         this.effectState.embodied = true;
///         this.boost({ def: 1 }, pokemon);
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.baseSpecies.name === 'Ogerpon-Cornerstone-Tera' && pokemon.terastallized && !this.effectState.embodied)
    let (base_species_name, is_terastallized, already_embodied) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_species_name = pokemon.get_base_species_name(&battle.dex);
        let is_terastallized = pokemon.terastallized.is_some();
        let already_embodied = pokemon
            .ability_state
            .data
            .get("embodied")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        (base_species_name, is_terastallized, already_embodied)
    };

    if base_species_name.as_deref() == Some("Ogerpon-Cornerstone-Tera")
        && is_terastallized
        && !already_embodied
    {
        // this.effectState.embodied = true;
        {
            let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_mut.ability_state.data.insert("embodied".to_string(), serde_json::json!(true));
        }

        // this.boost({ def: 1 }, pokemon);
        battle.boost(
            &[("def", 1)],
            pokemon_pos,
            Some(pokemon_pos),
            None,
            false,
            false,
        );
    }

    EventResult::Continue
}

