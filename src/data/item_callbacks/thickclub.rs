//! Thick Club Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyAtk(atk, pokemon) {
///     if (pokemon.baseSpecies.baseSpecies === 'Cubone' || pokemon.baseSpecies.baseSpecies === 'Marowak') {
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_atk(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Get pokemon
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Get pokemon's base species to access baseSpecies.baseSpecies
    let base_species_name = match pokemon.get_base_species_base_species(&battle.dex) {
        Some(name) => name,
        None => return EventResult::Continue,
    };

    // if (pokemon.baseSpecies.baseSpecies === 'Cubone' || pokemon.baseSpecies.baseSpecies === 'Marowak') {
    //     return this.chainModify(2);
    // }
    if base_species_name == "Cubone" || base_species_name == "Marowak" {
        let result = battle.chain_modify(2.0);
        return EventResult::Number(result);
    }

    EventResult::Continue
}
