//! Rocky Helmet Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (this.checkMoveMakesContact(move, source, target)) {
///         this.damage(source.baseMaxhp / 6, source, target);
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, damage: i32, target_pos: (usize, usize), source_pos: (usize, usize)) -> EventResult {
    // if (this.checkMoveMakesContact(move, source, target)) {
    //     this.damage(source.baseMaxhp / 6, source, target);
    // }
    // TODO: Need battle.checkMoveMakesContact() and battle.damage() to deal 1/6 max HP damage
    // to attacker when hit by a contact move
    EventResult::Continue
}
