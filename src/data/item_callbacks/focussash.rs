//! Focus Sash Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;
use crate::Pokemon;

/// onDamage(damage, target, source, effect) {
///     if (target.hp === target.maxhp && damage >= target.hp && effect && effect.effectType === 'Move') {
///         if (target.useItem()) {
///             return target.hp - 1;
///         }
///     }
/// }
pub fn on_damage(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>, effect_type: Option<&str>) -> EventResult {
    // if (target.hp === target.maxhp && damage >= target.hp && effect && effect.effectType === 'Move')
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let (target_hp, target_maxhp) = if let Some(pokemon) = battle.pokemon_at(target.0, target.1) {
        (pokemon.hp, pokemon.maxhp)
    } else {
        return EventResult::Continue;
    };

    // target.hp === target.maxhp
    if target_hp != target_maxhp {
        return EventResult::Continue;
    }

    // damage >= target.hp
    if damage < target_hp {
        return EventResult::Continue;
    }

    // effect && effect.effectType === 'Move'
    // Use the effect_type passed from the Effect struct directly
    // This is correct because the Effect struct preserves the actual effect type
    // (e.g., gmaxvolcalith side condition has type "Condition", not "Move")
    if effect_type == Some("Move") {
        // if (target.useItem())
        let used_item = if let Some(_pokemon) = battle.pokemon_at_mut(target.0, target.1) {
            Pokemon::use_item(battle, target, None, None).is_some()
        } else {
            false
        };

        if used_item {
            // return target.hp - 1;
            return EventResult::Number(target_hp - 1);
        }
    }

    EventResult::Continue
}
