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
pub fn on_base_power(battle: &mut Battle, base_power: i32, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // Double base power if target HP is 50% or less
    if target.hp * 2 <= target.maxhp {
        // chainModify(2) means multiply base power by 2
        EventResult::Number(base_power * 2)
    } else {
        EventResult::Continue
    }
}

