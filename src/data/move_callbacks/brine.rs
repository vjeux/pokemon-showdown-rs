//! Brine Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBasePower(basePower, pokemon, target) {
///     if (target.hp * 2 <= target.maxhp) {
///         return this.chainModify(2);
///     }
/// }
pub fn on_base_power(
    battle: &mut Battle,
    _base_power: i32,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get target pokemon
    let target_pokemon = match battle.pokemon_at(target.0, target.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // if (target.hp * 2 <= target.maxhp) {
    //     return this.chainModify(2);
    // }
    if target_pokemon.hp * 2 <= target_pokemon.maxhp {
        let result = battle.chain_modify(2.0);
        return EventResult::Number(result);
    }

    EventResult::Continue
}
