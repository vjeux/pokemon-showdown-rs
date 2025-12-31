//! Effect Spore Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

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
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // Check if move makes contact
    let _target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // Get move to check if it makes contact
    let move_data = match battle.dex.moves().get(move_id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // JavaScript: if (this.checkMoveMakesContact(move, source, target) && !source.status && source.runStatusImmunity('powder'))
    if !move_data.flags.contains_key("contact") {
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

    // TODO: Check runStatusImmunity('powder') - for now, always pass

    // JavaScript: const r = this.random(100);
    let r = battle.random(100) as i32;

    // Apply status based on random roll
    // Get mutable reference to source Pokemon
    let source_mut = match battle.pokemon_at_mut(source_pos.0, source_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    if r < 11 {
        // JavaScript: source.setStatus('slp', target);
        source_mut.try_set_status(crate::dex_data::ID::from("slp"), None);
    } else if r < 21 {
        // JavaScript: source.setStatus('par', target);
        source_mut.try_set_status(crate::dex_data::ID::from("par"), None);
    } else if r < 30 {
        // JavaScript: source.setStatus('psn', target);
        source_mut.try_set_status(crate::dex_data::ID::from("psn"), None);
    }

    EventResult::Continue
}

