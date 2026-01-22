//! Defiant Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
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
pub fn on_after_each_boost(battle: &mut Battle, boost: &crate::dex_data::BoostsTable, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
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
    let stats_lowered = boost.atk < 0 || boost.def < 0 || boost.spa < 0 || boost.spd < 0 || boost.spe < 0 || boost.accuracy < 0 || boost.evasion < 0;

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

