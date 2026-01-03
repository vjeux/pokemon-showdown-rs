//! Truant Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     pokemon.removeVolatile('truant');
///     if (pokemon.activeTurns && (pokemon.moveThisTurnResult !== undefined || !this.queue.willMove(pokemon))) {
///         pokemon.addVolatile('truant');
///     }
/// }
pub fn on_start(_battle: &mut Battle, _pokemon_pos: (usize, usize)) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onBeforeMove(pokemon) {
///     if (pokemon.removeVolatile('truant')) {
///         this.add('cant', pokemon, 'ability: Truant');
///         return false;
///     }
///     pokemon.addVolatile('truant');
/// }
pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>, _move_id: &str) -> EventResult {
    // if (pokemon.removeVolatile('truant'))
    let removed = crate::pokemon::Pokemon::remove_volatile(
        battle,
        pokemon_pos,
        &crate::ID::from("truant"),
    );

    if removed {
        // this.add('cant', pokemon, 'ability: Truant');
        let pokemon_slot = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => return EventResult::Boolean(false),
            };
            pokemon.get_slot()
        };

        battle.add(
            "cant",
            &[
                crate::battle::Arg::from(pokemon_slot),
                crate::battle::Arg::from("ability: Truant"),
            ],
        );

        // return false;
        return EventResult::Boolean(false);
    }

    // pokemon.addVolatile('truant');
    crate::pokemon::Pokemon::add_volatile(
        battle,
        pokemon_pos,
        crate::ID::from("truant"),
        None,
        None,
        None,
    );

    EventResult::Continue
}

