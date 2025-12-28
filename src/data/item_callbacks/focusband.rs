//! Focus Band Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::battle::Arg;

/// onDamage(damage, target, source, effect) {
///     if (this.randomChance(1, 10) && damage >= target.hp && effect && effect.effectType === 'Move') {
///         this.add("-activate", target, "item: Focus Band");
///         return target.hp - 1;
///     }
/// }
pub fn on_damage(battle: &mut Battle, damage: i32, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    // if (this.randomChance(1, 10) && damage >= target.hp && effect && effect.effectType === 'Move')

    // this.randomChance(1, 10)
    if !battle.random_chance(1, 10) {
        return EventResult::Continue;
    }

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // damage >= target.hp
    let target_hp = if let Some(pokemon) = battle.pokemon_at(target.0, target.1) {
        pokemon.hp
    } else {
        return EventResult::Continue;
    };

    if damage < target_hp {
        return EventResult::Continue;
    }

    // effect && effect.effectType === 'Move'
    if let Some(eff_id) = effect_id {
        let effect_type = battle.get_effect_type(&eff_id.into());
        if effect_type == "Move" {
            // this.add("-activate", target, "item: Focus Band");
            // Get the pokemon name first to avoid borrow checker issues
            let target_str = if let Some(target_pokemon) = battle.pokemon_at(target.0, target.1) {
                target_pokemon.to_string()
            } else {
                String::new()
            };

            if !target_str.is_empty() {
                battle.add("-activate", &[
                    Arg::String(target_str),
                    Arg::Str("item: Focus Band"),
                ]);
            }
            // return target.hp - 1;
            return EventResult::Number(target_hp - 1);
        }
    }

    EventResult::Continue
}
