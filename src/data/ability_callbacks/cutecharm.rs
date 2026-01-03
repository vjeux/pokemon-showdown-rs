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
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (this.checkMoveMakesContact(move, source, target))
    if battle.check_move_makes_contact(&crate::ID::from(move_id), target_pos, source_pos, false) {
        // if (this.randomChance(3, 10))
        if battle.random_chance(3, 10) {
            // source.addVolatile('attract', this.effectState.target);
            // The second parameter in JS is the source of the attract (the Pokemon with Cute Charm)
            // In the Rust API, this is passed as source_pos to add_volatile
            crate::pokemon::Pokemon::add_volatile(
                battle,
                source_pos,  // The attacker gets attracted
                crate::ID::from("attract"),
                Some(target_pos),  // The target (Cute Charm holder) is the source of attract
                None,
                None,
            );
        }
    }

    EventResult::Continue
}

