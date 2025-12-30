//! Leek Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyCritRatio(critRatio, user) {
///     if (["farfetchd", "sirfetchd"].includes(this.toID(user.baseSpecies.baseSpecies))) {
///         return critRatio + 2;
///     }
/// }
pub fn on_modify_crit_ratio(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    crit_ratio: i32,
) -> EventResult {

    // if (["farfetchd", "sirfetchd"].includes(this.toID(user.baseSpecies.baseSpecies)))
    let is_farfetchd_family = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Get species data to access baseSpecies field
        let species_data = match battle.dex.get_species_by_id(&pokemon.base_species) {
            Some(s) => s,
            None => return EventResult::Continue,
        };

        // Get the ultimate base species (or current species if base_species is None)
        let ultimate_base = species_data
            .base_species
            .as_deref()
            .unwrap_or_else(|| pokemon.base_species.as_str());

        ultimate_base == "farfetchd" || ultimate_base == "sirfetchd"
    };

    if is_farfetchd_family {
        // return critRatio + 2;
        return EventResult::Number(crit_ratio + 2);
    }

    EventResult::Continue
}
