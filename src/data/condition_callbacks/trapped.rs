//! Trapped Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::battle::Arg;
use crate::event::EventResult;

/// onTrapPokemon
/// JavaScript source (data/conditions.ts):
/// ```js
/// onTrapPokemon(pokemon) {
///     pokemon.tryTrap();
/// }
/// ```
pub fn on_trap_pokemon(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // pokemon.tryTrap();
    crate::pokemon::Pokemon::try_trap(battle, pokemon_pos, false);

    EventResult::Continue
}

/// onStart
/// JavaScript source (data/conditions.ts):
/// ```js
/// onStart(target) {
///     this.add('-activate', target, 'trapped');
/// }
/// ```
pub fn on_start(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
) -> EventResult {
    // this.add('-activate', target, 'trapped');
    let target_ident = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-activate", &[Arg::String(target_ident), Arg::Str("trapped")]);

    EventResult::Continue
}

