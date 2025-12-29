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

    // Check if sourceEffect is 'firepledge' or 'grasspledge'
    if let Some(ref source_effect) = active_move.source_effect {
        let source_str = source_effect.as_str();
        if source_str == "firepledge" || source_str == "grasspledge" {
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
pub fn on_prepare_hit(
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
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
    ///     this.add('-sidestart', targetSide, 'Water Pledge');
    /// }
    pub fn on_side_start(battle: &mut Battle) -> EventResult {
        // this.add('-sidestart', targetSide, 'Water Pledge');
        // Following the pattern from gmaxcannonade.rs - access side via current_effect_state
        if let Some(effect_state) = &battle.current_effect_state {
            if let Some(side_index) = effect_state.side {
                let side_id = if side_index == 0 { "p1" } else { "p2" };

                let side_arg = crate::battle::Arg::Str(side_id);
                battle.add("-sidestart", &[side_arg, "Water Pledge".into()]);
            }
        }

        EventResult::Continue
    }

    /// onSideEnd(targetSide) {
    ///     this.add('-sideend', targetSide, 'Water Pledge');
    /// }
    pub fn on_side_end(battle: &mut Battle) -> EventResult {
        // this.add('-sideend', targetSide, 'Water Pledge');
        // Following the pattern from gmaxcannonade.rs - access side via current_effect_state
        if let Some(effect_state) = &battle.current_effect_state {
            if let Some(side_index) = effect_state.side {
                let side_id = if side_index == 0 { "p1" } else { "p2" };

                let side_arg = crate::battle::Arg::Str(side_id);
                battle.add("-sideend", &[side_arg, "Water Pledge".into()]);
            }
        }

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
    pub fn on_modify_move(
        _battle: &mut Battle,
        _pokemon_pos: (usize, usize),
        _target_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
