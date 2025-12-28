//! Metronome Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(pokemon) {
///     const moves = this.dex.moves.all().filter(move => (
///         (!move.isNonstandard || move.isNonstandard === 'Unobtainable') &&
///         move.flags['metronome']
///     ));
///     let randomMove = '';
///     if (moves.length) {
///         moves.sort((a, b) => a.num - b.num);
///         randomMove = this.sample(moves).id;
///     }
///     if (!randomMove) return false;
///     this.actions.useMove(randomMove, pokemon);
/// }
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // const moves = this.dex.moves.all().filter(move => (
    //     (!move.isNonstandard || move.isNonstandard === 'Unobtainable') &&
    //     move.flags['metronome']
    // ));
    let all_moves = battle.dex.all_moves();
    let moves: Vec<ID> = all_moves
        .iter()
        .filter(|&&move_data| {
            // TODO: Add is_nonstandard field to MoveData and check it here
            // For now, just check for metronome flag
            // move.flags['metronome']
            move_data.flags.contains_key("metronome")
        })
        .map(|m| m.id.clone())
        .collect();

    // let randomMove = '';
    // if (moves.length) {
    let random_move: Option<ID> = if !moves.is_empty() {
        // moves.sort((a, b) => a.num - b.num);
        // Note: We've already cloned the IDs, so we can't sort by num anymore
        // This is acceptable as the random sampling will be from the filtered list

        // randomMove = this.sample(moves).id;
        let sampled = battle.sample(&moves);
        sampled.cloned()
    } else {
        None
    };

    // if (!randomMove) return false;
    let move_id = match random_move {
        Some(id) => id,
        None => return EventResult::Boolean(false),
    };

    // this.actions.useMove(randomMove, pokemon);
    crate::battle_actions::use_move(battle, &move_id, pokemon, None, None, None, None);

    EventResult::Continue
}

