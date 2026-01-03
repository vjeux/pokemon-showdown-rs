//! Keen Eye Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryBoost(boost, target, source, effect) {
///     if (source && target === source) return;
///     if (boost.accuracy && boost.accuracy < 0) {
///         delete boost.accuracy;
///         if (!(effect as ActiveMove).secondaries) {
///             this.add("-fail", target, "unboost", "accuracy", "[from] ability: Keen Eye", `[of] ${target}`);
///         }
///     }
/// }
pub fn on_try_boost(
    battle: &mut Battle,
    target_pos: (usize, usize),
    boost: Option<&mut crate::dex_data::BoostsTable>,
) -> EventResult {
    // Get source from current event
    let source_pos = battle.current_event.as_ref().and_then(|e| e.source);

    // if (source && target === source) return;
    if let Some(src) = source_pos {
        if src == target_pos {
            return EventResult::Continue;
        }
    }

    // Check if we have a boost table
    let boost = match boost {
        Some(b) => b,
        None => return EventResult::Continue,
    };

    // if (boost.accuracy && boost.accuracy < 0) {
    if boost.accuracy < 0 {
        // delete boost.accuracy;
        boost.accuracy = 0;

        // if (!(effect as ActiveMove).secondaries) {
        let has_secondaries = battle.active_move.as_ref()
            .map(|m| !m.secondaries.is_empty())
            .unwrap_or(false);

        // Only show message if no secondaries
        if !has_secondaries {
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
                    crate::battle::Arg::from("accuracy"),
                    crate::battle::Arg::from("[from] ability: Keen Eye"),
                    crate::battle::Arg::from(format!("[of] {}", target_slot)),
                ],
            );
        }
    }

    EventResult::Continue
}

/// onModifyMove(move) {
///     move.ignoreEvasion = true;
/// }
pub fn on_modify_move(battle: &mut Battle, _move_id: &str) -> EventResult {
    // move.ignoreEvasion = true;
    if let Some(ref mut active_move) = battle.active_move {
        active_move.ignore_evasion = true;
    }

    EventResult::Continue
}

