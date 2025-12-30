//! Griseous Orb Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, user, target, move) {
///     if (user.baseSpecies.num === 487 && (move.type === 'Ghost' || move.type === 'Dragon')) {
///         return this.chainModify([4915, 4096]);
///     }
/// }
pub fn on_base_power(battle: &mut Battle, _base_power: i32, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (user.baseSpecies.num === 487 && (move.type === 'Ghost' || move.type === 'Dragon'))

    // Check user.baseSpecies.num === 487
    let is_giratina = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        battle.dex.get_species(pokemon.base_species.as_str())
            .map(|s| s.num == 487)
            .unwrap_or(false)
    };

    if !is_giratina {
        return EventResult::Continue;
    }

    // move.type === 'Ghost' || move.type === 'Dragon'
    let is_ghost_or_dragon = battle.active_move.as_ref()
        .map(|m| m.move_type.as_str() == "Ghost" || m.move_type.as_str() == "Dragon")
        .unwrap_or(false);

    if is_ghost_or_dragon {
        // return this.chainModify([4915, 4096]);
        battle.chain_modify_fraction(4915, 4096);
    }

    EventResult::Continue
}
