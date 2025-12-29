//! Gooey Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (this.checkMoveMakesContact(move, source, target, true)) {
///         this.add('-ability', target, 'Gooey');
///         this.boost({ spe: -1 }, source, target, null, true);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

