//! Serene Grace Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onModifyMove(move) {
///     if (move.secondaries) {
///         this.debug('doubling secondary chance');
///         for (const secondary of move.secondaries) {
///             if (secondary.chance) secondary.chance *= 2;
///         }
///     }
///     if (move.self?.chance) move.self.chance *= 2;
/// }
pub fn on_modify_move(_battle: &mut Battle, active_move: Option<&mut crate::battle_actions::ActiveMove>, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (move.secondaries)
    // NOTE: Must check the passed-in active_move parameter, NOT battle.active_move
    // During ModifyMove dispatch, battle.active_move is temporarily taken out (None)
    let has_secondaries = if let Some(ref m) = active_move {
        !m.secondaries.is_empty()
    } else {
        false
    };

    if !has_secondaries {
        return EventResult::Continue;
    }

    // this.debug('doubling secondary chance');
    // for (const secondary of move.secondaries) {
    //     if (secondary.chance) secondary.chance *= 2;
    // }
    if let Some(active_move) = active_move {
        for secondary in &mut active_move.secondaries {
            if let Some(ref mut chance) = secondary.chance {
                *chance *= 2;
            }
        }

        // if (move.self?.chance) move.self.chance *= 2;
        if let Some(ref mut self_effect) = active_move.self_effect {
            if let Some(ref mut chance) = self_effect.chance {
                *chance *= 2;
            }
        }
    }

    EventResult::Continue
}

