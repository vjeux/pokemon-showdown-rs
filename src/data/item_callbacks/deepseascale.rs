//! Deep Sea Scale Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifySpD(spd, pokemon) {
///     if (pokemon.baseSpecies.name === 'Clamperl') {
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_sp_d(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.baseSpecies.name === 'Clamperl')
    let is_clamperl = {
        if let Some(pokemon) = battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            if let Some(species) = battle.dex.species().get(pokemon.base_species.as_str()) {
                species.name == "Clamperl"
            } else {
                false
            }
        } else {
            false
        }
    };

    if is_clamperl {
        // return this.chainModify(2);
        battle.chain_modify(2.0);
    }

    EventResult::Continue
}
