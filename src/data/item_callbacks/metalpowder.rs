//! Metal Powder Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyDef(def, pokemon) {
///     if (pokemon.species.name === 'Ditto' && !pokemon.transformed) {
///         return this.chainModify(2);
///     }
/// }
pub fn on_modify_def(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // if (pokemon.species.name === 'Ditto' && !pokemon.transformed)
    let (species_name, transformed) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let species_name = battle.dex.get_species(pokemon.species_id.as_str())
            .map(|s| s.name.clone())
            .unwrap_or_default();

        (species_name, pokemon.transformed)
    };

    if species_name == "Ditto" && !transformed {
        // return this.chainModify(2);
        battle.chain_modify(2.0);
    }

    EventResult::Continue
}
