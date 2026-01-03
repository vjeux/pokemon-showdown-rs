//! Aroma Veil Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onAllyTryAddVolatile(status, target, source, effect) {
///     if (['attract', 'disable', 'encore', 'healblock', 'taunt', 'torment'].includes(status.id)) {
///         if (effect.effectType === 'Move') {
///             const effectHolder = this.effectState.target;
///             this.add('-block', target, 'ability: Aroma Veil', `[of] ${effectHolder}`);
///         }
///         return null;
///     }
/// }
pub fn on_ally_try_add_volatile(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;

    let status_id = match status {
        Some(s) => s,
        None => return EventResult::Continue,
    };

    let target_pos = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (['attract', 'disable', 'encore', 'healblock', 'taunt', 'torment'].includes(status.id))
    let blocked_statuses = ["attract", "disable", "encore", "healblock", "taunt", "torment"];
    if !blocked_statuses.contains(&status_id) {
        return EventResult::Continue;
    }

    // if (effect.effectType === 'Move')
    let is_from_move = if let Some(effect) = effect_id {
        // Check if the effect is a move by looking it up in the moves dex
        battle.dex.moves().get(effect).is_some()
    } else {
        false
    };

    if is_from_move {
        // const effectHolder = this.effectState.target;
        // this.add('-block', target, 'ability: Aroma Veil', `[of] ${effectHolder}`);
        // For now, we can't get effectState.target easily, but we can show the message without it
        // TODO: Once effectState infrastructure is available, use effectState.target for effect_holder
        let target_slot = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Null,
            };
            target.get_slot()
        };

        battle.add("-block", &[
            Arg::String(target_slot),
            Arg::Str("ability: Aroma Veil"),
            // TODO: Add [of] ${effectHolder} when effectState.target is available
        ]);
    }

    // return null;
    EventResult::Null
}

