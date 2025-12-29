//! Lucky Punch Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyCritRatio(critRatio, user) {
///     if (user.baseSpecies.name === 'Chansey') {
///         return critRatio + 2;
///     }
/// }
pub fn on_modify_crit_ratio(battle: &mut Battle, crit_ratio: i32) -> EventResult {
    // Get user from current event target
    let pokemon_pos = match battle.current_event.as_ref().and_then(|e| e.target) {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (user.baseSpecies.name === 'Chansey')
    let is_chansey = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_species.as_str() == "chansey"
    };

    if is_chansey {
        // return critRatio + 2;
        return EventResult::Number(crit_ratio + 2);
    }

    EventResult::Continue
}
