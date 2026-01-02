//! G-Max Depletion Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(source)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(source) {
/// 				for (const pokemon of source.foes()) {
/// 					let move: Move | ActiveMove | null = pokemon.lastMove;
/// 					if (!move || move.isZ) continue;
/// 					if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
///
/// 					const ppDeducted = pokemon.deductPP(move.id, 2);
/// 					if (ppDeducted) {
/// 						this.add("-activate", pokemon, 'move: G-Max Depletion', move.name, ppDeducted);
/// 						// Don't return here because returning early doesn't trigger
/// 						// activation text for the second Pokemon in doubles
/// 					}
/// 				}
/// 			},
///
/// 		}
/// ```
pub fn on_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {

    // for (const pokemon of source.foes()) {
    let all_active = battle.get_all_active(false);
    let source_side = source_pos.0;

    for pokemon_pos in all_active {
        // Only process foes (pokemon on different side than source)
        if pokemon_pos.0 == source_side {
            continue;
        }

        //     let move: Move | ActiveMove | null = pokemon.lastMove;
        //     if (!move || move.isZ) continue;
        let move_id = {
            let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => continue,
            };

            match &pokemon.last_move {
                Some(move_id) => move_id.clone(),
                None => continue,
            }
        };

        // Get move data to check if it's a Z-move or Max move
        let move_data = match battle.dex.moves().get_by_id(&move_id) {
            Some(m) => m,
            None => continue,
        };

        // if (!move || move.isZ) continue;
        if move_data.is_z.is_some() {
            continue;
        }

        //     if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
        let actual_move_id = if move_data.is_max.is_some() {
            if let Some(ref base_move) = move_data.base_move {
                base_move.clone()
            } else {
                move_id.clone()
            }
        } else {
            move_id.clone()
        };

        //     const ppDeducted = pokemon.deductPP(move.id, 2);
        let pp_deducted = {
            let gen = battle.gen;
            let pokemon = match battle.pokemon_at_mut(pokemon_pos.0, pokemon_pos.1) {
                Some(p) => p,
                None => continue,
            };
            pokemon.deduct_pp(gen, &actual_move_id, Some(2))
        };

        //     if (ppDeducted) {
        //         this.add("-activate", pokemon, 'move: G-Max Depletion', move.name, ppDeducted);
        if pp_deducted > 0 {
            let move_name = battle.dex.moves().get_by_id(&actual_move_id)
                .map(|m| m.name.clone())
                .unwrap_or_else(|| actual_move_id.to_string());

            let pokemon_slot = {
                let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
                    Some(p) => p,
                    None => continue,
                };
                pokemon.get_slot()
            };

            battle.add(
                "-activate",
                &[
                    crate::battle::Arg::from(pokemon_slot),
                    crate::battle::Arg::from("move: G-Max Depletion"),
                    crate::battle::Arg::from(move_name),
                    crate::battle::Arg::from("2"),
                ],
            );
        //         // Don't return here because returning early doesn't trigger
        //         // activation text for the second Pokemon in doubles
        }
    //     }
    // }
    }

    EventResult::Continue
}
