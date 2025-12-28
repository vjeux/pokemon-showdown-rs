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
    let flying_effectiveness = battle.dex.get_effectiveness(&ID::from("flying"), &ID::from(target_type));

    EventResult::Number(type_mod + flying_effectiveness)
}

