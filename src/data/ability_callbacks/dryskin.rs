//! Dry Skin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, Effect, hp_fraction};
use crate::event::EventResult;

/// onTryHit(target, source, move) {
///     if (target !== source && move.type === 'Water') {
///         if (!this.heal(target.baseMaxhp / 4)) {
///             this.add('-immune', target, '[from] ability: Dry Skin');
///         }
///         return null;
///     }
/// }
pub fn on_try_hit(battle: &mut Battle, target_pos: (usize, usize), source_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // if (target !== source && move.type === 'Water')
    if target_pos == source_pos {
        return EventResult::Continue;
    }

    // JavaScript checks move.type (the active move's type, not the dex type)
    // This is important for moves like Soak or type-changing abilities
    let is_water = active_move.map(|m| m.move_type == "Water").unwrap_or(false);
    if is_water {
        // if (!this.heal(target.baseMaxhp / 4))
        let heal_amount = {
            let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            hp_fraction(target.base_maxhp, 4)
        };

        let healed = battle.heal(heal_amount, Some(target_pos), None, None);

        if healed.is_none() || matches!(healed, Some(0)) {
            // this.add('-immune', target, '[from] ability: Dry Skin');
            let target_slot = {
                let target = match battle.pokemon_at(target_pos.0, target_pos.1) {
                    Some(p) => p,
                    None => return EventResult::Continue,
                };
                target.get_slot()
            };

            battle.add(
                "-immune",
                &[
                    crate::battle::Arg::from(target_slot),
                    crate::battle::Arg::from("[from] ability: Dry Skin"),
                ],
            );
        }

        // return null;
        return EventResult::Null;
    }

    EventResult::Continue
}

/// onSourceBasePower(basePower, attacker, defender, move) {
///     if (move.type === 'Fire') {
///         return this.chainModify(1.25);
///     }
/// }
pub fn on_source_base_power(battle: &mut Battle, _base_power: i32, _attacker_pos: (usize, usize), _defender_pos: (usize, usize), active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // JavaScript checks move.type (the active move's type, not the dex type)
    let is_fire = active_move.map(|m| m.move_type == "Fire").unwrap_or(false);
    if is_fire {
        battle.chain_modify(1.25);
    }
    EventResult::Continue
}

/// onWeather(target, source, effect) {
///     if (target.hasItem('utilityumbrella')) return;
///     if (effect.id === 'raindance' || effect.id === 'primordialsea') {
///         this.heal(target.baseMaxhp / 8);
///     } else if (effect.id === 'sunnyday' || effect.id === 'desolateland') {
///         this.damage(target.baseMaxhp / 8, target, target);
///     }
/// }
pub fn on_weather(battle: &mut Battle, weather_id: &str, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
    // Check if target has Utility Umbrella
    let has_umbrella = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_item(battle, &["utilityumbrella"])
    };

    if has_umbrella {
        return EventResult::Continue;
    }

    if weather_id == "raindance" || weather_id == "primordialsea" {
        // Heal 1/8 HP in rain
        let heal_amount = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            hp_fraction(pokemon.base_maxhp, 8)
        };
        battle.heal(heal_amount, Some(pokemon_pos), None, None);
    } else if weather_id == "sunnyday" || weather_id == "desolateland" {
        // Take 1/8 HP damage in sun
        let damage = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            hp_fraction(pokemon.base_maxhp, 8)
        };
        battle.damage(damage, Some(pokemon_pos), Some(pokemon_pos), None, false);
    }

    EventResult::Continue
}

