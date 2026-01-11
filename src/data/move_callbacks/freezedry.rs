//! Freeze-Dry Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onEffectiveness(typeMod, target, type) {
///     if (type === 'Water') return 1;
/// }
pub fn on_effectiveness(_battle: &mut Battle, _type_mod: i32, target_type: &str) -> EventResult {
    // if (type === 'Water') return 1;
    // Note: JavaScript uses capitalized type names like "Water"
    if target_type == "Water" {
        return EventResult::Number(1);
    }

    EventResult::Continue
}
