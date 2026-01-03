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
pub fn on_damage(battle: &mut Battle, _damage: i32, _target_pos: (usize, usize), source_pos: Option<(usize, usize)>, effect_id: Option<&str>) -> EventResult {
    use crate::battle::Arg;

    // if (effect.effectType !== 'Move')
    if let Some(eff_id) = effect_id {
        // Check if effect is a move by looking it up in moves dex
        let is_move = battle.dex.moves().get(eff_id).is_some();

        if !is_move {
            // if (effect.effectType === 'Ability') this.add('-activate', source, 'ability: ' + effect.name);
            // Check if it's an ability
            if let Some(ability_data) = battle.dex.abilities().get(eff_id) {
                if let Some(src_pos) = source_pos {
                    let source_id = {
                        let source = match battle.pokemon_at(src_pos.0, src_pos.1) {
                            Some(p) => p,
                            None => return EventResult::Boolean(false),
                        };
                        let side_id = format!("p{}", source.side_index + 1);
                        if source.is_active {
                            let pos_letter = (b'a' + source.position as u8) as char;
                            format!("{}{}: {}", side_id, pos_letter, source.name)
                        } else {
                            format!("{}: {}", side_id, source.name)
                        }
                    };

                    battle.add("-activate", &[
                        Arg::String(source_id),
                        Arg::String(format!("ability: {}", ability_data.name)),
                    ]);
                }
            }

            // return false;
            return EventResult::Boolean(false);
        }
    }

    EventResult::Continue
}

