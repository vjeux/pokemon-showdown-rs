//! Inner Focus Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryAddVolatile(status, pokemon) {
///     if (status.id === 'flinch') return null;
/// }
pub fn on_try_add_volatile(_battle: &mut Battle, status_id: &str, _target_pos: (usize, usize), __source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    if status_id == "flinch" {
        return EventResult::Null;
    }
    EventResult::Continue
}

/// onTryBoost(boost, target, source, effect) {
///     if (effect.name === 'Intimidate' && boost.atk) {
///         delete boost.atk;
///         this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Inner Focus', `[of] ${target}`);
///     }
/// }
pub fn on_try_boost(
    battle: &mut Battle,
    boost: Option<&mut crate::dex_data::BoostsTable>, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>,
) -> EventResult {
    let is_intimidate = battle.current_event.as_ref()
        .and_then(|e| e.effect.as_ref())
        .map(|id| id.as_str() == "intimidate")
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
                crate::battle::Arg::from("[from] ability: Inner Focus"),
                crate::battle::Arg::from(format!("[of] {}", target_slot)),
            ],
        );
    }

    EventResult::Continue
}

