//! Hold Back Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamage(damage, target, source, effect) {
///     if (damage >= target.hp) return target.hp - 1;
/// }
pub fn on_damage(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    // Note: This callback modifies damage dealt to prevent KO
    // The actual implementation needs damage context that isn't available in current signature
    // TODO: Need damage value and target position to properly implement
    EventResult::Continue
}

