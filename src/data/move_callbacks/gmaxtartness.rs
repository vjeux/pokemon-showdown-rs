//! G-Max Tartness Move
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
/// 					this.boost({ evasion: -1 }, pokemon);
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
    //     this.boost({ evasion: -1 }, pokemon);
    // }
    let source_side = source_pos.0;
    let foe_side = 1 - source_side;

    let foe_positions: Vec<(usize, usize)> = battle
        .get_all_active(false)
        .into_iter()
        .filter(|(side_idx, _)| *side_idx == foe_side)
        .collect();

    for foe_pos in foe_positions {
        battle.boost(&[("evasion", -1)], foe_pos, Some(source_pos), None, false, false);
    }

    EventResult::Continue
}

