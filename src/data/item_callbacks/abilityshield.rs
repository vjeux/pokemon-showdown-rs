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
    use crate::battle::Arg;
    use crate::event_system::EffectType;

    // if (effect && effect.effectType === 'Ability' && effect.name !== 'Trace') {
    //     this.add('-ability', source, effect);
    // }
    if let Some(effect_data) = &battle.current_effect_data {
        if effect_data.effect_type == EffectType::Ability && effect_data.name != "trace" {
            if let Some(source_pos) = source_pos {
                let source_str = if let Some(source) = battle.pokemon_at(source_pos.0, source_pos.1) {
                    source.to_string()
                } else {
                    String::new()
                };

                if !source_str.is_empty() {
                    let effect_name = effect_data.name.clone();
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
    EventResult::Boolean(false)
}
