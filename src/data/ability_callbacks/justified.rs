//! Justified Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (move.type === 'Dark') {
///         this.boost({ atk: 1 });
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Boost Attack by 1 stage when hit by a Dark-type move
    if let Some(target) = target_pos {
        // JavaScript checks move.type (the active move's type, not the dex type)
        let is_dark_type = active_move.map(|m| m.move_type == "Dark").unwrap_or(false);

        if is_dark_type {
            battle.boost(&[("atk", 1)], target, None, None, false, false);
        }
    }
    EventResult::Continue
}
