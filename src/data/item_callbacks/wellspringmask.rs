//! Wellspring Mask Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (user.baseSpecies.name.startsWith('Ogerpon-Wellspring')) {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (user.baseSpecies.name.startsWith('Ogerpon-Wellspring')) {
    //     return this.chainModify([4915, 4096]);
    // }
    let starts_with_ogerpon_wellspring = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.dex.species().get_by_id(&pokemon.base_species)
            .map(|species| species.name.starts_with("Ogerpon-Wellspring"))
            .unwrap_or(false)
    };

    if starts_with_ogerpon_wellspring {
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
    // return true;

    // Get source position (using pokemon_pos as source in this context)
    let source = source_pos.unwrap_or(pokemon_pos);

    let base_species_is_ogerpon = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        battle.dex.species().get_by_id(&source_pokemon.base_species)
            .and_then(|species| species.base_species.as_ref())
            .map(|base_species| base_species == "Ogerpon")
            .unwrap_or(false)
    };

    if base_species_is_ogerpon {
        // return false;
        return EventResult::Boolean(false);
    }

    // return true;
    EventResult::Boolean(true)
}
