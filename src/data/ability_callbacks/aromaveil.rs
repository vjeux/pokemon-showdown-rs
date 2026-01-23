//! Aroma Veil Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::battle::EffectType;
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
pub fn on_ally_try_add_volatile(battle: &mut Battle, status: Option<&str>, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
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
    // Check if the effect type is Move (not just if a move with this ID exists!)
    let is_from_move = if let Some(effect) = effect {
        effect.effect_type == EffectType::Move
    } else {
        false
    };

    if is_from_move {
        // const effectHolder = this.effectState.target;
        let effect_holder_pos = match battle.effect_state.target {
            Some(pos) => pos,
            None => return EventResult::Null,
        };

        // this.add('-block', target, 'ability: Aroma Veil', `[of] ${effectHolder}`);
        let (target_slot, effect_holder_slot) = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Null,
            };
            let effect_holder = match battle.pokemon_at(effect_holder_pos.0, effect_holder_pos.1) {
                Some(p) => p,
                None => return EventResult::Null,
            };
            (target.get_slot(), effect_holder.get_slot())
        };

        battle.add("-block", &[
            Arg::String(target_slot),
            Arg::Str("ability: Aroma Veil"),
            Arg::String(format!("[of] {}", effect_holder_slot)),
        ]);
    }

    // return null;
    EventResult::Null
}

