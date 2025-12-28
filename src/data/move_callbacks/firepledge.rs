//! Fire Pledge Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(target, source, move) {
///     if (['grasspledge', 'waterpledge'].includes(move.sourceEffect)) {
///         this.add('-combine');
///         return 150;
///     }
///     return move.basePower;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the active move
    let active_move = match &battle.active_move {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // Check if sourceEffect is 'grasspledge' or 'waterpledge'
    if let Some(ref source_effect) = active_move.source_effect {
        let source_str = source_effect.as_str();
        if source_str == "grasspledge" || source_str == "waterpledge" {
            // Note: JS has this.add('-combine') which we don't have infrastructure for yet
            // this.add('-combine');
            return EventResult::Number(150);
        }
    }

    // Get the move data for base power
    let move_data = match battle.dex.get_move_by_id(&active_move.id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    EventResult::Number(move_data.base_power)
}

/// onPrepareHit(target, source, move) {
///     for (const action of this.queue.list as MoveAction[]) {
///         if (
///             !action.move || !action.pokemon?.isActive ||
///             action.pokemon.fainted || action.maxMove || action.zmove
///         ) {
///             continue;
///         }
///         if (action.pokemon.isAlly(source) && ['grasspledge', 'waterpledge'].includes(action.move.id)) {
///             this.queue.prioritizeAction(action, move);
///             this.add('-waiting', source, action.pokemon);
///             return null;
///         }
///     }
/// }
pub fn on_prepare_hit(
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyMove(move) {
///     if (move.sourceEffect === 'waterpledge') {
///         move.type = 'Water';
///         move.forceSTAB = true;
///         move.self = { sideCondition: 'waterpledge' };
///     }
///     if (move.sourceEffect === 'grasspledge') {
///         move.type = 'Fire';
///         move.forceSTAB = true;
///         move.sideCondition = 'firepledge';
///     }
/// }
pub fn on_modify_move(
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

pub mod condition {
    use super::*;

    /// onSideStart(targetSide) {
    ///     this.add('-sidestart', targetSide, 'Fire Pledge');
    /// }
    pub fn on_side_start(_battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onResidual(pokemon) {
    ///     if (!pokemon.hasType('Fire')) this.damage(pokemon.baseMaxhp / 8, pokemon);
    /// }
    pub fn on_residual(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }

    /// onSideEnd(targetSide) {
    ///     this.add('-sideend', targetSide, 'Fire Pledge');
    /// }
    pub fn on_side_end(_battle: &mut Battle) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
