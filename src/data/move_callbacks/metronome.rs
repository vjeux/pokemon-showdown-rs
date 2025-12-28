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
pub fn on_hit(battle: &mut Battle, pokemon_pos: (usize, usize), target_pos: Option<(usize, usize)>) -> EventResult {
    use crate::dex_data::ID;

    let pokemon = pokemon_pos;

    // const moves = this.dex.moves.all().filter(move => (
    //     (!move.isNonstandard || move.isNonstandard === 'Unobtainable') &&
    //     move.flags['metronome']
    // ));
    let all_moves = battle.dex.get_all_moves();
    let mut moves: Vec<_> = all_moves
        .iter()
        .filter(|move_data| {
            // (!move.isNonstandard || move.isNonstandard === 'Unobtainable')
            let nonstandard_ok = move_data.is_nonstandard.is_none() ||
                                 move_data.is_nonstandard.as_deref() == Some("Unobtainable");
            // move.flags['metronome']
            let has_metronome_flag = move_data.flags.contains_key("metronome");

            nonstandard_ok && has_metronome_flag
        })
        .collect();

    // let randomMove = '';
    let random_move: Option<ID>;

    // if (moves.length) {
    if !moves.is_empty() {
        // moves.sort((a, b) => a.num - b.num);
        moves.sort_by_key(|m| m.num);

        // randomMove = this.sample(moves).id;
        let sampled = battle.sample(&moves);
        random_move = sampled.map(|m| m.id.clone());
    } else {
        random_move = None;
    }

    // if (!randomMove) return false;
    let move_id = match random_move {
        Some(id) => id,
        None => return EventResult::Boolean(false),
    };

    // this.actions.useMove(randomMove, pokemon);
    crate::battle_actions::use_move(battle, &move_id, pokemon, None, None);

    EventResult::Continue
}

