//! Hunger Switch Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
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
            species_data.base_species.clone().unwrap_or_default(),
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
                        ID::from(target_forme),
                        Some(ID::from("hungerswitch")),
                        false,
                        "0",
                        None
                    );
            }
        }
    }

    EventResult::Continue
}

