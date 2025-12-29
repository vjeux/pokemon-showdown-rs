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
    _source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (!target.hp) return;
    // let move: Move | ActiveMove | null = target.lastMove;
    // if (!move || move.isZ) return;
    // if (move.isMax && move.baseMove) move = this.dex.moves.get(move.baseMove);
    //
    // const ppDeducted = target.deductPP(move.id, 3);
    // if (!ppDeducted) return;
    // this.add('-activate', target, 'move: Eerie Spell', move.name, ppDeducted);

    let _target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // TODO: Infrastructure needed - Pokemon::deduct_pp() method
    // This move deducts 3 PP from the target's last used move
    // For now, returning Continue as the infrastructure doesn't exist

    EventResult::Continue
}
