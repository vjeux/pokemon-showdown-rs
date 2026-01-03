//! Serene Grace Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
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
pub fn on_modify_move(battle: &mut Battle, _move_id: &str) -> EventResult {
    // if (move.secondaries)
    let has_secondaries = if let Some(ref active_move) = battle.active_move {
        !active_move.secondaries.is_empty()
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
    if let Some(ref mut active_move) = battle.active_move {
        for secondary in &mut active_move.secondaries {
            if let Some(ref mut chance) = secondary.chance {
                *chance *= 2;
            }
        }
    }

    // if (move.self?.chance) move.self.chance *= 2;
    // TODO: ActiveMove doesn't have a self field yet
    // This would be for self-inflicted secondary effects (like Superpower lowering own stats)
    // For now, skip this part as the infrastructure doesn't exist

    EventResult::Continue
}

