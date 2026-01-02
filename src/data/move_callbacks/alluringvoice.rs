//! Alluring Voice Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

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
    source_pos: (usize, usize),
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
        // target.addVolatile('confusion', source, move);
        // JavaScript: target.addVolatile('confusion', source, move)
        // ✅ NOW PASSING: source_pos = Some(source_pos), source_effect = Some("alluringvoice"), linked_status = None
        Pokemon::add_volatile(
            battle,
            target,
            ID::from("confusion"),
            Some(source_pos),
            Some(&ID::new("alluringvoice")),
            None,
        );
    }

    EventResult::Continue
}
