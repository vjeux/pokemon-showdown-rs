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

    let (filtered, should_debug) = if let Some(ref active_move) = battle.active_move {
        // Filter secondaries to keep only effects with self_secondary (self-targeting)
        // !!effect.self in JavaScript means "keep if self is truthy"
        let am = active_move.borrow();
        let filtered: Vec<_> = am.secondaries
            .iter()
            .filter(|effect| effect.self_secondary.is_some())
            .cloned()
            .collect();

        let should_debug = filtered.len() < am.secondaries.len();
        (Some(filtered), should_debug)
    } else {
        (None, false)
    };

    if should_debug {
        battle.debug("Shield Dust prevent secondary");
    }

    // Return the filtered secondaries so the caller uses them
    if let Some(filtered) = filtered {
        return EventResult::Secondaries(filtered);
    }

    EventResult::Continue
}

