//! Terrain Pulse Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyType(move, pokemon) {
///     if (!pokemon.isGrounded()) return;
///     switch (this.field.terrain) {
///     case 'electricterrain':
///         move.type = 'Electric';
///         break;
///     case 'grassyterrain':
///         move.type = 'Grass';
///         break;
///     case 'mistyterrain':
///         move.type = 'Fairy';
///         break;
///     case 'psychicterrain':
///         move.type = 'Psychic';
///         break;
///     }
/// }
pub fn on_modify_type(
    battle: &mut Battle,
    _move_id: &str,
    pokemon_pos: (usize, usize),
) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // if (!pokemon.isGrounded()) return;
    let is_grounded = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.is_grounded(battle, false)
    };

    if !is_grounded {
        return EventResult::Continue;
    }

    // switch (this.field.terrain)
    let terrain = &battle.field.terrain;

    let new_type = if terrain == &ID::from("electricterrain") {
        "Electric"
    } else if terrain == &ID::from("grassyterrain") {
        "Grass"
    } else if terrain == &ID::from("mistyterrain") {
        "Fairy"
    } else if terrain == &ID::from("psychicterrain") {
        "Psychic"
    } else {
        return EventResult::Continue;
    };

    // move.type = ...
    if let Some(ref mut active_move) = battle.active_move {
        active_move.move_type = new_type.to_string();
    }

    EventResult::Continue
}

/// onModifyMove(move, pokemon) {
///     if (this.field.terrain && pokemon.isGrounded()) {
///         move.basePower *= 2;
///         this.debug('BP doubled in Terrain');
///     }
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {

    let pokemon = pokemon_pos;

    // if (this.field.terrain && pokemon.isGrounded())
    let terrain_active = !battle.field.terrain.is_empty();

    if !terrain_active {
        return EventResult::Continue;
    }

    let is_grounded = {
        let pokemon_ref = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_ref.is_grounded(battle, false)
    };

    if !is_grounded {
        return EventResult::Continue;
    }

    // move.basePower *= 2;
    if let Some(ref mut active_move) = battle.active_move {
        active_move.base_power *= 2;
        // this.debug('BP doubled in Terrain');
        // Debug is typically not logged in production code
    }

    EventResult::Continue
}
