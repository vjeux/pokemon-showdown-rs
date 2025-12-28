//! Flail Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// basePowerCallback(pokemon) {
///     const ratio = Math.max(Math.floor(pokemon.hp * 48 / pokemon.maxhp), 1);
///     let bp;
///     if (ratio < 2) {
///         bp = 200;
///     } else if (ratio < 5) {
///         bp = 150;
///     } else if (ratio < 10) {
///         bp = 100;
///     } else if (ratio < 17) {
///         bp = 80;
///     } else if (ratio < 33) {
///         bp = 40;
///     } else {
///         bp = 20;
///     }
///     this.debug(`BP: ${bp}`);
///     return bp;
/// }
pub fn base_power_callback(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    let pokemon = pokemon_pos;

    // const ratio = Math.max(Math.floor(pokemon.hp * 48 / pokemon.maxhp), 1);
    let ratio = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        std::cmp::max((pokemon_pokemon.hp * 48 / pokemon_pokemon.max_hp) as i32, 1)
    };

    // let bp;
    // if (ratio < 2) {
    //     bp = 200;
    // } else if (ratio < 5) {
    //     bp = 150;
    // } else if (ratio < 10) {
    //     bp = 100;
    // } else if (ratio < 17) {
    //     bp = 80;
    // } else if (ratio < 33) {
    //     bp = 40;
    // } else {
    //     bp = 20;
    // }
    let bp = if ratio < 2 {
        200
    } else if ratio < 5 {
        150
    } else if ratio < 10 {
        100
    } else if ratio < 17 {
        80
    } else if ratio < 33 {
        40
    } else {
        20
    };

    // this.debug(`BP: ${bp}`);
    battle.debug(&format!("BP: {}", bp));

    // return bp;
    EventResult::Number(bp)
}

