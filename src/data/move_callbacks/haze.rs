//! Haze Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHitField() {
///     this.add('-clearallboost');
///     for (const pokemon of this.getAllActive()) {
///         pokemon.clearBoosts();
///     }
/// }
pub fn on_hit_field(battle: &mut Battle) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

