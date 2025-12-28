//! Mirror Move Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onTryHit(target, pokemon) {
///     const move = target.lastMove;
///     if (!move?.flags['mirror'] || move.isZ || move.isMax) {
///         return false;
///     }
///     this.actions.useMove(move.id, pokemon, { target });
///     return null;
/// }
pub fn on_try_hit(battle: &mut Battle, source_pos: (usize, usize), target_pos: (usize, usize)) -> EventResult {
    

    let pokemon = source_pos;
    let target = target_pos;

    // const move = target.lastMove;
    let last_move_id = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Boolean(false),
        };
        target_pokemon.last_move.clone()
    };

    // if (!move?.flags['mirror'] || move.isZ || move.isMax) {
    //     return false;
    // }
    let move_id = match last_move_id {
        Some(id) => id,
        None => return EventResult::Boolean(false),
    };

    let move_data = match battle.dex.get_move_by_id(&move_id) {
        Some(m) => m,
        None => return EventResult::Boolean(false),
    };

    // Check if move has mirror flag
    if !move_data.flags.contains_key("mirror") {
        return EventResult::Boolean(false);
    }

    // Check if move is Z or Max
    if move_data.is_z_or_max_powered {
        return EventResult::Boolean(false);
    }

    // this.actions.useMove(move.id, pokemon, { target });
    crate::battle_actions::use_move(battle, &move_id, pokemon, Some(target), None, None, None);

    // return null;
    EventResult::Stop
}

