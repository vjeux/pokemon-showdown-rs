//! Spite Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// ```ignore
/// onHit(target) {
///     let move: Move | ActiveMove | null = target.lastMove;
///     if (!move || move.isZ) return false;
///     if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
///
///     const ppDeducted = target.deductPP(move.id, 4);
///     if (!ppDeducted) return false;
///     this.add("-activate", target, 'move: Spite', move.name, ppDeducted);
/// }
/// ```
pub fn on_hit(
    battle: &mut Battle,
    _pokemon_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // onHit(target) {
    //     let move: Move | ActiveMove | null = target.lastMove;
    //     if (!move || move.isZ) return false;
    //     if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
    //
    //     const ppDeducted = target.deductPP(move.id, 4);
    //     if (!ppDeducted) return false;
    //     this.add("-activate", target, 'move: Spite', move.name, ppDeducted);
    // }
    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // let move: Move | ActiveMove | null = target.lastMove;
    // if (!move || move.isZ) return false;
    let (last_move_id, is_z, is_max, base_move) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        match &target_pokemon.last_move {
            Some(move_id) => {
                let move_data = battle.dex.moves().get_by_id(move_id);
                match move_data {
                    Some(m) => (
                        move_id.clone(),
                        m.is_z.clone(),
                        m.is_max.clone(),
                        m.base_move.clone(),
                    ),
                    None => return EventResult::Boolean(false),
                }
            }
            None => return EventResult::Boolean(false),
        }
    };

    if is_z.is_some() {
        return EventResult::Boolean(false);
    }

    // if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
    let move_id = if let (Some(_), Some(base)) = (is_max, base_move) {
        base
    } else {
        last_move_id
    };

    // const ppDeducted = target.deductPP(move.id, 4);
    // if (!ppDeducted) return false;
    let pp_deducted = {
        let gen = battle.gen;
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.deduct_pp(gen, &move_id, Some(4))
    };
    if pp_deducted == 0 {
        return EventResult::Boolean(false);
    }

    // this.add("-activate", target, 'move: Spite', move.name, ppDeducted);
    let (target_arg, move_name) = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        let move_data = battle.dex.moves().get_by_id(&move_id);
        let move_name = move_data.map(|m| m.name.clone()).unwrap_or_default();

        (target_pokemon.get_slot(), move_name)
    };

    battle.add(
        "-activate",
        &[
            target_arg.into(),
            "move: Spite".into(),
            move_name.into(),
            pp_deducted.to_string().into(),
        ],
    );

    EventResult::Continue
}
