//! Sandstorm Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Arg;
use crate::dex_data::ID;
use crate::event::EventResult;

/// durationCallback
/// JavaScript source (data/conditions.ts):
/// ```js
/// durationCallback(source, effect) {
///     if (source?.hasItem('smoothrock')) {
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
    // if (source?.hasItem('smoothrock'))
    let has_smoothrock = if let Some(pos) = source_pos {
        let pokemon = match battle.pokemon_at(pos.0, pos.1) {
            Some(p) => p,
            None => return EventResult::Number(5),
        };
        pokemon.has_item(battle, &["smoothrock"])
    } else {
        false
    };

    if has_smoothrock {
        // return 8;
        return EventResult::Number(8);
    }

    // return 5;
    EventResult::Number(5)
}

/// onModifySpD
/// JavaScript source (data/conditions.ts):
/// ```js
/// onModifySpDPriority: 10,
/// onModifySpD(spd, pokemon) {
///     if (pokemon.hasType('Rock') && this.field.isWeather('sandstorm')) {
///         return this.modify(spd, 1.5);
///     }
/// }
/// ```
pub fn on_modify_sp_d(
    battle: &mut Battle,
    _spd: i32,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // if (pokemon.hasType('Rock') && this.field.isWeather('sandstorm'))
    let has_rock_type = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.has_type(battle, "rock")
    };

    let is_sandstorm = battle.field.weather == ID::from("sandstorm");

    if has_rock_type && is_sandstorm {
        // return this.modify(spd, 1.5);
        battle.chain_modify(1.5); return EventResult::Continue;
    }

    EventResult::Continue
}

/// onFieldStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onFieldStart(field, source, effect) {
///     if (effect?.effectType === 'Ability') {
///         if (this.gen <= 5) this.effectState.duration = 0;
///         this.add('-weather', 'Sandstorm', '[from] ability: ' + effect.name, `[of] ${source}`);
///     } else {
///         this.add('-weather', 'Sandstorm');
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

        // this.add('-weather', 'Sandstorm', '[from] ability: ' + effect.name, `[of] ${source}`);
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
                Arg::Str("Sandstorm"),
                Arg::String(format!("[from] ability: {}", ability_name)),
                Arg::String(format!("[of] {}", source_ident)),
            ],
        );
    } else {
        // this.add('-weather', 'Sandstorm');
        battle.add("-weather", &[Arg::Str("Sandstorm")]);
    }

    EventResult::Continue
}

/// onFieldResidual
/// Implemented 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// sandstorm: {
///     onFieldResidualOrder: 1,
///     onFieldResidual() {
///         this.add('-weather', 'Sandstorm', '[upkeep]');
///         if (this.field.isWeather('sandstorm')) this.eachEvent('Weather');
///     }
/// }
pub fn on_field_residual(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
) -> EventResult {
    // Add weather upkeep message
    use crate::battle::Arg;
    battle.add("weather", &[
        Arg::from("-weather"),
        Arg::from("Sandstorm"),
        Arg::from("[upkeep]"),
    ]);

    // Check if weather is still sandstorm
    if battle.field.weather == ID::from("sandstorm") {
        // Pass sandstorm as the effect so Weather event knows which weather condition to use
        // In JavaScript: this.eachEvent('Weather') uses this.effect which is the sandstorm condition
        use crate::battle::Effect;
        battle.each_event("Weather", Some(&Effect::weather(ID::from("sandstorm"))), None);
    }

    EventResult::Continue
}

/// onWeather
/// Implemented 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// sandstorm: {
///     onWeather(target) {
///         this.damage(target.baseMaxhp / 16);
///     }
/// }
pub fn on_weather(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _effect_id: Option<&str>,
) -> EventResult {
    // Get target's base max HP
    let base_maxhp = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.maxhp
    };

    // Calculate damage as 1/16 of max HP
    let damage_amount = base_maxhp / 16;

    // Apply damage through the battle's damage function
    // JavaScript doesn't pass effect explicitly - it comes from this.effect in the event context
    // So we pass None to let it use event.effect
    battle.damage(
        damage_amount,
        Some(pokemon_pos),
        None, // no source
        None, // effect comes from event context
        false, // not instafaint
    );

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

