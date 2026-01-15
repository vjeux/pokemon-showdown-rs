//! Hunger Switch Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect};
use crate::event::EventResult;

/// onResidual(pokemon) {
///     if (pokemon.species.baseSpecies !== 'Morpeko' || pokemon.terastallized) return;
///     const targetForme = pokemon.species.name === 'Morpeko' ? 'Morpeko-Hangry' : 'Morpeko';
///     pokemon.formeChange(targetForme);
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::dex_data::ID;
    

    // if (pokemon.species.baseSpecies !== 'Morpeko' || pokemon.terastallized) return;
    let (base_species, terastallized, species_name) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let species_data = match battle.dex.species().get(pokemon.species_id.as_str()) {
            Some(data) => data,
            None => return EventResult::Continue,
        };

        (
            // JavaScript: species.baseSpecies defaults to species.name if not set
            species_data.base_species.clone().unwrap_or_else(|| species_data.name.clone()),
            pokemon.terastallized.clone(),
            species_data.name.clone(),
        )
    };

    if base_species != "Morpeko" || terastallized.is_some() {
        return EventResult::Continue;
    }

    // const targetForme = pokemon.species.name === 'Morpeko' ? 'Morpeko-Hangry' : 'Morpeko';
    let target_forme = if species_name == "Morpeko" {
        "morpekohangry"
    } else {
        "morpeko"
    };

    // pokemon.formeChange(targetForme);
    // pokemon_pos is already (side_idx, pokemon_index), pass it directly
    crate::pokemon::Pokemon::forme_change(
        battle,
        pokemon_pos,
        ID::from(target_forme),
        Some(Effect::ability("hungerswitch")),
        false,
        "0",
        None
    );

    EventResult::Continue
}

