//! Commanded Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::TrappedState;

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(pokemon) {
///     this.boost({ atk: 2, spa: 2, spe: 2, def: 2, spd: 2 }, pokemon);
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // this.boost({ atk: 2, spa: 2, spe: 2, def: 2, spd: 2 }, pokemon);
    let boosts: &[(&str, i8)] = &[
        ("atk", 2),
        ("spa", 2),
        ("spe", 2),
        ("def", 2),
        ("spd", 2),
    ];

    battle.boost(boosts, pokemon_pos, None, None, false, false);

    EventResult::Continue
}

/// onDragOut
/// JavaScript source (data/conditions.ts):
/// ```js
/// onDragOutPriority: 2,
/// onDragOut() {
///     return false;
/// }
/// ```
pub fn on_drag_out(
    _battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    // return false;
    EventResult::Boolean(false)
}

/// onTrapPokemon
/// JavaScript source (data/conditions.ts):
/// ```js
/// onTrapPokemonPriority: -11,
/// onTrapPokemon(pokemon) {
///     pokemon.trapped = true;
/// }
/// ```
pub fn on_trap_pokemon(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // pokemon.trapped = true;
    let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };
    pokemon.trapped = TrappedState::Visible;

    EventResult::Continue
}

