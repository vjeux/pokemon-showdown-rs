//! Forewarn Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onStart(pokemon) {
///     let warnMoves: (Move | Pokemon)[][] = [];
///     let warnBp = 1;
///     for (const target of pokemon.foes()) {
///         for (const moveSlot of target.moveSlots) {
///             const move = this.dex.moves.get(moveSlot.move);
///             let bp = move.basePower;
///             if (move.ohko) bp = 150;
///             if (move.id === 'counter' || move.id === 'metalburst' || move.id === 'mirrorcoat') bp = 120;
///             if (bp === 1) bp = 80;
///             if (!bp && move.category !== 'Status') bp = 80;
///             if (bp > warnBp) {
///                 warnMoves = [[move, target]];
///                 warnBp = bp;
///             } else if (bp === warnBp) {
///                 warnMoves.push([move, target]);
///             }
///         }
///     }
///     if (!warnMoves.length) return;
///     const [warnMoveName, warnTarget] = this.sample(warnMoves);
///     this.add('-activate', pokemon, 'ability: Forewarn', warnMoveName, `[of] ${warnTarget}`);
/// }
pub fn on_start(battle: &mut Battle, pokemon_pos: (usize, usize)) -> EventResult {
    use crate::battle::Arg;

    // let warnMoves: (Move | Pokemon)[][] = [];
    // let warnBp = 1;
    let mut warn_moves: Vec<(String, String)> = Vec::new(); // Vec of (move_name, target_slot)
    let mut warn_bp = 1;

    // for (const target of pokemon.foes())
    let foe_positions = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.foes(battle, false)
    };

    for foe_pos in foe_positions {
        // for (const moveSlot of target.moveSlots)
        let (foe_moves, target_slot) = {
            let foe = match battle.pokemon_at(foe_pos.0, foe_pos.1) {
                Some(p) => p,
                None => continue,
            };
            let mut moves_bp: Vec<(i32, String)> = Vec::new();
            for slot in &foe.move_slots {
                // const move = this.dex.moves.get(moveSlot.move);
                if let Some(move_data) = battle.dex.moves().get(slot.id.as_str()) {
                    // let bp = move.basePower;
                    let mut bp = move_data.base_power;

                    // if (move.ohko) bp = 150;
                    if move_data.ohko.is_some() {
                        bp = 150;
                    // if (move.id === 'counter' || move.id === 'metalburst' || move.id === 'mirrorcoat') bp = 120;
                    } else if slot.id.as_str() == "counter" || slot.id.as_str() == "metalburst" || slot.id.as_str() == "mirrorcoat" {
                        bp = 120;
                    // if (bp === 1) bp = 80;
                    } else if bp == 1 {
                        bp = 80;
                    // if (!bp && move.category !== 'Status') bp = 80;
                    } else if bp == 0 && move_data.category != "Status" {
                        bp = 80;
                    }

                    moves_bp.push((bp, move_data.name.clone()));
                }
            }
            (moves_bp, foe.get_slot())
        };

        // Process moves
        for (bp, move_name) in foe_moves {
            if bp > warn_bp {
                // warnMoves = [[move, target]];
                // warnBp = bp;
                warn_moves.clear();
                warn_moves.push((move_name, target_slot.clone()));
                warn_bp = bp;
            } else if bp == warn_bp && bp > 0 {
                // warnMoves.push([move, target]);
                warn_moves.push((move_name, target_slot.clone()));
            }
        }
    }

    // if (!warnMoves.length) return;
    if warn_moves.is_empty() {
        return EventResult::Continue;
    }

    // const [warnMoveName, warnTarget] = this.sample(warnMoves);
    let (warn_move_name, warn_target_slot) = {
        let indices: Vec<usize> = (0..warn_moves.len()).collect();
        let idx = match battle.sample(&indices) {
            Some(&i) => i,
            None => return EventResult::Continue,
        };
        (warn_moves[idx].0.clone(), warn_moves[idx].1.clone())
    };

    // this.add('-activate', pokemon, 'ability: Forewarn', warnMoveName, `[of] ${warnTarget}`);
    let pokemon_slot = {
        let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        pokemon.get_slot()
    };

    battle.add("-activate", &[
        Arg::String(pokemon_slot),
        Arg::Str("ability: Forewarn"),
        Arg::String(warn_move_name),
        Arg::String(format!("[of] {}", warn_target_slot)),
    ]);

    EventResult::Continue
}

