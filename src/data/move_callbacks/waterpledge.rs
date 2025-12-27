//! Water Pledge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(target, source, move) {
///     if (['firepledge', 'grasspledge'].includes(move.sourceEffect)) {
///         this.add('-combine');
///         return 150;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onPrepareHit(target, source, move) {
///     for (const action of this.queue) {
///         if (action.choice !== 'move') continue;
///         const otherMove = action.move;
///         const otherMoveUser = action.pokemon;
///         if (
///             !otherMove || !action.pokemon || !otherMoveUser.isActive ||
///             otherMoveUser.fainted || action.maxMove || action.zmove
///         ) {
///             continue;
///         }
///         if (otherMoveUser.isAlly(source) && ['firepledge', 'grasspledge'].includes(otherMove.id)) {
///             this.queue.prioritizeAction(action, move);
///             this.add('-waiting', source, otherMoveUser);
///             return null;
///         }
///     }
/// }
pub fn on_prepare_hit(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyMove(move) {
///     if (move.sourceEffect === 'grasspledge') {
///         move.type = 'Grass';
///         move.forceSTAB = true;
///         move.sideCondition = 'grasspledge';
///     }
///     if (move.sourceEffect === 'firepledge') {
///         move.type = 'Water';
///         move.forceSTAB = true;
///         move.self = { sideCondition: 'waterpledge' };
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, move_id: &str) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onSideStart(targetSide) {
    ///     this.add('-sidestart', targetSide, 'Water Pledge');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onSideEnd(targetSide) {
    ///     this.add('-sideend', targetSide, 'Water Pledge');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onModifyMove(move, pokemon) {
    ///     if (move.secondaries && move.id !== 'secretpower') {
    ///         this.debug('doubling secondary chance');
    ///         for (const secondary of move.secondaries) {
    ///             if (pokemon.hasAbility('serenegrace') && secondary.volatileStatus === 'flinch') continue;
    ///             if (secondary.chance) secondary.chance *= 2;
    ///         }
    ///         if (move.self?.chance) move.self.chance *= 2;
    ///     }
    /// }
    pub fn on_modify_move(battle: &mut Battle, move_id: &str, pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
