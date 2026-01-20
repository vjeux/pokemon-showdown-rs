//! Cornerstone Mask Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (user.baseSpecies.name.startsWith('Ogerpon-Cornerstone')) {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (user.baseSpecies.name.startsWith('Ogerpon-Cornerstone'))
    let starts_with_ogerpon_cornerstone = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.dex.species().get(pokemon.base_species.as_str())
            .map(|species| species.name.starts_with("Ogerpon-Cornerstone"))
            .unwrap_or(false)
    };

    if starts_with_ogerpon_cornerstone {
        // return this.chainModify([4915, 4096]);
        battle.chain_modify_fraction(4915, 4096);
    }

    EventResult::Continue
}

/// onTakeItem(item, source) {
///     if (source.baseSpecies.baseSpecies === 'Ogerpon') return false;
///     return true;
/// }
pub fn on_take_item(battle: &mut Battle, _item_pos: Option<(usize, usize)>, pokemon_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // if (source.baseSpecies.baseSpecies === 'Ogerpon') return false;
    // In JavaScript, pokemon.baseSpecies is a Species object, and species.baseSpecies
    // is the name of the base species (or the species name itself if it's the base form).

    // Get source position - in the TakeItem callback, source is the Pokemon holding the item
    let source = source_pos.unwrap_or(pokemon_pos);

    let base_species_is_ogerpon = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        // Get the species from dex
        let species = battle.dex.species().get(source_pokemon.base_species.as_str());
        match species {
            Some(s) => {
                // If the species has a baseSpecies field (it's a forme), use that
                // Otherwise, the species IS the base species, so use its name
                let base_species_name = s.base_species.as_ref()
                    .map(|bs| bs.as_str())
                    .unwrap_or(s.name.as_str());
                base_species_name == "Ogerpon"
            }
            None => false,
        }
    };

    if base_species_is_ogerpon {
        // return false;
        return EventResult::Boolean(false);
    }

    // return true;
    EventResult::Boolean(true)
}
