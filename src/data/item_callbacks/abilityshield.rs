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
pub fn on_set_ability(battle: &mut Battle, target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;
    use crate::battle::EffectType;

    // if (effect && effect.effectType === 'Ability' && effect.name !== 'Trace') {
    //     this.add('-ability', source, effect);
    // }
    if let (Some(effect_type), Some(effect_id)) = (battle.current_effect_type(), battle.current_effect_id()) {
        if effect_type == EffectType::Ability && effect_id.as_str() != "trace" {
            if let Some(source_pos) = source_pos {
                let source_str = if let Some(source) = battle.pokemon_at(source_pos.0, source_pos.1) {
                    source.to_string()
                } else {
                    String::new()
                };

                if !source_str.is_empty() {
                    let effect_name = effect_id.to_string();
                    battle.add("-ability", &[
                        Arg::String(source_str),
                        Arg::String(effect_name),
                    ]);
                }
            }
        }
    }

    // this.add('-block', target, 'item: Ability Shield');
    if let Some(target_pos) = target_pos {
        let target_str = if let Some(target) = battle.pokemon_at(target_pos.0, target_pos.1) {
            target.to_string()
        } else {
            String::new()
        };

        if !target_str.is_empty() {
            battle.add("-block", &[
                Arg::String(target_str),
                Arg::Str("item: Ability Shield"),
            ]);
        }
    }

    // return null;
    // EventResult::Null prevents the ability from being changed
    EventResult::Null
}
