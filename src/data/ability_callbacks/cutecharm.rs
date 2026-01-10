//! Cute Charm Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (this.checkMoveMakesContact(move, source, target)) {
///         if (this.randomChance(3, 10)) {
///             source.addVolatile('attract', this.effectState.target);
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

    // Debug: Log that cutecharm on_damaging_hit was called
    let move_id = active_move.map(|m| m.id.as_str()).unwrap_or("none");
    let has_contact = active_move.map(|m| m.flags.contact).unwrap_or(false);
    let base_move = active_move.and_then(|m| m.base_move.as_ref()).map(|s| s.as_str()).unwrap_or("none");
    eprintln!("[CUTECHARM] on_damaging_hit called, move={}, has_contact={}, base_move={}, target_pos={:?}, source_pos={:?}",
        move_id, has_contact, base_move, target_pos, source_pos);

    // if (this.checkMoveMakesContact(move, source, target))
    // source = attacker, target = defender (the one with Cute Charm)
    // IMPORTANT: Use the ActiveMove directly to get the correct flags (including inherited flags for G-Max moves)
    let makes_contact = battle.check_move_makes_contact_with_active_move(active_move, source_pos, target_pos, false);
    eprintln!("[CUTECHARM] check_move_makes_contact_with_active_move returned {}", makes_contact);

    if makes_contact {
        // if (this.randomChance(3, 10))
        if battle.random_chance(3, 10) {
            // source.addVolatile('attract', this.effectState.target);
            // The second parameter in JS is the source of the attract (the Pokemon with Cute Charm)
            // In the Rust API, this is passed as source_pos to add_volatile
            crate::pokemon::Pokemon::add_volatile(battle, source_pos, // The attacker gets attracted
                crate::ID::from("attract"), Some(target_pos), // The target (Cute Charm holder) is the source of attract
                None, None,
            None);
        }
    }

    EventResult::Continue
}

