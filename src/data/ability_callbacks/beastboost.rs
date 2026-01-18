//! Beast Boost Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::battle::EffectType;
use crate::event::EventResult;

/// onSourceAfterFaint(length, target, source, effect) {
///     if (effect && effect.effectType === 'Move') {
///         const bestStat = source.getBestStat(true, true);
///         this.boost({ [bestStat]: length }, source);
///     }
/// }
pub fn on_source_after_faint(battle: &mut Battle, _length: i32, _target_pos: Option<(usize, usize)>, source_pos: Option<(usize, usize)>, effect: Option<&Effect>) -> EventResult {
    // Check if effect exists and effectType === 'Move'
    let is_move_effect = effect
        .map(|e| e.effect_type == EffectType::Move)
        .unwrap_or(false);

    if is_move_effect {
        // Effect is a move
        if let Some(src_pos) = source_pos {
            // const bestStat = source.getBestStat(true, true);
            // Calculate best stat directly without borrowing source
            let stats = [
                crate::dex_data::StatID::Atk,
                crate::dex_data::StatID::Def,
                crate::dex_data::StatID::SpA,
                crate::dex_data::StatID::SpD,
                crate::dex_data::StatID::Spe,
            ];

            let mut best_stat = crate::dex_data::StatID::Atk;
            let mut best_value = 0;

            for stat in stats {
                let value = battle.get_pokemon_stat(src_pos, stat, true, true);
                if value > best_value {
                    best_value = value;
                    best_stat = stat;
                }
            }

            // Convert StatID to string for boost
            let stat_name = match best_stat {
                crate::dex_data::StatID::Atk => "atk",
                crate::dex_data::StatID::Def => "def",
                crate::dex_data::StatID::SpA => "spa",
                crate::dex_data::StatID::SpD => "spd",
                crate::dex_data::StatID::Spe => "spe",
                _ => return EventResult::Continue, // Skip HP
            };

            // this.boost({ [bestStat]: length }, source);
            // In practice, length is always 1 for Beast Boost
            battle.boost(&[(stat_name, 1)], src_pos, None, None, false, false);
        }
    }
    EventResult::Continue
}
