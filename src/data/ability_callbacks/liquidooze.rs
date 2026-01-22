//! Liquid Ooze Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;

/// onSourceTryHeal(damage, target, source, effect) {
///     this.debug(`Heal is occurring: ${target} <- ${source} :: ${effect.id}`);
///     const canOoze = ['drain', 'leechseed', 'strengthsap'];
///     if (canOoze.includes(effect.id)) {
///         this.damage(damage);
///         return 0;
///     }
/// }
pub fn on_source_try_heal(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    let effect_id = effect.map(|e| e.id.as_str());
    // Check if the effect is one that can be reversed by Liquid Ooze
    let can_ooze = match effect_id {
        Some("drain") | Some("leechseed") | Some("strengthsap") => true,
        _ => false,
    };

    if can_ooze {
        // Damage the target (the Pokemon trying to heal) instead of healing
        // JS: this.damage(damage) uses the event target by default
        if let Some(target) = target_pos {
            battle.damage(damage, Some(target), None, None, false);
        }
        // Return 0 to prevent healing
        return EventResult::Number(0);
    }

    EventResult::Continue
}

