//! Magic Guard Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onDamage(damage, target, source, effect) {
///     if (effect.effectType !== 'Move') {
///         if (effect.effectType === 'Ability') this.add('-activate', source, 'ability: ' + effect.name);
///         return false;
///     }
/// }
pub fn on_damage(battle: &mut Battle, _damage: i32, _target_pos: (usize, usize), source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;
    use crate::battle::EffectType;

    // if (effect.effectType !== 'Move')
    // Check the effect_type from the EVENT's effect (source_effect passed to run_event), not self.effect
    // In JavaScript: this.event.effect is the effect that caused this damage
    // For moves, this will be EffectType::Move; for entry hazards like Spikes, it will be EffectType::SideCondition
    let effect_type = battle.event.as_ref()
        .and_then(|e| e.effect.as_ref())
        .map(|eff| eff.effect_type);

    match effect_type {
        Some(EffectType::Move) | Some(EffectType::MoveSelf) => {
            // Effect is a Move, Magic Guard doesn't block it
            EventResult::Continue
        }
        Some(EffectType::Ability) => {
            // if (effect.effectType === 'Ability') this.add('-activate', source, 'ability: ' + effect.name);
            if let Some(src_pos) = source_pos {
                let (source_id, ability_name) = {
                    let source = match battle.pokemon_at(src_pos.0, src_pos.1) {
                        Some(p) => p,
                        None => return EventResult::Boolean(false),
                    };
                    let ability_name = battle.event.as_ref()
                        .and_then(|e| e.effect.as_ref())
                        .map(|eff| eff.name.clone())
                        .unwrap_or_default();
                    let side_id = format!("p{}", source.side_index + 1);
                    let source_id = if source.is_active {
                        let pos_letter = (b'a' + source.position as u8) as char;
                        format!("{}{}: {}", side_id, pos_letter, source.name)
                    } else {
                        format!("{}: {}", side_id, source.name)
                    };
                    (source_id, ability_name)
                };

                battle.add("-activate", &[
                    Arg::String(source_id),
                    Arg::String(format!("ability: {}", ability_name)),
                ]);
            }

            // return false;
            EventResult::Boolean(false)
        }
        Some(_) | None => {
            // Effect is not a Move (could be SideCondition like Spikes, Weather, Status, etc.)
            // Magic Guard blocks this damage
            // return false;
            EventResult::Boolean(false)
        }
    }
}

