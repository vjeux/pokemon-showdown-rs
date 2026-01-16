//! Hydration Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onResidual(pokemon) {
///     if (pokemon.status && ['raindance', 'primordialsea'].includes(pokemon.effectiveWeather())) {
///         this.debug('hydration');
///         this.add('-activate', pokemon, 'ability: Hydration');
///         pokemon.cureStatus();
///     }
/// }
pub fn on_residual(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::pokemon::Pokemon;
    use crate::battle::Arg;

    // if (pokemon.status && ['raindance', 'primordialsea'].includes(pokemon.effectiveWeather()))
    let field_weather_id = battle.effective_weather();
    let field_weather_str = field_weather_id.to_string();
    let (has_status, effective_weather) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (!pokemon.status.is_empty(), pokemon.effective_weather(battle, &field_weather_str))
    };

    if has_status && (effective_weather == "raindance" || effective_weather == "primordialsea") {
        debug_elog!("hydration");

        let pokemon_id = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            let side_id = format!("p{}", pokemon.side_index + 1);
            if pokemon.is_active {
                let pos_letter = (b'a' + pokemon.position as u8) as char;
                format!("{}{}: {}", side_id, pos_letter, pokemon.name)
            } else {
                format!("{}: {}", side_id, pokemon.name)
            }
        };

        battle.add("-activate", &[
            Arg::String(pokemon_id),
            Arg::Str("ability: Hydration"),
        ]);

        Pokemon::cure_status(battle, pokemon_pos, false);
    }

    EventResult::Continue
}

