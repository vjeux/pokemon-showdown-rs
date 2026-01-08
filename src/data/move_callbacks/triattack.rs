//! Tri Attack Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(target, source)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(target, source) {
/// 			const result = this.random(3);
/// 			if (result === 0) {
/// 				target.trySetStatus('brn', source);
/// 			} else if (result === 1) {
/// 				target.trySetStatus('par', source);
/// 			} else {
/// 				target.trySetStatus('frz', source);
/// 			}
/// 		},
///
/// 	}
/// ```
pub fn on_hit(
    battle: &mut Battle,
    target_pos: (usize, usize),
    _source_pos: Option<(usize, usize)>,
) -> EventResult {
    // const result = this.random(3);
    // if (result === 0) {
    //     target.trySetStatus('brn', source);
    // } else if (result === 1) {
    //     target.trySetStatus('par', source);
    // } else {
    //     target.trySetStatus('frz', source);
    // }

    let result = battle.random(3);

    if result == 0 {
        Pokemon::try_set_status(battle, target_pos, ID::from("brn"), None);
    } else if result == 1 {
        Pokemon::try_set_status(battle, target_pos, ID::from("par"), None);
    } else {
        Pokemon::try_set_status(battle, target_pos, ID::from("frz"), None);
    }

    EventResult::Continue
}
