//! Weather Ball Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     switch (pokemon.effectiveWeather()) {
///     case 'sunnyday':
///     case 'desolateland':
///         move.type = 'Fire';
///         break;
///     case 'raindance':
///     case 'primordialsea':
///         move.type = 'Water';
///         break;
///     case 'sandstorm':
///         move.type = 'Rock';
///         break;
///     case 'hail':
///     case 'snowscape':
///         move.type = 'Ice';
///         break;
///     }
/// }
pub fn on_modify_type(
    battle: &mut Battle,
    _move_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    let pokemon = pokemon_pos;

    // switch (pokemon.effectiveWeather()) {
    let effective_weather = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.effective_weather(battle, battle.field.weather.as_str()).to_string()
    };

    if let Some(ref mut active_move) = battle.active_move {
        match effective_weather.as_ref() {
            // case 'sunnyday':
            // case 'desolateland':
            //     move.type = 'Fire';
            "sunnyday" | "desolateland" => {
                active_move.move_type = "Fire".to_string();
            }
            // case 'raindance':
            // case 'primordialsea':
            //     move.type = 'Water';
            "raindance" | "primordialsea" => {
                active_move.move_type = "Water".to_string();
            }
            // case 'sandstorm':
            //     move.type = 'Rock';
            "sandstorm" => {
                active_move.move_type = "Rock".to_string();
            }
            // case 'hail':
            // case 'snowscape':
            //     move.type = 'Ice';
            "hail" | "snowscape" => {
                active_move.move_type = "Ice".to_string();
            }
            _ => {}
        }
    }

    EventResult::Continue
}

/// onModifyMove(move, pokemon) {
///     switch (pokemon.effectiveWeather()) {
///     case 'sunnyday':
///     case 'desolateland':
///         move.basePower *= 2;
///         break;
///     case 'raindance':
///     case 'primordialsea':
///         move.basePower *= 2;
///         break;
///     case 'sandstorm':
///         move.basePower *= 2;
///         break;
///     case 'hail':
///     case 'snowscape':
///         move.basePower *= 2;
///         break;
///     }
///     this.debug(`BP: ${move.basePower}`);
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // switch (pokemon.effectiveWeather()) {
    let effective_weather = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.effective_weather(battle, battle.field.weather.as_str()).to_string()
    };

    let should_double = match effective_weather.as_ref() {
        "sunnyday" | "desolateland" | "raindance" | "primordialsea" | "sandstorm" | "hail"
        | "snowscape" => true,
        _ => false,
    };

    if should_double {
        if let Some(ref mut active_move) = battle.active_move {
            // case 'sunnyday':
            // case 'desolateland':
            //     move.basePower *= 2;
            // case 'raindance':
            // case 'primordialsea':
            //     move.basePower *= 2;
            // case 'sandstorm':
            //     move.basePower *= 2;
            // case 'hail':
            // case 'snowscape':
            //     move.basePower *= 2;
            active_move.base_power *= 2;
        }
    }

    // this.debug(`BP: ${move.basePower}`);
    let base_power = battle
        .active_move
        .as_ref()
        .map(|m| m.base_power)
        .unwrap_or(0);
    battle.debug(&format!("BP: {}", base_power));

    EventResult::Continue
}
