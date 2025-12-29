//! G-Max Stonesurge Move
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
/// 				for (const side of source.side.foeSidesWithConditions()) {
/// 					side.addSideCondition('stealthrock');
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
    use crate::dex_data::ID;

    // for (const side of source.side.foeSidesWithConditions()) {
    //     side.addSideCondition('stealthrock');
    // }

    let source_side_index = source_pos.0;

    // Add stealth rock to all foe sides (sides that are not the source's side)
    for (side_idx, side) in battle.sides.iter_mut().enumerate() {
        if side_idx != source_side_index {
            side.add_side_condition(ID::from("stealthrock"), None);
        }
    }

    EventResult::Continue
}
