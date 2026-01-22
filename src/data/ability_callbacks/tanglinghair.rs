//! Tangling Hair Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (this.checkMoveMakesContact(move, source, target, true)) {
///         this.add('-ability', target, 'Tangling Hair');
///         this.boost({ spe: -1 }, source, target, null, true);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Check if move makes contact
    if let (Some(target), Some(source)) = (target_pos, source_pos) {
        // IMPORTANT: Use the ActiveMove directly to get the correct flags (including inherited flags for G-Max moves)
        if battle.check_move_makes_contact_with_active_move(active_move, source, target, true) {
            // Lower attacker's Speed by 1 stage
            battle.boost(&[("spe", -1)], source, Some(target), None, true, false);
        }
    }
    EventResult::Continue
}

