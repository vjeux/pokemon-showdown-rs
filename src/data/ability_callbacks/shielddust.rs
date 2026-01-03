//! Shield Dust Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifySecondaries(secondaries) {
///     this.debug('Shield Dust prevent secondary');
///     return secondaries.filter(effect => !!effect.self);
/// }
pub fn on_modify_secondaries(battle: &mut Battle) -> EventResult {
    // JavaScript: return secondaries.filter(effect => !!effect.self);
    // This filters out all secondaries that don't have effect.self
    // In other words, it blocks all opponent-targeting secondaries

    if let Some(ref mut active_move) = battle.active_move {
        // Filter secondaries to keep only effects with self_effect = true
        // !!effect.self in JavaScript means "keep if self is truthy"
        let original_len = active_move.secondaries.len();
        active_move.secondaries.retain(|effect| effect.self_effect);

        if active_move.secondaries.len() < original_len {
            battle.debug("Shield Dust prevent secondary");
        }
    }

    EventResult::Continue
}

