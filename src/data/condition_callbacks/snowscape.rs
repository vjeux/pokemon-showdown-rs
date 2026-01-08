//! Snowscape Condition
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
///     if (source?.hasItem('icyrock')) {
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
    // if (source?.hasItem('icyrock')) { return 8; }
    let has_icy_rock = if let Some(pos) = source_pos {
        let pokemon = match battle.pokemon_at(pos.0, pos.1) {
            Some(p) => p,
            None => return EventResult::Number(5),
        };
        pokemon.has_item(battle, &["icyrock"])
    } else {
        false
    };

    if has_icy_rock {
        EventResult::Number(8)
    } else {
        EventResult::Number(5)
    }
}

/// onModifyDef
/// JavaScript source (data/conditions.ts):
/// ```js
/// onModifyDefPriority: 10,
/// onModifyDef(def, pokemon) {
///     if (pokemon.hasType('Ice') && this.field.isWeather('snowscape')) {
///         return this.modify(def, 1.5);
///     }
/// }
/// ```
pub fn on_modify_def(
    battle: &mut Battle,
    _def: i32,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // if (pokemon.hasType('Ice') && this.field.isWeather('snowscape'))
    let has_ice_type = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_type(battle, "Ice")
    };

    let is_snowscape = battle.is_weather("snowscape");

    if has_ice_type && is_snowscape {
        // return this.modify(def, 1.5);
        EventResult::Float(1.5)
    } else {
        EventResult::Continue
    }
}

/// onFieldStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldStart(field, source, effect) {
///     if (effect?.effectType === 'Ability') {
///         if (this.gen <= 5) this.effectState.duration = 0;
///         this.add('-weather', 'Snowscape', '[from] ability: ' + effect.name, `[of] ${source}`);
///     } else {
///         this.add('-weather', 'Snowscape');
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

        // this.add('-weather', 'Snowscape', '[from] ability: ' + effect.name, `[of] ${source}`);
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
                Arg::Str("Snowscape"),
                Arg::String(format!("[from] ability: {}", ability_name)),
                Arg::String(format!("[of] {}", source_ident)),
            ],
        );
    } else {
        // this.add('-weather', 'Snowscape');
        battle.add("-weather", &[Arg::Str("Snowscape")]);
    }

    EventResult::Continue
}

/// onFieldResidual
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldResidual() {
///     this.add('-weather', 'Snowscape', '[upkeep]');
///     if (this.field.isWeather('snowscape')) this.eachEvent('Weather');
/// }
/// ```
pub fn on_field_residual(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-weather', 'Snowscape', '[upkeep]');
    battle.add("-weather", &[Arg::Str("Snowscape"), Arg::Str("[upkeep]")]);

    // if (this.field.isWeather('snowscape')) this.eachEvent('Weather');
    if battle.is_weather("snowscape") {
        battle.each_event("Weather", None, None);
    }

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

