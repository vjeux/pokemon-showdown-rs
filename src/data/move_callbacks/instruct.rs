//! Instruct Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target, source) {
///     if (!target.lastMove || target.volatiles['dynamax']) return false;
///     const lastMove = target.lastMove;
///     const moveSlot = target.getMoveData(lastMove.id);
///     if (
///         lastMove.flags['failinstruct'] || lastMove.isZ || lastMove.isMax ||
///         lastMove.flags['charge'] || lastMove.flags['recharge'] ||
///         target.volatiles['beakblast'] || target.volatiles['focuspunch'] || target.volatiles['shelltrap'] ||
///         (moveSlot && moveSlot.pp <= 0)
///     ) {
///         return false;
///     }
///     this.add('-singleturn', target, 'move: Instruct', `[of] ${source}`);
///     this.queue.prioritizeAction(this.queue.resolveAction({
///         choice: 'move',
///         pokemon: target,
///         moveid: target.lastMove.id,
///         targetLoc: target.lastMoveTargetLoc!,
///     })[0] as MoveAction);
/// }
pub fn on_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

