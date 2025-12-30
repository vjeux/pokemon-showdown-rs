//! Alluring Voice Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onHit(target, source, move)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(target, source, move) {
/// 				if (target?.statsRaisedThisTurn) {
/// 					target.addVolatile('confusion', source, move);
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
    // if (target?.statsRaisedThisTurn) {
    //     target.addVolatile('confusion', source, move);
    // }

    let target = match target_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let stats_raised = {
        let target_pokemon = match battle.pokemon_at(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.stats_raised_this_turn
    };

    if stats_raised {
        let target_pokemon = match battle.pokemon_at_mut(target.0, target.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.add_volatile(ID::from("confusion"));
    }

    EventResult::Continue
}
