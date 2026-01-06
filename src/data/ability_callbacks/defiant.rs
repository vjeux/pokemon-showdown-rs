//! Defiant Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAfterEachBoost(boost, target, source, effect) {
///     if (!source || target.isAlly(source)) {
///         return;
///     }
///     let statsLowered = false;
///     let i: BoostID;
///     for (i in boost) {
///         if (boost[i]! < 0) {
///             statsLowered = true;
///         }
///     }
///     if (statsLowered) {
///         this.boost({ atk: 2 }, target, target, null, false, true);
///     }
/// }
pub fn on_after_each_boost(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!source || target.isAlly(source))
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    if battle.is_ally(target_pos, source_pos) {
        return EventResult::Continue;
    }

    // Check if any stats were lowered
    // The boost parameter contains the boosts that were just applied
    let stats_lowered = battle.current_event.as_ref()
        .and_then(|e| match &e.relay_var {
            Some(crate::event::EventResult::Boost(b)) => Some(b),
            _ => None,
        })
        .map(|b| {
            b.atk < 0 || b.def < 0 || b.spa < 0 || b.spd < 0 || b.spe < 0 || b.accuracy < 0 || b.evasion < 0
        })
        .unwrap_or(false);

    if stats_lowered {
        // this.boost({ atk: 2 }, target, target, null, false, true);
        battle.boost(
            &[("atk", 2)],
            target_pos,
            Some(target_pos),
            None,
            false,
            true,
        );
    }

    EventResult::Continue
}

