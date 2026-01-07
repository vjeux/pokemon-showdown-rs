//! Raindance Condition
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
///     if (source?.hasItem('damprock')) {
///         return 8;
///     }
///     return 5;
/// }
/// ```
pub fn duration_callback(
    battle: &mut Battle,
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    // if (source?.hasItem('damprock'))
    let has_damprock = if let Some(pos) = source_pos {
        let pokemon = match battle.pokemon_at(pos.0, pos.1) {
            Some(p) => p,
            None => return EventResult::Number(5),
        };
        pokemon.has_item(battle, &["damprock"])
    } else {
        false
    };

    if has_damprock {
        // return 8;
        return EventResult::Number(8);
    }

    // return 5;
    EventResult::Number(5)
}

/// onWeatherModifyDamage
/// JavaScript source (data/conditions.ts):
/// ```js
/// onWeatherModifyDamage(damage, attacker, defender, move) {
///     if (defender.hasItem('utilityumbrella')) return;
///     if (move.type === 'Water') {
///         this.debug('rain water boost');
///         return this.chainModify(1.5);
///     }
///     if (move.type === 'Fire') {
///         this.debug('rain fire suppress');
///         return this.chainModify(0.5);
///     }
/// }
/// ```
pub fn on_weather_modify_damage(
    battle: &mut Battle,
    _damage: i32,
    _attacker_pos: Option<(usize, usize)>,
    _defender_pos: Option<(usize, usize)>,
    _move_id: Option<&str>,
) -> EventResult {
    // Get defender position from battle context
    let defender_pos = match battle.active_target {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (defender.hasItem('utilityumbrella')) return;
    let has_utilityumbrella = {
        let pokemon = match battle.pokemon_at(defender_pos.0, defender_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_item(battle, &["utilityumbrella"])
    };

    if has_utilityumbrella {
        return EventResult::Continue;
    }

    // Get move type from active_move
    let move_type = match &battle.active_move {
        Some(m) => m.move_type.as_str(),
        None => return EventResult::Continue,
    };

    // if (move.type === 'Water')
    if move_type == "water" {
        // this.debug('rain water boost');
        // return this.chainModify(1.5);
        battle.chain_modify(1.5); return EventResult::Continue;
    }

    // if (move.type === 'Fire')
    if move_type == "fire" {
        // this.debug('rain fire suppress');
        // return this.chainModify(0.5);
        battle.chain_modify(0.5); return EventResult::Continue;
    }

    EventResult::Continue
}

/// onFieldStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldStart(field, source, effect) {
///     if (effect?.effectType === 'Ability') {
///         if (this.gen <= 5) this.effectState.duration = 0;
///         this.add('-weather', 'RainDance', '[from] ability: ' + effect.name, `[of] ${source}`);
///     } else {
///         this.add('-weather', 'RainDance');
///     }
/// }
/// ```
pub fn on_field_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // if (effect?.effectType === 'Ability')
    let is_ability = battle.effect.as_ref()
        .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
        .is_some();

    if is_ability {
        // if (this.gen <= 5) this.effectState.duration = 0;
        if battle.gen <= 5 {
            battle.field.weather_state.duration = Some(0);
        }

        // this.add('-weather', 'RainDance', '[from] ability: ' + effect.name, `[of] ${source}`);
        let ability_name = battle.effect.as_ref()
            .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
            .map(|ab| ab.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        let source_ident = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon.get_slot()
        };

        battle.add(
            "-weather",
            &[
                Arg::Str("RainDance"),
                Arg::String(format!("[from] ability: {}", ability_name)),
                Arg::String(format!("[of] {}", source_ident)),
            ],
        );
    } else {
        // this.add('-weather', 'RainDance');
        battle.add("-weather", &[Arg::Str("RainDance")]);
    }

    EventResult::Continue
}

/// onFieldResidual
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldResidualOrder: 1,
/// onFieldResidual() {
///     this.add('-weather', 'RainDance', '[upkeep]');
///     this.eachEvent('Weather');
/// }
/// ```
pub fn on_field_residual(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-weather', 'RainDance', '[upkeep]');
    battle.add("-weather", &[Arg::Str("RainDance"), Arg::Str("[upkeep]")]);

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

