//! Flinch Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onBeforeMove
/// JavaScript source (data/conditions.ts):
/// flinch: {
///     name: 'flinch',
///     duration: 1,
///     onBeforeMovePriority: 8,
///     onBeforeMove(pokemon) {
///         this.add('cant', pokemon, 'flinch');
///         this.runEvent('Flinch', pokemon);
///         return false;
///     },
/// },
pub fn on_before_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
    _move_id: &str,
) -> EventResult {
    // JavaScript: this.add('cant', pokemon, 'flinch');
    // Add "cant" message to battle log
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
            crate::battle::Arg::from("flinch"),
        ],
    );

    // JavaScript: this.runEvent('Flinch', pokemon);
    // Run Flinch event
    battle.run_event(
        "Flinch",
        Some(crate::event::EventTarget::Pokemon(pokemon_pos)),
        None,
        None,
        crate::event::EventResult::Number(1),
        false,
        false,
    );

    // JavaScript: return false;
    // Return false to prevent the move from executing
    EventResult::Boolean(false)
}

