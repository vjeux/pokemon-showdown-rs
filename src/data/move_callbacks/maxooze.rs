//! Max Ooze Move
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
/// 				if (!source.volatiles['dynamax']) return;
/// 				for (const pokemon of source.alliesAndSelf()) {
/// 					this.boost({ spa: 1 }, pokemon);
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
    // if (!source.volatiles['dynamax']) return;
    let source_has_dynamax = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.has_volatile(&ID::from("dynamax"))
    };

    if !source_has_dynamax {
        return EventResult::Continue;
    }

    // for (const pokemon of source.alliesAndSelf()) {
    //     this.boost({ spa: 1 }, pokemon);
    // }
    let source_side = source_pos.0;

    let ally_positions: Vec<(usize, usize)> = battle
        .get_all_active(false)
        .into_iter()
        .filter(|(side_idx, _)| *side_idx == source_side)
        .collect();

    for ally_pos in ally_positions {
        battle.boost(&[("spa", 1)], ally_pos, Some(source_pos), None);
    }

    EventResult::Continue
}
