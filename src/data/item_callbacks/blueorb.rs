//! Blue Orb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::{Battle, Effect};
use crate::dex_data::ID;
use crate::event::EventResult;

/// onSwitchIn(pokemon) {
///     if (pokemon.isActive && pokemon.baseSpecies.name === 'Kyogre' && !pokemon.transformed) {
///         pokemon.formeChange('Kyogre-Primal', this.effect, true);
///     }
/// }
pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.isActive && pokemon.baseSpecies.name === 'Kyogre' && !pokemon.transformed) {
    let (is_active, base_species_name, transformed) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_species_name = pokemon.get_base_species_name(&battle.dex);

        (pokemon.is_active, base_species_name, pokemon.transformed)
    };

    if is_active && base_species_name == Some("Kyogre".to_string()) && !transformed {
        // pokemon.formeChange('Kyogre-Primal', this.effect, true);
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
                        ID::from("kyogreprimal"),
                        Some(Effect::item("blueorb")),
                        true,
                        "0",
                        None,
                    );
                }
            }
        }
    }

    EventResult::Continue
}

/// onTakeItem(item, source) {
///     if (source.baseSpecies.baseSpecies === 'Kyogre') return false;
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, _item_pos: Option<(usize, usize)>, _pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if (source.baseSpecies.baseSpecies === 'Kyogre') return false;
    if let Some(source) = source_pos {
        if let Some(source_pokemon) = battle.pokemon_at(source.0, source.1) {
            let source_species = battle.dex.species().get(source_pokemon.base_species.as_str());
            if let Some(species_data) = source_species {
                let base_species_name = species_data.base_species
                    .as_ref()
                    .unwrap_or(&species_data.name);
                if base_species_name == "Kyogre" {
                    // return false;
                    return EventResult::Boolean(false);
                }
            }
        }
    }

    // return true;
    EventResult::Boolean(true)
}
