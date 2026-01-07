//! Clear Amulet Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryBoost(boost, target, source, effect) {
///     if (source && target === source) return;
///     let showMsg = false;
///     let i: BoostID;
///     for (i in boost) {
///         if (boost[i]! < 0) {
///             delete boost[i];
///             showMsg = true;
///         }
///     }
///     if (showMsg && !(effect as ActiveMove).secondaries && effect.id !== 'octolock') {
///         this.add('-fail', target, 'unboost', '[from] item: Clear Amulet', `[of] ${target}`);
///     }
/// }
pub fn on_try_boost(
    battle: &mut Battle,
    target_pos: (usize, usize),
    boost: &mut crate::dex_data::BoostsTable,
) -> EventResult {
    // Get source from current event
    let source_pos = battle.current_event.as_ref().and_then(|e| e.source);

    // if (source && target === source) return;
    if let Some(src) = source_pos {
        if src == target_pos {
            return EventResult::Continue;
        }
    }

    // let showMsg = false;
    let mut show_msg = false;

    // for (i in boost) {
    //     if (boost[i]! < 0) {
    //         delete boost[i];
    //         showMsg = true;
    //     }
    // }
    if boost.atk < 0 {
        boost.atk = 0;
        show_msg = true;
    }
    if boost.def < 0 {
        boost.def = 0;
        show_msg = true;
    }
    if boost.spa < 0 {
        boost.spa = 0;
        show_msg = true;
    }
    if boost.spd < 0 {
        boost.spd = 0;
        show_msg = true;
    }
    if boost.spe < 0 {
        boost.spe = 0;
        show_msg = true;
    }
    if boost.accuracy < 0 {
        boost.accuracy = 0;
        show_msg = true;
    }
    if boost.evasion < 0 {
        boost.evasion = 0;
        show_msg = true;
    }

    // if (showMsg && !(effect as ActiveMove).secondaries && effect.id !== 'octolock') {
    //     this.add('-fail', target, 'unboost', '[from] item: Clear Amulet', `[of] ${target}`);
    // }
    if show_msg {
        // Check if effect.secondaries exists and is non-empty, or if effect.id is 'octolock'
        let has_secondaries = battle.active_move.as_ref()
            .map(|m| !m.secondaries.is_empty())
            .unwrap_or(false);

        let is_octolock = battle.current_event.as_ref()
            .and_then(|e| e.effect.as_ref())
            .map(|eff| eff.id.as_str() == "octolock")
            .unwrap_or(false);

        // Only show message if no secondaries and not octolock
        if !has_secondaries && !is_octolock {
            let target_slot = {
                let pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                pokemon.get_slot()
            };

            battle.add(
                "-fail",
                &[
                    crate::battle::Arg::from(target_slot.clone()),
                    crate::battle::Arg::from("unboost"),
                    crate::battle::Arg::from("[from] item: Clear Amulet"),
                    crate::battle::Arg::from(format!("[of] {}", target_slot)),
                ],
            );
        }
    }

    EventResult::Continue
}
