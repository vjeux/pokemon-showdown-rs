//! G-Max Malodor Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

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
pub fn on_hit(
    battle: &mut Battle,
    source_pos: (usize, usize),
    _target_pos: Option<(usize, usize)>,
) -> EventResult {
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
        if let Some(foe) = battle.pokemon_at_mut(foe_pos.0, foe_pos.1) {
            foe.try_set_status(ID::from("psn"), None);
        }
    }

    EventResult::Continue
}

