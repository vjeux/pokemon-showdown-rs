//! Assist Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::battle_actions;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onHit(target) {
///     const moves = [];
///     for (const pokemon of target.side.pokemon) {
///         if (pokemon === target) continue;
///         for (const moveSlot of pokemon.moveSlots) {
///             const moveid = moveSlot.id;
///             const move = this.dex.moves.get(moveid);
///             if (move.flags['noassist'] || move.isZ || move.isMax) {
///                 continue;
///             }
///             moves.push(moveid);
///         }
///     }
///     let randomMove = '';
///     if (moves.length) randomMove = this.sample(moves);
///     if (!randomMove) {
///         return false;
///     }
///     this.actions.useMove(randomMove, target);
/// }
pub fn on_hit(
    battle: &mut Battle,
    pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // Get the target position
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // const moves = [];
    let mut moves: Vec<ID> = Vec::new();

    // for (const pokemon of target.side.pokemon) {
    let target_side_idx = target.0;
    let num_pokemon = battle.sides[target_side_idx].pokemon.len();

    for poke_idx in 0..num_pokemon {
        let poke_pos = (target_side_idx, poke_idx);

        // if (pokemon === target) continue;
        if poke_pos == target {
            continue;
        }

        let pokemon = match battle.pokemon_at(poke_pos.0, poke_pos.1) {
            Some(p) => p,
            None => continue,
        };

        // for (const moveSlot of pokemon.moveSlots) {
        for move_slot in &pokemon.move_slots {
            // const moveid = moveSlot.id;
            let moveid = &move_slot.id;

            // const move = this.dex.moves.get(moveid);
            let move_data = battle.dex.moves().get_by_id(moveid).unwrap();

            // if (move.flags['noassist'] || move.isZ || move.isMax) {
            //     continue;
            // }
            if move_data.flags.contains_key("noassist")
                || move_data.is_z.is_some()
                || move_data.is_max.is_some()
            {
                continue;
            }

            // moves.push(moveid);
            moves.push(moveid.clone());
        }
    }

    // let randomMove = '';
    // if (moves.length) randomMove = this.sample(moves);
    let random_move = if !moves.is_empty() {
        battle.sample(&moves)
    } else {
        None
    };

    // if (!randomMove) {
    //     return false;
    // }
    let random_move = match random_move {
        Some(m) => m,
        None => return EventResult::Boolean(false),
    };

    // this.actions.useMove(randomMove, target);
    // In JS, useMove(move, pokemon, options) - the 2nd arg is the user, and options.target is undefined
    // So the called move picks its own target. We pass None for target to match this behavior.
    let move_data = match battle.dex.moves().get_by_id(&random_move).cloned() {
        Some(m) => m,
        None => return EventResult::Continue,
    };
    battle_actions::use_move(
        battle,
        &move_data,
        target, // target is the Assist user (same as pokemon_pos since Assist targets self)
        None,   // Let the called move pick its own target
        None,
        None,
        None,
    );

    EventResult::Continue
}
