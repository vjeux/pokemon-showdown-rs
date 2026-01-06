//! Hail Condition
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
    pokemon_pos: (usize, usize),
) -> EventResult {
    // if (source?.hasItem('icyrock')) { return 8; }
    let has_icy_rock = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Number(5),
        };
        pokemon.has_item(battle, &["icyrock"])
    };

    if has_icy_rock {
        EventResult::Number(8)
    } else {
        EventResult::Number(5)
    }
}

/// onFieldStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldStart(field, source, effect) {
///     if (effect?.effectType === 'Ability') {
///         if (this.gen <= 5) this.effectState.duration = 0;
///         this.add('-weather', 'Hail', '[from] ability: ' + effect.name, `[of] ${source}`);
///     } else {
///         this.add('-weather', 'Hail');
///     }
/// }
/// ```
pub fn on_field_start(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // The source and effect come from the weather state
    // Check if the effect is an ability
    // In Rust, we need to check battle.effect or battle.active_move to see what triggered this
    // For weather, the source effect is typically stored in the field weather state

    // Try to get source effect from battle.effect
    let is_ability = battle.effect.as_ref()
        .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
        .is_some();

    if is_ability {
        // if (this.gen <= 5) this.effectState.duration = 0;
        if battle.gen <= 5 {
            // Set weather duration to 0 (infinite)
            battle.field.weather_state.duration = Some(0);
        }

        // this.add('-weather', 'Hail', '[from] ability: ' + effect.name, `[of] ${source}`);
        let ability_name = battle.effect.as_ref()
            .and_then(|eff_id| battle.dex.abilities().get(eff_id.as_str()))
            .map(|ab| ab.name.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        // Get source Pokemon ident
        // The source is typically stored in the weather state
        let source_ident = battle.field.weather_state.source
            .and_then(|(side_idx, poke_idx)| battle.pokemon_at(side_idx, poke_idx))
            .map(|p| p.get_slot())
            .unwrap_or_else(|| String::new());

        battle.add(
            "-weather",
            &[
                Arg::Str("Hail"),
                Arg::String(format!("[from] ability: {}", ability_name)),
                Arg::String(format!("[of] {}", source_ident)),
            ],
        );
    } else {
        // this.add('-weather', 'Hail');
        battle.add("-weather", &[Arg::Str("Hail")]);
    }

    EventResult::Continue
}

/// onFieldResidual
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldResidual() {
///     this.add('-weather', 'Hail', '[upkeep]');
///     if (this.field.isWeather('hail')) this.eachEvent('Weather');
/// }
/// ```
pub fn on_field_residual(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-weather', 'Hail', '[upkeep]');
    battle.add("-weather", &[Arg::Str("Hail"), Arg::Str("[upkeep]")]);

    // if (this.field.isWeather('hail')) this.eachEvent('Weather');
    if battle.is_weather("hail") {
        battle.each_event("Weather", None, None);
    }

    EventResult::Continue
}

/// onWeather
/// JavaScript source (data/conditions.ts):
/// ```js
/// onWeather(target) {
///     this.damage(target.baseMaxhp / 16);
/// }
/// ```
pub fn on_weather(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // this.damage(target.baseMaxhp / 16);
    let base_maxhp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.base_maxhp
    };

    let damage = base_maxhp / 16;
    battle.damage(damage, Some(pokemon_pos), None, None, false);

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

