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
pub fn on_effectiveness(battle: &mut Battle, type_mod: i32, target_type: &str) -> EventResult {
    // if (type === 'Water') return 1;
    if target_type == "water" {
        return EventResult::Int(1);
    }

    EventResult::Continue
}

