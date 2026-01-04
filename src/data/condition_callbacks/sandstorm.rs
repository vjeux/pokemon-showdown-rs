//! Sandstorm Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// durationCallback
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// sandstorm: {
///     durationCallback(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn duration_callback(
    _battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[SANDSTORM_DURATION_CALLBACK] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

/// onModifySpD
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// sandstorm: {
///     onModifySpD(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_modify_sp_d(
    _battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[SANDSTORM_ON_MODIFY_SP_D] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

/// onFieldStart
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// sandstorm: {
///     onFieldStart(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_field_start(
    _battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[SANDSTORM_ON_FIELD_START] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
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
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[SANDSTORM_ON_FIELD_RESIDUAL] Called for {:?}", pokemon_pos);

    // Add weather upkeep message
    use crate::battle::Arg;
    battle.add("weather", &[
        Arg::from("-weather"),
        Arg::from("Sandstorm"),
        Arg::from("[upkeep]"),
    ]);

    // Check if weather is still sandstorm
    if battle.field.weather == ID::from("sandstorm") {
        eprintln!("[SANDSTORM_ON_FIELD_RESIDUAL] Calling eachEvent('Weather')");
        battle.each_event("Weather", None, None);
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
) -> EventResult {
    eprintln!("[SANDSTORM_ON_WEATHER] Called for {:?}", pokemon_pos);

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

    eprintln!("[SANDSTORM_ON_WEATHER] Dealing {} damage (maxhp={}) to {:?}",
        damage_amount, base_maxhp, pokemon_pos);

    // Apply damage through the battle's damage function
    // JavaScript doesn't pass effect explicitly - it comes from this.effect in the event context
    // So we pass None to let it use current_event.effect
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
/// TODO: Implement 1-to-1 from JavaScript
/// JavaScript source (data/conditions.ts):
/// sandstorm: {
///     onFieldEnd(...) {
///         // Extract implementation from conditions.ts
///     }
/// }
pub fn on_field_end(
    _battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    eprintln!("[SANDSTORM_ON_FIELD_END] Called for {:?}", pokemon_pos);
    // TODO: Implement callback
    EventResult::Continue
}

