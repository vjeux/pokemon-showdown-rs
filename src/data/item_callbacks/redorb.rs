//! Red Orb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSwitchIn(pokemon) {
///     if (pokemon.isActive && pokemon.baseSpecies.name === 'Groudon' && !pokemon.transformed) {
///         pokemon.formeChange('Groudon-Primal', this.effect, true);
///     }
/// }
pub fn on_switch_in(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.isActive && pokemon.baseSpecies.name === 'Groudon' && !pokemon.transformed) {
    let (is_active, base_species_name, transformed) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let base_species_name = pokemon.get_base_species_name(&battle.dex);

        (pokemon.is_active, base_species_name, pokemon.transformed)
    };

    if is_active && base_species_name == Some("Groudon".to_string()) && !transformed {
        // pokemon.formeChange('Groudon-Primal', this.effect, true);
        // Get the Groudon-Primal species data
        let (new_types, new_ability) = {
            use crate::dex_data::ID;
            let primal_species = battle.dex.get_species("groudonprimal");
            match primal_species {
                Some(species) => {
                    let types = species.types.clone();
                    // Get the first ability from the Primal form
                    let ability = match &species.abilities.slot0 {
                        Some(ability_name) if !ability_name.is_empty() => {
                            Some(ID::from(ability_name.as_str()))
                        }
                        _ => None,
                    };
                    (types, ability)
                }
                None => return EventResult::Continue,
            }
        };

        // Call forme_change on the pokemon
        let pokemon_mut = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        use crate::dex_data::ID;
        pokemon_mut.forme_change(ID::from("groudonprimal"), new_types, new_ability);
    }

    EventResult::Continue
}

/// onTakeItem(item, source) {
///     if (source.baseSpecies.baseSpecies === 'Groudon') return false;
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if (source.baseSpecies.baseSpecies === 'Groudon') return false;
    if let Some(source) = source_pos {
        if let Some(source_pokemon) = battle.pokemon_at(source.0, source.1) {
            let source_species = battle.dex.get_species(source_pokemon.base_species.as_str());
            if let Some(species_data) = source_species {
                let base_species_name = species_data.base_species
                    .as_ref()
                    .unwrap_or(&species_data.name);
                if base_species_name == "Groudon" {
                    // return false;
                    return EventResult::Boolean(false);
                }
            }
        }
    }

    // return true;
    EventResult::Boolean(true)
}
