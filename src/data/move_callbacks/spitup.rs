//! Spit Up Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

/// basePowerCallback(pokemon) {
///     if (!pokemon.volatiles['stockpile']?.layers) return false;
///     return pokemon.volatiles['stockpile'].layers * 100;
/// }
pub fn base_power_callback(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // basePowerCallback(pokemon) {
    //     if (!pokemon.volatiles['stockpile']?.layers) return false;
    //     return pokemon.volatiles['stockpile'].layers * 100;
    // }
    let pokemon = pokemon_pos;

    // if (!pokemon.volatiles['stockpile']?.layers) return false;
    let layers = {
        let pokemon_data = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        if let Some(volatile) = pokemon_data.volatiles.get(&ID::from("stockpile")) {
            volatile.borrow().layers.unwrap_or(0)
        } else {
            0
        }
    };

    if layers == 0 {
        return EventResult::Boolean(false);
    }

    // return pokemon.volatiles['stockpile'].layers * 100;
    EventResult::Number(layers * 100)
}

/// onTry(source) {
///     return !!source.volatiles['stockpile'];
/// }
pub fn on_try(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // onTry(source) {
    //     return !!source.volatiles['stockpile'];
    // }
    let source = source_pos;

    // return !!source.volatiles['stockpile'];
    let has_stockpile = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon
            .volatiles
            .contains_key(&ID::from("stockpile"))
    };

    EventResult::Boolean(has_stockpile)
}

/// onAfterMove(pokemon) {
///     pokemon.removeVolatile('stockpile');
/// }
pub fn on_after_move(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    use crate::dex_data::ID;

    // onAfterMove(pokemon) {
    //     pokemon.removeVolatile('stockpile');
    // }
    let pokemon = source_pos;

    // pokemon.removeVolatile('stockpile');
    {
        Pokemon::remove_volatile(battle, pokemon, &ID::from("stockpile"));
    }

    EventResult::Continue
}
