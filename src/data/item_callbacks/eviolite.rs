//! Eviolite Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyDef(def, pokemon) {
///     if (pokemon.baseSpecies.nfe) {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_modify_def(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.baseSpecies.nfe)
    let is_nfe = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.dex.species().get(pokemon.base_species.as_str())
            .map(|species| !species.evos.is_empty())
            .unwrap_or(false)
    };

    if is_nfe {
        // return this.chainModify(1.5);
        battle.chain_modify(1.5);
    }

    EventResult::Continue
}

/// onModifySpD(spd, pokemon) {
///     if (pokemon.baseSpecies.nfe) {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_modify_sp_d(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.baseSpecies.nfe)
    let is_nfe = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.dex.species().get(pokemon.base_species.as_str())
            .map(|species| !species.evos.is_empty())
            .unwrap_or(false)
    };

    if is_nfe {
        // return this.chainModify(1.5);
        battle.chain_modify(1.5);
    }

    EventResult::Continue
}
