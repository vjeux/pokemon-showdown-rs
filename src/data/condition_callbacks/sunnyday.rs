//! Sunnyday Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Arg;
use crate::event::EventResult;

/// durationCallback
/// JavaScript source (data/conditions.ts):
/// ```js
/// durationCallback(source, effect) {
///     if (source?.hasItem('heatrock')) {
///         return 8;
///     }
///     return 5;
/// }
/// ```
pub fn duration_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // if (source?.hasItem('heatrock')) { return 8; }
    let has_heat_rock = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Number(5),
        };
        pokemon.has_item(battle, &["heatrock"])
    };

    if has_heat_rock {
        EventResult::Number(8)
    } else {
        EventResult::Number(5)
    }
}

/// onWeatherModifyDamage
/// JavaScript source (data/conditions.ts):
/// ```js
/// onWeatherModifyDamage(damage, attacker, defender, move) {
///     if (move.id === 'hydrosteam' && !attacker.hasItem('utilityumbrella')) {
///         this.debug('Sunny Day Hydro Steam boost');
///         return this.chainModify(1.5);
///     }
///     if (defender.hasItem('utilityumbrella')) return;
///     if (move.type === 'Fire') {
///         this.debug('Sunny Day fire boost');
///         return this.chainModify(1.5);
///     }
///     if (move.type === 'Water') {
///         this.debug('Sunny Day water suppress');
///         return this.chainModify(0.5);
///     }
/// }
/// ```
pub fn on_weather_modify_damage(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // Get the active move
    let (move_id, move_type) = match &battle.active_move {
        Some(m) => (m.id.clone(), m.move_type.clone()),
        None => return EventResult::Continue,
    };

    // Get attacker and defender positions
    let attacker_pos = battle.active_pokemon.unwrap_or(pokemon_pos);
    let defender_pos = battle.active_target.unwrap_or(pokemon_pos);

    // if (move.id === 'hydrosteam' && !attacker.hasItem('utilityumbrella'))
    if move_id.as_str() == "hydrosteam" {
        let has_utility_umbrella = {
            let attacker = match battle.pokemon_at(attacker_pos.0, attacker_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            attacker.has_item(battle, &["utilityumbrella"])
        };

        if !has_utility_umbrella {
            // this.debug('Sunny Day Hydro Steam boost');
            // return this.chainModify(1.5);
            return EventResult::Float(1.5);
        }
    }

    // if (defender.hasItem('utilityumbrella')) return;
    let defender_has_umbrella = {
        let defender = match battle.pokemon_at(defender_pos.0, defender_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        defender.has_item(battle, &["utilityumbrella"])
    };

    if defender_has_umbrella {
        return EventResult::Continue;
    }

    // if (move.type === 'Fire')
    if move_type == "Fire" {
        // this.debug('Sunny Day fire boost');
        // return this.chainModify(1.5);
        return EventResult::Float(1.5);
    }

    // if (move.type === 'Water')
    if move_type == "Water" {
        // this.debug('Sunny Day water suppress');
        // return this.chainModify(0.5);
        return EventResult::Float(0.5);
    }

    EventResult::Continue
}

/// onFieldStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldStart(battle, source, effect) {
///     if (effect?.effectType === 'Ability') {
///         if (this.gen <= 5) this.effectState.duration = 0;
///         this.add('-weather', 'SunnyDay', '[from] ability: ' + effect.name, `[of] ${source}`);
///     } else {
///         this.add('-weather', 'SunnyDay');
///     }
/// }
/// ```
pub fn on_field_start(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // Check if the effect is an ability
    let is_ability = battle.effect.as_ref()
        .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
        .is_some();

    if is_ability {
        // if (this.gen <= 5) this.effectState.duration = 0;
        if battle.gen <= 5 {
            // Set weather duration to 0 (infinite)
            battle.field.weather_state.duration = Some(0);
        }

        // this.add('-weather', 'SunnyDay', '[from] ability: ' + effect.name, `[of] ${source}`);
        let ability_name = battle.effect.as_ref()
            .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
            .map(|ab| ab.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        // Get source Pokemon ident
        let source_ident = battle.field.weather_state.source
            .and_then(|(side_idx, poke_idx)| battle.pokemon_at(side_idx, poke_idx))
            .map(|p| p.get_slot())
            .unwrap_or_else(|| String::new());

        battle.add(
            "-weather",
            &[
                Arg::Str("SunnyDay"),
                Arg::String(format!("[from] ability: {}", ability_name)),
                Arg::String(format!("[of] {}", source_ident)),
            ],
        );
    } else {
        // this.add('-weather', 'SunnyDay');
        battle.add("-weather", &[Arg::Str("SunnyDay")]);
    }

    EventResult::Continue
}

/// onImmunity
/// JavaScript source (data/conditions.ts):
/// ```js
/// onImmunity(type, pokemon) {
///     if (pokemon.hasItem('utilityumbrella')) return;
///     if (type === 'frz') return false;
/// }
/// ```
pub fn on_immunity(
    battle: &mut Battle,
    immunity_type: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // if (pokemon.hasItem('utilityumbrella')) return;
    let has_utility_umbrella = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_item(battle, &["utilityumbrella"])
    };

    if has_utility_umbrella {
        return EventResult::Continue;
    }

    // if (type === 'frz') return false;
    if immunity_type == "frz" {
        return EventResult::Boolean(false);
    }

    EventResult::Continue
}

/// onFieldResidual
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldResidual() {
///     this.add('-weather', 'SunnyDay', '[upkeep]');
///     this.eachEvent('Weather');
/// }
/// ```
pub fn on_field_residual(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-weather', 'SunnyDay', '[upkeep]');
    battle.add("-weather", &[Arg::Str("SunnyDay"), Arg::Str("[upkeep]")]);

    // this.eachEvent('Weather');
    battle.each_event("Weather", None, None);

    EventResult::Continue
}

/// onFieldEnd
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldEnd() {
///     this.add('-weather', 'none');
/// }
/// ```
pub fn on_field_end(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-weather', 'none');
    battle.add("-weather", &[Arg::Str("none")]);

    EventResult::Continue
}

