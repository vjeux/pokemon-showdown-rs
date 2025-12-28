//! Flying Press Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onEffectiveness(typeMod, target, type, move) {
///     return typeMod + this.dex.getEffectiveness('Flying', type);
/// }
pub fn on_effectiveness(battle: &mut Battle, type_mod: i32, target_type: &str) -> EventResult {
    use crate::dex_data::ID;

    // return typeMod + this.dex.getEffectiveness('Flying', type);
    let flying_effectiveness = battle.dex.get_effectiveness("flying", target_type);
    // Convert f64 effectiveness to i32 log2 representation
    // In Pokemon Showdown, type effectiveness is stored as log2 values
    // 2.0 -> 1, 1.0 -> 0, 0.5 -> -1, 0.0 -> very negative number
    let effectiveness_log = if flying_effectiveness == 0.0 {
        -5  // Treat immunity as very negative
    } else {
        flying_effectiveness.log2() as i32
    };

    EventResult::Number(type_mod + effectiveness_log)
}

