//! Eerie Spell Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;

/// onHit(target)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(target) {
/// 				if (!target.hp) return;
/// 				let move: Move | ActiveMove | null = target.lastMove;
/// 				if (!move || move.isZ) return;
/// 				if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
///
/// 				const ppDeducted = target.deductPP(move.id, 3);
/// 				if (!ppDeducted) return;
/// 				this.add('-activate', target, 'move: Eerie Spell', move.name, ppDeducted);
/// 			},
///
/// 		}
/// ```
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (!target.hp) return;
    let target_hp = {
        let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.hp
    };

    if target_hp == 0 {
        return EventResult::Continue;
    }

    // let move: Move | ActiveMove | null = target.lastMove;
    // if (!move || move.isZ) return;
    let move_id = {
        let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };

        match &target_pokemon.last_move {
            Some(move_id) => move_id.clone(),
            None => return EventResult::Continue,
        }
    };

    // Get move data to check if it's a Z-move or Max move
    let move_data = match battle.dex.moves().get_by_id(&move_id) {
        Some(m) => m,
        None => return EventResult::Continue,
    };

    // if (!move || move.isZ) return;
    if move_data.is_z.is_some() {
        return EventResult::Continue;
    }

    // if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
    let actual_move_id = if move_data.is_max.is_some() {
        if let Some(ref base_move) = move_data.base_move {
            base_move.clone()
        } else {
            move_id.clone()
        }
    } else {
        move_id.clone()
    };

    // const ppDeducted = target.deductPP(move.id, 3);
    // if (!ppDeducted) return;
    let pp_deducted = {
        let gen = battle.gen;
        // Create ActiveMove before mutable borrow
        let active_move_for_pp = battle.dex.get_active_move(actual_move_id.as_str());
        let target_pokemon = match battle.pokemon_at_mut(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        if let Some(ref am) = active_move_for_pp {
            target_pokemon.deduct_pp(gen, am, Some(3))
        } else {
            0
        }
    };

    if pp_deducted == 0 {
        return EventResult::Continue;
    }

    // this.add('-activate', target, 'move: Eerie Spell', move.name, ppDeducted);
    let move_name = battle.dex.moves().get_by_id(&actual_move_id)
        .map(|m| m.name.clone())
        .unwrap_or_else(|| actual_move_id.to_string());

    let target_slot = {
        let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.get_slot()
    };

    battle.add(
        "-activate",
        &[
            crate::battle::Arg::from(target_slot),
            crate::battle::Arg::from("move: Eerie Spell"),
            crate::battle::Arg::from(move_name),
            crate::battle::Arg::from(pp_deducted.to_string()),
        ],
    );

    EventResult::Continue
}
