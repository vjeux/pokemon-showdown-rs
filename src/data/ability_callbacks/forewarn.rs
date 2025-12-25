//! Forewarn Ability - Reveals foe's strongest move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	forewarn: {
//! 		onStart(pokemon) {
//! 			let warnMoves: (Move | Pokemon)[] = [];
//! 			let warnBp = 1;
//! 			for (const target of pokemon.foes()) {
//! 				for (const moveSlot of target.moveSlots) {
//! 					const move = this.dex.moves.get(moveSlot.move);
//! 					let bp = move.basePower;
//! 					if (move.ohko) bp = 150;
//! 					if (move.id === 'counter' || move.id === 'metalburst' || move.id === 'mirrorcoat') bp = 120;
//! 					if (!bp && move.category !== 'Status') bp = 80;
//! 					if (bp > warnBp) {
//! 						warnMoves = [[move, target] as [Move, Pokemon]];
//! 						warnBp = bp;
//! 					} else if (bp === warnBp) {
//! 						warnMoves.push([move, target]);
//! 					}
//! 				}
//! 			}
//! 			if (!warnMoves.length) return;
//! 			const [warnMoveName, warnTarget] = this.sample(warnMoves);
//! 			this.add('-activate', pokemon, 'ability: Forewarn', warnMoveName, `[of] ${warnTarget}`);
//! 		},
//! 		flags: {},
//! 		name: "Forewarn",
//! 		rating: 0.5,
//! 		num: 108,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType, get_move};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Reveals the opponent's strongest move
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // let warnMoves: (Move | Pokemon)[] = [];
    // let warnBp = 1;
    let mut warn_moves: Vec<(ID, usize, usize)> = Vec::new(); // (move_id, foe_side, foe_position)
    let mut warn_bp = 1;

    // for (const target of pokemon.foes())
    let foe_side_index = 1 - pokemon.side_index;
    if let Some(foe_side) = battle.sides.get(foe_side_index) {
        for foe in foe_side.pokemon.iter().filter(|p| p.is_active && !p.fainted) {
            // for (const moveSlot of target.moveSlots)
            for move_slot in &foe.move_slots {
                // const move = this.dex.moves.get(moveSlot.move);
                if let Some(move_def) = get_move(&move_slot.id) {
                    // let bp = move.basePower;
                    let mut bp = move_def.base_power;

                    // if (move.ohko) bp = 150;
                    if move_def.ohko {
                        bp = 150;
                    }
                    // if (move.id === 'counter' || move.id === 'metalburst' || move.id === 'mirrorcoat') bp = 120;
                    else if move_def.id.as_str() == "counter" ||
                            move_def.id.as_str() == "metalburst" ||
                            move_def.id.as_str() == "mirrorcoat" {
                        bp = 120;
                    }
                    // if (!bp && move.category !== 'Status') bp = 80;
                    else if bp == 0 && move_def.category != MoveCategory::Status {
                        bp = 80;
                    }

                    // if (bp > warnBp)
                    if bp > warn_bp {
                        // warnMoves = [[move, target] as [Move, Pokemon]];
                        // warnBp = bp;
                        warn_moves.clear();
                        warn_moves.push((move_def.id.clone(), foe.side_index, foe.position));
                        warn_bp = bp;
                    // } else if (bp === warnBp) {
                    } else if bp == warn_bp {
                        // warnMoves.push([move, target]);
                        warn_moves.push((move_def.id.clone(), foe.side_index, foe.position));
                    }
                }
            }
        }
    }

    // if (!warnMoves.length) return;
    if warn_moves.is_empty() {
        return AbilityHandlerResult::Undefined;
    }

    // const [warnMoveName, warnTarget] = this.sample(warnMoves);
    let idx = battle.prng.random_int(warn_moves.len() as u32) as usize;
    let (warn_move_id, foe_side, foe_pos) = &warn_moves[idx];
    let warn_target = &battle.sides[*foe_side].pokemon[*foe_pos];

    // this.add('-activate', pokemon, 'ability: Forewarn', warnMoveName, `[of] ${warnTarget}`);
    battle.add("-activate", &[
        Arg::Pokemon(pokemon),
        Arg::Str("ability: Forewarn"),
        Arg::String(warn_move_id.to_string()),
        Arg::String(format!("[of] {}", warn_target.name))
    ]);

    AbilityHandlerResult::Undefined
}
