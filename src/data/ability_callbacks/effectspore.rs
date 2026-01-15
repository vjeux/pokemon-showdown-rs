//! Effect Spore Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// onDamagingHit(damage, target, source, move) {
///     if (this.checkMoveMakesContact(move, source, target) && !source.status && source.runStatusImmunity('powder')) {
///         const r = this.random(100);
///         if (r < 11) {
///             source.setStatus('slp', target);
///         } else if (r < 21) {
///             source.setStatus('par', target);
///         } else if (r < 30) {
///             source.setStatus('psn', target);
///         }
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Check if move makes contact
    let _target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get active_move to check if it makes contact (use runtime flags, not dex data)
    // Shell Side Arm dynamically adds contact flag via onModifyMove when it uses physical attack
    let has_contact = match active_move {
        Some(m) => m.flags.contact,
        None => return EventResult::Continue,
    };

    // JavaScript: if (this.checkMoveMakesContact(move, source, target) && !source.status && source.runStatusImmunity('powder'))
    if !has_contact {
        return EventResult::Continue;
    }

    // Check if source has no status
    let source_status = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.status.clone()
    };

    if !source_status.is_empty() {
        return EventResult::Continue;
    }

    // Check runStatusImmunity('powder') - returns true if immune (should not apply status)
    // JavaScript: source.runStatusImmunity('powder')
    if !Pokemon::run_status_immunity(battle, source_pos, "powder", false) {
        return EventResult::Continue;
    }

    // JavaScript: const r = this.random(100);
    let r = battle.random(100) as i32;

    // Note: target_pos (the Effect Spore Pokemon) is the source of the status
    // This is important for Synchronize to know who to pass the status back to

    // Apply status based on random roll
    if r < 11 {
        // JavaScript: source.setStatus('slp', target);
        Pokemon::try_set_status(battle, source_pos, crate::dex_data::ID::from("slp"), target_pos, None);
    } else if r < 21 {
        // JavaScript: source.setStatus('par', target);
        Pokemon::try_set_status(battle, source_pos, crate::dex_data::ID::from("par"), target_pos, None);
    } else if r < 30 {
        // JavaScript: source.setStatus('psn', target);
        Pokemon::try_set_status(battle, source_pos, crate::dex_data::ID::from("psn"), target_pos, None);
    }

    EventResult::Continue
}

