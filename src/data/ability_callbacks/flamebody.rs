//! Flame Body Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (this.checkMoveMakesContact(move, source, target)) {
///         if (this.randomChance(3, 10)) {
///             source.trySetStatus('brn', target);
///         }
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (this.checkMoveMakesContact(move, source, target))
    // source = attacker, target = defender (the one with Flame Body)
    // IMPORTANT: Use the ActiveMove directly to get the correct flags (including inherited flags for G-Max moves)
    if battle.check_move_makes_contact_with_active_move(active_move, source_pos, target_pos, false) {
        // if (this.randomChance(3, 10))
        if battle.random_chance(3, 10) {
            // source.trySetStatus('brn', target);
            crate::pokemon::Pokemon::try_set_status(battle, source_pos, crate::ID::from("brn"), None);
        }
    }

    EventResult::Continue
}

