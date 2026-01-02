//! Dire Claw Move
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
/// 				const result = this.random(3);
/// 				if (result === 0) {
/// 					target.trySetStatus('psn', source);
/// 				} else if (result === 1) {
/// 					target.trySetStatus('par', source);
/// 				} else {
/// 					target.trySetStatus('slp', source);
/// 				}
/// 			},
///
/// 		}
/// ```
pub fn on_hit(
    battle: &mut Battle,
    _source_pos: (usize, usize),
    target_pos: Option<(usize, usize)>,
) -> EventResult {
    // const result = this.random(3);
    // if (result === 0) {
    //     target.trySetStatus('psn', source);
    // } else if (result === 1) {
    //     target.trySetStatus('par', source);
    // } else {
    //     target.trySetStatus('slp', source);
    // }

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let result = battle.random(3);

    if let Some(target_pokemon) = battle.pokemon_at_mut(target.0, target.1) {
        if result == 0 {
            Pokemon::try_set_status(battle, target, ID::from("psn"), None);
        } else if result == 1 {
            Pokemon::try_set_status(battle, target, ID::from("par"), None);
        } else {
            Pokemon::try_set_status(battle, target, ID::from("slp"), None);
        }
    }

    EventResult::Continue
}
