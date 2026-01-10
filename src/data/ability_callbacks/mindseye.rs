//! Mind's Eye Ability
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
///             this.add("-fail", target, "unboost", "accuracy", "[from] ability: Mind's Eye", `[of] ${target}`);
///         }
///     }
/// }
pub fn on_try_boost(
    battle: &mut Battle,
    boost: Option<&mut crate::dex_data::BoostsTable>, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, _effect_id: Option<&str>,
) -> EventResult {
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
                    crate::battle::Arg::from("[from] ability: Mind's Eye"),
                    crate::battle::Arg::from(format!("[of] {}", target_slot)),
                ],
            );
        }
    }

    EventResult::Continue
}

/// onModifyMove(move) {
///     move.ignoreEvasion = true;
///     if (!move.ignoreImmunity) move.ignoreImmunity = {};
///     if (move.ignoreImmunity !== true) {
///         move.ignoreImmunity['Fighting'] = true;
///         move.ignoreImmunity['Normal'] = true;
///     }
/// }
pub fn on_modify_move(_battle: &mut Battle, active_move: Option<&mut crate::battle_actions::ActiveMove>, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    if let Some(active_move) = active_move {
        // move.ignoreEvasion = true;
        active_move.ignore_evasion = true;

        // if (!move.ignoreImmunity) move.ignoreImmunity = {};
        // if (move.ignoreImmunity !== true)
        match &mut active_move.ignore_immunity {
            Some(crate::battle_actions::IgnoreImmunity::All) => {
                // Already set to true (All), don't override
            }
            Some(crate::battle_actions::IgnoreImmunity::Specific(ref mut map)) => {
                // Add to existing map
                map.insert("Fighting".to_string(), true);
                map.insert("Normal".to_string(), true);
            }
            None => {
                // Create new map with Fighting and Normal
                let mut map = std::collections::HashMap::new();
                map.insert("Fighting".to_string(), true);
                map.insert("Normal".to_string(), true);
                active_move.ignore_immunity = Some(crate::battle_actions::IgnoreImmunity::Specific(map));
            }
        }
    }

    EventResult::Continue
}

