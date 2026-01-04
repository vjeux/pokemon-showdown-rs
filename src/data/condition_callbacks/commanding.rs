//! Commanding Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::pokemon::TrappedState;

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

/// onBeforeTurn
/// JavaScript source (data/conditions.ts):
/// ```js
/// onBeforeTurn(pokemon) {
///     this.queue.cancelAction(pokemon);
/// }
/// ```
pub fn on_before_turn(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // this.queue.cancelAction(pokemon);
    battle.queue.cancel_action(pokemon_pos.0, pokemon_pos.1);

    EventResult::Continue
}

