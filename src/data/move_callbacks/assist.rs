//! Assist Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts
//!
//! ```text
//! JS Source (data/moves.ts):
//! 	assist: {
//! 		num: 274,
//! 		accuracy: true,
//! 		basePower: 0,
//! 		category: "Status",
//! 		isNonstandard: "Past",
//! 		name: "Assist",
//! 		pp: 20,
//! 		priority: 0,
//! 		flags: { failencore: 1, nosleeptalk: 1, noassist: 1, failcopycat: 1, failmimic: 1, failinstruct: 1 },
//! 		onHit(target) {
//! 			const moves = [];
//! 			for (const pokemon of target.side.pokemon) {
//! 				if (pokemon === target) continue;
//! 				for (const moveSlot of pokemon.moveSlots) {
//! 					const moveid = moveSlot.id;
//! 					const move = this.dex.moves.get(moveid);
//! 					if (move.flags['noassist'] || move.isZ || move.isMax) {
//! 						continue;
//! 					}
//! 					moves.push(moveid);
//! 				}
//! 			}
//! 			let randomMove = '';
//! 			if (moves.length) randomMove = this.sample(moves);
//! 			if (!randomMove) {
//! 				return false;
//! 			}
//! 			this.actions.useMove(randomMove, target);
//! 		},
//! 		callsMove: true,
//! 		secondary: null,
//! 		target: "self",
//! 		type: "Normal",
//! 		contestType: "Cute",
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{MoveHandlerResult, Status, Effect};

/// onHit callback for Assist
/// Randomly uses a move from a teammate's moveset
pub fn on_hit(
    battle: &mut Battle,
    target: (usize, usize),
    source: (usize, usize),
    move_id: &ID,
) -> MoveHandlerResult {
    // JavaScript: const moves = [];
    let mut moves: Vec<ID> = Vec::new();

    // JavaScript: for (const pokemon of target.side.pokemon) { ... }
    let (side_idx, target_idx) = target;
    let pokemon_count = if let Some(side) = battle.sides.get(side_idx) {
        side.pokemon.len()
    } else {
        return MoveHandlerResult::False;
    };

    for poke_idx in 0..pokemon_count {
        // JavaScript: if (pokemon === target) continue;
        if poke_idx == target_idx {
            continue;
        }

        // JavaScript: for (const moveSlot of pokemon.moveSlots) { ... }
        let pokemon_moves = if let Some(side) = battle.sides.get(side_idx) {
            if let Some(pokemon) = side.pokemon.get(poke_idx) {
                pokemon.move_slots.clone()
            } else {
                continue;
            }
        } else {
            continue;
        };

        for move_slot in &pokemon_moves {
            // JavaScript: const moveid = moveSlot.id;
            let moveid = &move_slot.id;

            // JavaScript: const move = this.dex.moves.get(moveid);
            // JavaScript: if (move.flags['noassist'] || move.isZ || move.isMax) { continue; }
            if let Some(move_def) = crate::data::moves::get_move(&moveid) {
                // Check noassist flag (TODO: need to add flags to MoveDef)
                // For now, skip checking flags and just add all moves
                // TODO: Check move.flags['noassist'], move.isZ, move.isMax
                moves.push(moveid.clone());
            }
        }
    }

    // JavaScript: let randomMove = '';
    // JavaScript: if (moves.length) randomMove = this.sample(moves);
    let random_move = if !moves.is_empty() {
        let idx = battle.random(moves.len() as u32) as usize;
        Some(moves[idx].clone())
    } else {
        None
    };

    // JavaScript: if (!randomMove) { return false; }
    if random_move.is_none() {
        return MoveHandlerResult::False;
    }

    let random_move = random_move.unwrap();

    // JavaScript: this.actions.useMove(randomMove, target);
    // TODO: Need to implement Battle::use_move_from_callback or similar
    // For now, this is a partial implementation that will need infrastructure
    battle.debug(&format!("Assist would use move: {}", random_move.as_str()));

    MoveHandlerResult::Undefined
}
