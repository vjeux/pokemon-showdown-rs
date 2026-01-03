//! Rattled Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamagingHit(damage, target, source, move) {
///     if (['Dark', 'Bug', 'Ghost'].includes(move.type)) {
///         this.boost({ spe: 1 });
///     }
/// }
pub fn on_damaging_hit(battle: &mut Battle, _damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, move_id: &str) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (['Dark', 'Bug', 'Ghost'].includes(move.type))
    if let Some(move_data) = battle.dex.moves().get(move_id) {
        if move_data.move_type == "Dark" || move_data.move_type == "Bug" || move_data.move_type == "Ghost" {
            // this.boost({ spe: 1 });
            battle.boost(
                &[("spe", 1)],
                target_pos,
                Some(target_pos),
                None,
                false,
                false,
            );
        }
    }

    EventResult::Continue
}

/// onAfterBoost(boost, target, source, effect) {
///     if (effect?.name === 'Intimidate' && boost.atk) {
///         this.boost({ spe: 1 });
///     }
/// }
pub fn on_after_boost(battle: &mut Battle, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (effect?.name === 'Intimidate')
    let is_intimidate = battle.current_event.as_ref()
        .and_then(|e| e.effect.as_ref())
        .map(|id| id.as_str() == "intimidate")
        .unwrap_or(false);

    if !is_intimidate {
        return EventResult::Continue;
    }

    // Check if atk boost exists (boost.atk is present)
    // The boost parameter contains the boosts that were just applied
    // In JS, we check if boost.atk exists. In Rust, we need to check the relay_var_boost
    let has_atk_boost = battle.current_event.as_ref()
        .and_then(|e| e.relay_var_boost.as_ref())
        .map(|b| b.atk != 0)
        .unwrap_or(false);

    if has_atk_boost {
        // this.boost({ spe: 1 });
        battle.boost(
            &[("spe", 1)],
            target_pos,
            Some(target_pos),
            None,
            false,
            false,
        );
    }

    EventResult::Continue
}

