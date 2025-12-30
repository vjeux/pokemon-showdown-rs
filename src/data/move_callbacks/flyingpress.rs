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
    // return typeMod + this.dex.getEffectiveness('Flying', type);
    let flying_effectiveness = battle.dex.get_effectiveness("Flying", target_type);
    EventResult::Number(type_mod + flying_effectiveness)
}
