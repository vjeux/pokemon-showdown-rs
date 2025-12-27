//! Worry Seed Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryImmunity(target) {
///     // Truant and Insomnia have special treatment; they fail before
///     // checking accuracy and will double Stomping Tantrum's BP
///     if (target.ability === 'truant' || target.ability === 'insomnia') {
///         return false;
///     }
/// }
pub fn on_try_immunity(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onTryHit(target) {
///     if (target.getAbility().flags['cantsuppress']) {
///         return false;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onHit(target, source) {
///     const oldAbility = target.setAbility('insomnia');
///     if (!oldAbility) return oldAbility as false | null;
///     if (target.status === 'slp') target.cureStatus();
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

