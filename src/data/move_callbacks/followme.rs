//! Follow Me Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry(source) {
///     return this.activePerHalf > 1;
/// }
pub fn on_try(battle: &mut Battle, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target, source, effect) {
    ///     if (effect?.id === 'zpower') {
    ///         this.add('-singleturn', target, 'move: Follow Me', '[zeffect]');
    ///     } else {
    ///         this.add('-singleturn', target, 'move: Follow Me');
    ///     }
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
    /// onFoeRedirectTarget(target, source, source2, move) {
    ///     if (!this.effectState.target.isSkyDropped() && this.validTarget(this.effectState.target, source, move.target)) {
    ///         if (move.smartTarget) move.smartTarget = false;
    ///         this.debug("Follow Me redirected target of move");
    ///         return this.effectState.target;
    ///     }
    /// }
    pub fn on_foe_redirect_target(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
