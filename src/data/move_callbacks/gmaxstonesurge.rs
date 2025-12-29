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
    _source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
    // for (const side of source.side.foeSidesWithConditions()) {
    //     side.addSideCondition('stealthrock');
    // }

    // TODO: Infrastructure needed - Battle::add_side_condition() method
    // This move needs to add stealth rock as a side condition to foe sides
    // For now, returning Continue as the infrastructure doesn't exist

    EventResult::Continue
}
