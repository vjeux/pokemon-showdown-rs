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
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, _target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    eprintln!("[FLAME_BODY] onDamagingHit called with move_id={}", move_id);

    // Check if move makes contact
    let move_id = crate::ID::new(move_id);
    let move_data = match battle.dex.moves.get(&move_id) {
        Some(m) => m,
        None => {
            eprintln!("[FLAME_BODY] Move not found in dex");
            return EventResult::Continue;
        }
    };

    eprintln!("[FLAME_BODY] Move flags: {:?}", move_data.flags);

    // Check the contact flag
    if !move_data.flags.contains_key("contact") {
        eprintln!("[FLAME_BODY] Move does not make contact");
        return EventResult::Continue;
    }

    eprintln!("[FLAME_BODY] Move makes contact, checking random chance");

    // checkMoveMakesContact would also check source/target positions,
    // but the basic flag check should be sufficient for most cases
    // TODO: Implement full checkMoveMakesContact logic if needed

    // 30% chance to burn the source
    if !battle.random_chance(3, 10) {
        eprintln!("[FLAME_BODY] Random chance failed");
        return EventResult::Continue;
    }

    eprintln!("[FLAME_BODY] Random chance succeeded, attempting to burn");

    // Try to set burn status on the source Pokemon
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => {
            eprintln!("[FLAME_BODY] No source position");
            return EventResult::Continue;
        }
    };

    // trySetStatus in JavaScript
    if let Some(source) = battle.pokemon_at_mut(source_pos.0, source_pos.1) {
        eprintln!("[FLAME_BODY] Source status: {:?}", source.status);
        if source.status.is_empty() {
            source.status = crate::ID::new("brn");
            eprintln!("[FLAME_BODY] Burned source pokemon!");
            // TODO: Add battle message for burn
            // TODO: Call set_status or try_set_status properly to handle events
        } else {
            eprintln!("[FLAME_BODY] Source already has status");
        }
    }

    EventResult::Continue
}

