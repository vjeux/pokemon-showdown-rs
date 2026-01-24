//! Struggle Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onModifyMove(move, pokemon, target) {
///     move.type = '???';
///     this.add('-activate', pokemon, 'move: Struggle');
/// }
pub fn on_modify_move(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    let pokemon = pokemon_pos;

    // move.type = '???';
    if let Some(ref mut active_move) = battle.active_move {
        active_move.borrow_mut().move_type = "???".to_owned();
    }

    // this.add('-activate', pokemon, 'move: Struggle');
    let pokemon_arg = {
        let pokemon_pokemon = match battle.pokemon_at(pokemon.0, pokemon.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon_pokemon.get_slot()
    };

    battle.add("-activate", &[pokemon_arg.into(), "move: Struggle".into()]);

    EventResult::Continue
}
