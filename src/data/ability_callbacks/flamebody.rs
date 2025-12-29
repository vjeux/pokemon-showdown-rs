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
pub fn on_damaging_hit(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

