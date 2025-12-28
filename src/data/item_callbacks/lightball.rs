//! Light Ball Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyAtk(atk, pokemon) {
///     if (pokemon.baseSpecies.baseSpecies === 'Pikachu') {
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_atk(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Get pokemon's base_species
    let base_species_id = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_species.clone()
    };

    // Look up the species data to get base_species field
    let species_data = match battle.dex.get_species_by_id(&base_species_id) {
        Some(s) => s,
        None => return EventResult::Continue,
    };

    // Check if pokemon.baseSpecies.baseSpecies === 'Pikachu'
    let is_pikachu = match &species_data.base_species {
        Some(base) => base.to_lowercase() == "pikachu",
        None => base_species_id.to_string().to_lowercase() == "pikachu",
    };

    if is_pikachu {
        // return this.chainModify(2);
        return EventResult::Number(battle.chain_modify(2.0));
    }

    EventResult::Continue
}

/// onModifySpA(spa, pokemon) {
///     if (pokemon.baseSpecies.baseSpecies === 'Pikachu') {
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_sp_a(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Get pokemon's base_species
    let base_species_id = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_species.clone()
    };

    // Look up the species data to get base_species field
    let species_data = match battle.dex.get_species_by_id(&base_species_id) {
        Some(s) => s,
        None => return EventResult::Continue,
    };

    // Check if pokemon.baseSpecies.baseSpecies === 'Pikachu'
    let is_pikachu = match &species_data.base_species {
        Some(base) => base.to_lowercase() == "pikachu",
        None => base_species_id.to_string().to_lowercase() == "pikachu",
    };

    if is_pikachu {
        // return this.chainModify(2);
        return EventResult::Number(battle.chain_modify(2.0));
    }

    EventResult::Continue
}
