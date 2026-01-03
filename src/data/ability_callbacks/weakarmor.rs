//! Weak Armor Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (move.category === 'Physical') {
///         this.boost({ def: -1, spe: 2 }, target, target);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // When hit by Physical move, lower Defense by 1 and boost Speed by 2
    if let Some(target) = target_pos {
        // Check if the move is Physical category
        let is_physical = {
            let move_data = match battle.dex.moves().get(move_id) {
                Some(m) => m,
                None => return EventResult::Continue,
            };
            move_data.category == "Physical"
        };

        if is_physical {
            // Lower Defense by 1 and boost Speed by 2
            battle.boost(&[("def", -1), ("spe", 2)], target, Some(target), None, false, false);
        }
    }
    EventResult::Continue
}

