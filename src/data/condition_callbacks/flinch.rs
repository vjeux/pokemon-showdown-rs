//! Flinch Condition
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! JavaScript source: data/conditions.ts

use crate::battle::Battle;
use crate::dex_data::ID;
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
    battle.run_event_bool("Flinch", Some(pokemon_pos), None, None);

    // JavaScript: return false;
    // Return false to prevent the move from executing
    EventResult::Boolean(false)
}

