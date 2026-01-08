//! Scrappy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move) {
///     if (!move.ignoreImmunity) move.ignoreImmunity = {};
///     if (move.ignoreImmunity !== true) {
///         move.ignoreImmunity['Fighting'] = true;
///         move.ignoreImmunity['Normal'] = true;
///     }
/// }
pub fn on_modify_move(battle: &mut Battle, _active_move: Option<&crate::battle_actions::ActiveMove>, _source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    if let Some(ref mut active_move) = battle.active_move {
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

/// onTryBoost(boost, target, source, effect) {
///     if (effect.name === 'Intimidate' && boost.atk) {
///         delete boost.atk;
///         this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Scrappy', `[of] ${target}`);
///     }
/// }
pub fn on_try_boost(
    battle: &mut Battle,
    boost: Option<&mut crate::dex_data::BoostsTable>, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>,
) -> EventResult {
    let is_intimidate = battle.event.as_ref()
        .and_then(|e| e.effect.as_ref())
        .map(|id| id.id.as_str() == "intimidate")
        .unwrap_or(false);

    if !is_intimidate {
        return EventResult::Continue;
    }

    // Check if we have a boost table
    let boost = match boost {
        Some(b) => b,
        None => return EventResult::Continue,
    };

    // if (boost.atk) {
    if boost.atk != 0 {
        // delete boost.atk;
        boost.atk = 0;

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
                crate::battle::Arg::from("Attack"),
                crate::battle::Arg::from("[from] ability: Scrappy"),
                crate::battle::Arg::from(format!("[of] {}", target_slot)),
            ],
        );
    }

    EventResult::Continue
}

