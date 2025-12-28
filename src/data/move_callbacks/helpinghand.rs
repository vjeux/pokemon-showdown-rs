//! Helping Hand Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target) {
///     if (!target.newlySwitched && !this.queue.willMove(target)) return false;
/// }
pub fn on_try_hit(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    target_pos: (usize, usize),
) -> EventResult {
    let target = target_pos;

    // if (!target.newlySwitched && !this.queue.willMove(target)) return false;
    let (newly_switched, will_move) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (
            target_pokemon.newly_switched,
            battle.queue.will_move(target.0, target.1).is_some(),
        )
    };

    if !newly_switched && !will_move {
        return EventResult::NotFail;
    }

    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onStart(target, source) {
    ///     this.effectState.multiplier = 1.5;
    ///     this.add('-singleturn', target, 'Helping Hand', `[of] ${source}`);
    /// }
    pub fn on_start(
        _battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onRestart(target, source) {
    ///     this.effectState.multiplier *= 1.5;
    ///     this.add('-singleturn', target, 'Helping Hand', `[of] ${source}`);
    /// }
    pub fn on_restart(
        _battle: &mut Battle,
        _target_pos: Option<(usize, usize)>,
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onBasePower(basePower) {
    ///     this.debug('Boosting from Helping Hand: ' + this.effectState.multiplier);
    ///     return this.chainModify(this.effectState.multiplier);
    /// }
    pub fn on_base_power(
        _battle: &mut Battle,
        _base_power: i32,
        _pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
