//! Synthesis Move
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
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // Get effective weather and determine factor
    // NOTE: Must use battle.effective_weather() to account for Air Lock/Cloud Nine
    let field_weather = battle.effective_weather();
    let (numerator, denominator) = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let weather = pokemon_pokemon.effective_weather(battle, field_weather.as_str());

        // let factor = 0.5;
        // switch (pokemon.effectiveWeather()) {
        // case 'sunnyday':
        // case 'desolateland':
        //     factor = 0.667;
        //     break;
        // case 'raindance':
        // case 'primordialsea':
        // case 'sandstorm':
        // case 'hail':
        // case 'snowscape':
        //     factor = 0.25;
        //     break;
        // }
        match weather {
            "sunnyday" | "desolateland" => (2, 3), // 0.667
            "raindance" | "primordialsea" | "sandstorm" | "hail" | "snowscape" => (1, 4), // 0.25
            _ => (1, 2), // 0.5
        }
    };

    // const success = !!this.heal(this.modify(pokemon.maxhp, factor));
    let heal_amount = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        battle.modify(pokemon_pokemon.maxhp, numerator, denominator)
    };

    let success = battle.heal(heal_amount, Some(pokemon), None, None);

    // if (!success) {
    //     this.add('-fail', pokemon, 'heal');
    //     return this.NOT_FAIL;
    // }
    if success.is_none() {
        let pokemon_arg = {
            let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            pokemon_pokemon.get_slot()
        };

        battle.add("-fail", &[pokemon_arg.into(), "heal".into()]);
        return EventResult::NotFail;
    }

    // return success;
    EventResult::Continue
}
