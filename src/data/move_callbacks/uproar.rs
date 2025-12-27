//! Uproar Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target) {
///     const activeTeam = target.side.activeTeam();
///     const foeActiveTeam = target.side.foe.activeTeam();
///     for (const [i, allyActive] of activeTeam.entries()) {
///         if (allyActive && allyActive.status === 'slp') allyActive.cureStatus();
///         const foeActive = foeActiveTeam[i];
///         if (foeActive && foeActive.status === 'slp') foeActive.cureStatus();
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target) {
    ///     this.add('-start', target, 'Uproar');
    /// }
    pub fn on_start(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onResidual(target) {
    ///     if (target.volatiles['throatchop']) {
    ///         target.removeVolatile('uproar');
    ///         return;
    ///     }
    ///     if (target.lastMove && target.lastMove.id === 'struggle') {
    ///         // don't lock
    ///         delete target.volatiles['uproar'];
    ///     }
    ///     this.add('-start', target, 'Uproar', '[upkeep]');
    /// }
    pub fn on_residual(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onEnd(target) {
    ///     this.add('-end', target, 'Uproar');
    /// }
    pub fn on_end(battle: &mut Battle, target_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onAnySetStatus(status, pokemon) {
    ///     if (status.id === 'slp') {
    ///         if (pokemon === this.effectState.target) {
    ///             this.add('-fail', pokemon, 'slp', '[from] Uproar', '[msg]');
    ///         } else {
    ///             this.add('-fail', pokemon, 'slp', '[from] Uproar');
    ///         }
    ///         return null;
    ///     }
    /// }
    pub fn on_any_set_status(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

}
