//! Morning Sun Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(pokemon) {
///     let factor = 0.5;
///     switch (pokemon.effectiveWeather()) {
///     case 'sunnyday':
///     case 'desolateland':
///         factor = 0.667;
///         break;
///     case 'raindance':
///     case 'primordialsea':
///     case 'sandstorm':
///     case 'hail':
///     case 'snowscape':
///         factor = 0.25;
///         break;
///     }
///     const success = !!this.heal(this.modify(pokemon.maxhp, factor));
///     if (!success) {
///         this.add('-fail', pokemon, 'heal');
///         return this.NOT_FAIL;
///     }
///     return success;
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    

    let pokemon = pokemon_pos;

    // let factor = 0.5;
    let mut factor = 0.5;

    // switch (pokemon.effectiveWeather()) {
    let effective_weather = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.effective_weather(battle.field.weather.as_str())
    };

    match effective_weather.as_str() {
        // case 'sunnyday':
        // case 'desolateland':
        "sunnyday" | "desolateland" => {
            // factor = 0.667;
            factor = 0.667;
        }
        // case 'raindance':
        // case 'primordialsea':
        // case 'sandstorm':
        // case 'hail':
        // case 'snowscape':
        "raindance" | "primordialsea" | "sandstorm" | "hail" | "snowscape" => {
            // factor = 0.25;
            factor = 0.25;
        }
        _ => {
            // Default factor remains 0.5
        }
    }

    // const success = !!this.heal(this.modify(pokemon.maxhp, factor));
    let max_hp = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.maxhp
    };

    let heal_amount = (max_hp as f64 * factor).round() as i32;
    let heal_result = battle.heal(heal_amount, Some(pokemon), None, None);
    let success = heal_result.unwrap_or(0) > 0;

    // if (!success) {
    if !success {
        // this.add('-fail', pokemon, 'heal');
        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_slot()
        };

        battle.add("-fail", &[pokemon_arg.into(), "heal".into()]);

        // return this.NOT_FAIL;
        return EventResult::NotFail;
    }

    // return success;
    EventResult::Boolean(success)
}

