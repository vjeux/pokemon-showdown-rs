//! Solar Power Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::{Battle, hp_fraction};
use crate::event::EventResult;

/// onModifySpA(spa, pokemon) {
///     if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather())) {
///         return this.chainModify(1.5);
///     }
/// }
pub fn on_modify_sp_a(battle: &mut Battle, _spa: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Get field weather
    let field_weather = battle.effective_weather();

    // Get pokemon and check effective weather
    let pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    let eff_weather = pokemon.effective_weather(battle, field_weather.as_str());

    if eff_weather == "sunnyday" || eff_weather == "desolateland" {
        battle.chain_modify(1.5); return EventResult::Continue;
    }

    EventResult::Continue
}

/// onWeather(target, source, effect) {
///     if (target.hasItem('utilityumbrella')) return;
///     if (effect.id === 'sunnyday' || effect.id === 'desolateland') {
///         this.damage(target.baseMaxhp / 8, target, target);
///     }
/// }
pub fn on_weather(battle: &mut Battle, weather_id: &str, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
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

    if weather_id == "sunnyday" || weather_id == "desolateland" {
        // Get base max HP and calculate damage
        let damage = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            hp_fraction(pokemon.base_maxhp, 8)
        };

        // Damage the pokemon
        battle.damage(damage, Some(pokemon_pos), Some(pokemon_pos), None, false);
    }

    EventResult::Continue
}

