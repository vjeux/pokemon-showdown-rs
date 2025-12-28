//! Wide Guard Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTry() {
///     return !!this.queue.willAct();
/// }
pub fn on_try(_battle: &mut Battle, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onHitSide(side, source) {
///     source.addVolatile('stall');
/// }
pub fn on_hit_side(_battle: &mut Battle, _source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onSideStart(target, source) {
    ///     this.add('-singleturn', source, 'Wide Guard');
    /// }
    pub fn on_side_start(_battle: &mut Battle, _target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onTryHit(target, source, move) {
    ///     // Wide Guard blocks all spread moves
    ///     if (move?.target !== 'allAdjacent' && move.target !== 'allAdjacentFoes') {
    ///         return;
    ///     }
    ///     if (move.isZ || move.isMax) {
    ///         if (['gmaxoneblow', 'gmaxrapidflow'].includes(move.id)) return;
    ///         target.getMoveHitData(move).zBrokeProtect = true;
    ///         return;
    ///     }
    ///     this.add('-activate', target, 'move: Wide Guard');
    ///     const lockedmove = source.getVolatile('lockedmove');
    ///     if (lockedmove) {
    ///         // Outrage counter is reset
    ///         if (source.volatiles['lockedmove'].duration === 2) {
    ///             delete source.volatiles['lockedmove'];
    ///         }
    ///     }
    ///     return this.NOT_FAIL;
    /// }
    pub fn on_try_hit(_battle: &mut Battle, _source_pos: (usize, usize), _target_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
