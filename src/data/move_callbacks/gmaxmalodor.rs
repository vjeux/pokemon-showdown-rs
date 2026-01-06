//! G-Max Malodor Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::Pokemon;

/// onHit(source)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(source) {
/// 				for (const pokemon of source.foes()) {
/// 					pokemon.trySetStatus('psn', source);
/// 				}
/// 			},
///
/// 		}
/// ```
/// NOTE: JavaScript onHit(source) means this is a self-targeting move
/// but our dispatcher passes (target, source), so source is in second param
pub fn on_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    let source_pos = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // for (const pokemon of source.foes()) {
    //     pokemon.trySetStatus('psn', source);
    // }
    let foe_positions = {
        let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.foes(battle, false)
    };

    for foe_pos in foe_positions {
        Pokemon::try_set_status(battle, foe_pos, ID::from("psn"), None);
    }

    EventResult::Continue
}

