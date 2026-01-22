//! Commanding Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Effect;
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
    _source_pos: Option<(usize, usize)>,
    _active_move: Option<&crate::battle_actions::ActiveMove>,
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

/// onInvulnerability: false
/// JavaScript source (data/conditions.ts):
/// ```js
/// // Dodging moves is handled in BattleActions#hitStepInvulnerabilityEvent
/// // This is here for moves that manually call this event like Perish Song
/// onInvulnerability: false,
/// ```
///
/// When onInvulnerability is set to the static value `false`, it means
/// the Pokemon is ALWAYS invulnerable while this condition is active.
pub fn on_invulnerability(
    _battle: &mut Battle,
    _target_pos: Option<(usize, usize)>,
    _source_pos: Option<(usize, usize)>,
    _attacking_active_move: Option<&crate::battle_actions::ActiveMove>,
) -> EventResult {
    // onInvulnerability: false means always invulnerable
    EventResult::Boolean(false)
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

