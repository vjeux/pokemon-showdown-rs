//! Truant Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::battle::Effect;
use crate::event::EventResult;
use crate::battle_actions::MoveResult;

/// onStart(pokemon) {
///     pokemon.removeVolatile('truant');
///     if (pokemon.activeTurns && (pokemon.moveThisTurnResult !== undefined || !this.queue.willMove(pokemon))) {
///         pokemon.addVolatile('truant');
///     }
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize), _source_pos: Option<(usize, usize)>, _effect: Option<&Effect>) -> EventResult {
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

    // Check if queue.willMove(pokemon) - does this pokemon have a move action queued?
    let will_move = battle.queue.will_move(pokemon_pos.0, pokemon_pos.1).is_some();

    // Check if activeTurns > 0 (JavaScript treats 0 as falsy)
    if active_turns > 0 {
        // Check if moveThisTurnResult !== undefined (in Rust: not MoveResult::Undefined)
        // OR !this.queue.willMove(pokemon)
        if move_this_turn_result != MoveResult::Undefined || !will_move {
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
pub fn on_before_move(battle: &mut Battle, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>, _active_move: Option<&crate::battle_actions::ActiveMove>) -> EventResult {
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

