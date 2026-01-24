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
    target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // if (target?.statsRaisedThisTurn) {
    //     target.addVolatile('confusion', source, move);
    // }

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let stats_raised = {
        let target_pokemon = match battle.pokemon_at(target_pos.0, target_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        target_pokemon.stats_raised_this_turn
    };

    if stats_raised {
        // target.addVolatile('confusion', source, move);
        let move_effect = battle.make_move_effect(&ID::new("alluringvoice"));
        Pokemon::add_volatile(battle, target_pos, ID::from("confusion"), Some(source), Some(&move_effect), None, None);
    }

    EventResult::Continue
}
