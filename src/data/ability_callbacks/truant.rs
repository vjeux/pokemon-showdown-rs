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
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect_id: Option<&str>) -> EventResult {
    use crate::dex_data::ID;
    use crate::pokemon::Pokemon;

    // pokemon.removeVolatile('truant');
    Pokemon::remove_volatile(battle, pokemon_pos, &ID::from("truant"));

    // if (pokemon.activeTurns && (pokemon.moveThisTurnResult !== undefined || !this.queue.willMove(pokemon)))
    let (active_turns, move_this_turn_result) = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        (pokemon.active_turns, pokemon.move_this_turn_result)
    };

    // Check if activeTurns > 0 (JavaScript treats 0 as falsy)
    if active_turns > 0 {
        // Check if moveThisTurnResult !== undefined (in Rust: is Some)
        // OR !this.queue.willMove(pokemon)
        // Note: We don't have access to queue.willMove yet, but moveThisTurnResult being Some
        // should cover the main case (Pokemon moved this turn)
        if move_this_turn_result.is_some() {
            // pokemon.addVolatile('truant');
            Pokemon::add_volatile(battle, pokemon_pos, ID::from("truant"), None, None, None, None);
        }
    }

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
    crate::pokemon::Pokemon::add_volatile(battle, pokemon_pos, crate::ID::from("truant"), None, None, None,
            None);

    EventResult::Continue
}

