//! Big Pecks Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onTryBoost(boost, target, source, effect) {
///     if (source && target === source) return;
///     if (boost.def && boost.def < 0) {
///         delete boost.def;
///         if (!(effect as ActiveMove).secondaries && effect.id !== 'octolock') {
///             this.add("-fail", target, "unboost", "Defense", "[from] ability: Big Pecks", `[of] ${target}`);
///         }
///     }
/// }
pub fn on_try_boost(
    battle: &mut Battle,
    boost: Option<&mut crate::dex_data::BoostsTable>, target_pos: (usize, usize), source_pos: Option<(usize, usize)>, _effect: Option<&Effect>,
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

    // if (boost.def && boost.def < 0) {
    if boost.def < 0 {
        // delete boost.def;
        boost.def = 0;

        // if (!(effect as ActiveMove).secondaries && effect.id !== 'octolock') {
        let has_secondaries = battle.active_move.as_ref()
            .map(|m| !m.secondaries.is_empty())
            .unwrap_or(false);

        let is_octolock = battle.event.as_ref()
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
                    crate::battle::Arg::from("Defense"),
                    crate::battle::Arg::from("[from] ability: Big Pecks"),
                    crate::battle::Arg::from(format!("[of] {}", target_slot)),
                ],
            );
        }
    }

    EventResult::Continue
}

