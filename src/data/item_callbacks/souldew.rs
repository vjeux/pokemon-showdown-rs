//! Soul Dew Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (
///         move && (user.baseSpecies.num === 380 || user.baseSpecies.num === 381) &&
///         (move.type === 'Psychic' || move.type === 'Dragon')
///     ) {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move && (user.baseSpecies.num === 380 || user.baseSpecies.num === 381) && (move.type === 'Psychic' || move.type === 'Dragon'))
    let (species_num, move_type) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let species_num = battle.dex.species().get(pokemon.base_species.as_str())
            .map(|s| s.num)
            .unwrap_or(0);
        let move_type = battle.active_move.as_ref()
            .map(|m| m.borrow().move_type.clone())
            .unwrap_or_default();
        (species_num, move_type)
    };

    if (species_num == 380 || species_num == 381) && (move_type == "Psychic" || move_type == "Dragon") {
        // return this.chainModify([4915, 4096]);
        battle.chain_modify_fraction(4915, 4096);
    }

    EventResult::Continue
}
