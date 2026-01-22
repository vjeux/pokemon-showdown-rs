//! Weak Armor Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (move.category === 'Physical') {
///         this.boost({ def: -1, spe: 2 }, target, target);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // When hit by Physical move, lower Defense by 1 and boost Speed by 2
    if let Some(target) = target_pos {
        // JavaScript checks move.category (the active move's category, not the dex category)
        // This is important for moves like Shell Side Arm which can change category
        let is_physical = active_move.map(|m| m.category == "Physical").unwrap_or(false);

        if is_physical {
            // Lower Defense by 1 and boost Speed by 2
            battle.boost(&[("def", -1), ("spe", 2)], target, Some(target), None, false, false);
        }
    }
    EventResult::Continue
}
