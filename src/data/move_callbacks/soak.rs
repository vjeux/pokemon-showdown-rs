//! Soak Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target) {
///     if (target.getTypes().join() === 'Water' || !target.setType('Water')) {
///         // Soak should animate even when it fails.
///         // Returning false would suppress the animation.
///         this.add('-fail', target);
///         return null;
///     }
///     this.add('-start', target, 'typechange', 'Water');
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

