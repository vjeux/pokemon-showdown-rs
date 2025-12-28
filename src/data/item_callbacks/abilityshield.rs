//! Ability Shield Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSetAbility(ability, target, source, effect) {
///     if (effect && effect.effectType === 'Ability' && effect.name !== 'Trace') {
///         this.add('-ability', source, effect);
///     }
///     this.add('-block', target, 'item: Ability Shield');
///     return null;
/// }
pub fn on_set_ability(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::{Arg, EffectType};

    // if (effect && effect.effectType === 'Ability' && effect.name !== 'Trace') {
    //     this.add('-ability', source, effect);
    // }
    if let Some(effect_data) = &battle.current_effect_data {
        if effect_data.effect_type == EffectType::Ability && effect_data.name != "trace" {
            if let Some(source_pos) = source_pos {
                if let Some(source) = battle.pokemon_at(source_pos.0, source_pos.1) {
                    battle.add("-ability", &[
                        Arg::Pokemon(source),
                        Arg::String(effect_data.name.clone()),
                    ]);
                }
            }
        }
    }

    // this.add('-block', target, 'item: Ability Shield');
    if let Some(target_pos) = target_pos {
        if let Some(target) = battle.pokemon_at(target_pos.0, target_pos.1) {
            battle.add("-block", &[
                Arg::Pokemon(target),
                Arg::Str("item: Ability Shield"),
            ]);
        }
    }

    // return null;
    EventResult::Boolean(false)
}
