//! Orichalcum Pulse Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     if (this.field.setWeather('sunnyday')) {
///         this.add('-activate', pokemon, 'Orichalcum Pulse', '[source]');
///     } else if (this.field.isWeather('sunnyday')) {
///         this.add('-activate', pokemon, 'ability: Orichalcum Pulse');
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::ID;

    // Get pokemon ident for logging
    let pokemon_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    let current_weather = battle.field.weather.clone();

    // Try to set weather to sunny day
    if battle.set_weather(ID::new("sunnyday"), None, None) == Some(true) {
        battle.add(
            "-weather",
            &["SunnyDay".into(), "[from] ability: Orichalcum Pulse".into(), format!("[of] {}", pokemon_ident).into()]
        );
        battle.add(
            "-activate",
            &[pokemon_ident.as_str().into(), "Orichalcum Pulse".into(), "[source]".into()]
        );
    } else if current_weather.as_str() == "sunnyday" {
        battle.add(
            "-activate",
            &[pokemon_ident.as_str().into(), "ability: Orichalcum Pulse".into()]
        );
    }

    EventResult::Continue
}

/// onModifyAtk(atk, pokemon) {
///     if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather())) {
///         this.debug('Orichalcum boost');
///         return this.chainModify([5461, 4096]);
///     }
/// }
pub fn on_modify_atk(battle: &mut Battle, _atk: i32, attacker_pos: (usize, usize), _defender_pos: (usize, usize), _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
    // Check if the attacker is in sunny day or desolate land weather
    let effective_weather = {
        let pokemon = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        let field_weather = battle.field.weather.as_str();
        pokemon.effective_weather(battle, field_weather)
    };

    eprintln!("[ORICHALCUM] effective_weather={}", effective_weather);

    if effective_weather == "sunnyday" || effective_weather == "desolateland" {
        battle.debug("Orichalcum boost");
        // Call chain_modify_fraction to set the modifier, then return Continue
        // The event system will apply the modifier automatically
        battle.chain_modify_fraction(5461, 4096);
        eprintln!("[ORICHALCUM] Applied boost!");
        return EventResult::Continue;
    }

    EventResult::Continue
}

