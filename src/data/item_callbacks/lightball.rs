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
    // if (pokemon.baseSpecies.baseSpecies === 'Pikachu')
    let is_pikachu = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.dex.species().get(pokemon.base_species.as_str())
            .and_then(|species| species.base_species.as_ref())
            .map(|base_species| base_species == "Pikachu")
            .unwrap_or(false)
    };

    if is_pikachu {
        // return this.chainModify(2);
        battle.chain_modify(2.0);
    }

    EventResult::Continue
}

/// onModifySpA(spa, pokemon) {
///     if (pokemon.baseSpecies.baseSpecies === 'Pikachu') {
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_sp_a(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.baseSpecies.baseSpecies === 'Pikachu')
    let is_pikachu = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.dex.species().get(pokemon.base_species.as_str())
            .and_then(|species| species.base_species.as_ref())
            .map(|base_species| base_species == "Pikachu")
            .unwrap_or(false)
    };

    if is_pikachu {
        // return this.chainModify(2);
        battle.chain_modify(2.0);
    }

    EventResult::Continue
}
