//! Leaf Guard Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onSetStatus(status, target, source, effect) {
///     if (['sunnyday', 'desolateland'].includes(target.effectiveWeather())) {
///         if ((effect as Move)?.status) {
///             this.add('-immune', target, '[from] ability: Leaf Guard');
///         }
///         return false;
///     }
/// }
pub fn on_set_status(battle: &mut Battle, _status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;

    // if (['sunnyday', 'desolateland'].includes(target.effectiveWeather()))
    let effective_weather = {
        let field_weather = battle.field.effective_weather();
        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target.effective_weather(battle, field_weather.as_str())
    };

    if effective_weather == "sunnyday" || effective_weather == "desolateland" {
        // if ((effect as Move)?.status)
        // Check if effect is a move with status
        if let Some(eff_id) = effect_id {
            if let Some(move_data) = battle.dex.moves().get(eff_id) {
                if move_data.status.is_some() {
                    let target_id = {
                        let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                            Some(p) => p,
                            None => return EventResult::Boolean(false),
                        };
                        let side_id = format!("p{}", target.side_index + 1);
                        if target.is_active {
                            let pos_letter = (b'a' + target.position as u8) as char;
                            format!("{}{}: {}", side_id, pos_letter, target.name)
                        } else {
                            format!("{}: {}", side_id, target.name)
                        }
                    };

                    battle.add("-immune", &[
                        Arg::String(target_id),
                        Arg::Str("[from] ability: Leaf Guard"),
                    ]);
                }
            }
        }

        // return false;
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onTryAddVolatile(status, target) {
///     if (status.id === 'yawn' && ['sunnyday', 'desolateland'].includes(target.effectiveWeather())) {
///         this.add('-immune', target, '[from] ability: Leaf Guard');
///         return null;
///     }
/// }
pub fn on_try_add_volatile(battle: &mut Battle, status_id: &str, target_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;

    // if (status.id === 'yawn' && ['sunnyday', 'desolateland'].includes(target.effectiveWeather()))
    if status_id == "yawn" {
        let effective_weather = {
            let field_weather = battle.field.effective_weather();
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            target.effective_weather(battle, field_weather.as_str())
        };

        if effective_weather == "sunnyday" || effective_weather == "desolateland" {
            let target_id = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Null,
                };
                let side_id = format!("p{}", target.side_index + 1);
                if target.is_active {
                    let pos_letter = (b'a' + target.position as u8) as char;
                    format!("{}{}: {}", side_id, pos_letter, target.name)
                } else {
                    format!("{}: {}", side_id, target.name)
                }
            };

            battle.add("-immune", &[
                Arg::String(target_id),
                Arg::Str("[from] ability: Leaf Guard"),
            ]);

            // return null;
            return EventResult::Null;
        }
    }

    EventResult::Continue
}

