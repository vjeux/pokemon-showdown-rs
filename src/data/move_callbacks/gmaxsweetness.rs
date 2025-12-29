//! G-Max Sweetness Move
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
/// 				for (const ally of source.side.pokemon) {
/// 					ally.cureStatus();
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
    // for (const ally of source.side.pokemon) {
    //     ally.cureStatus();
    // }
    let source_side = source_pos.0;

    // Get the number of pokemon on the source's side
    let pokemon_count = battle.sides[source_side].pokemon.len();

    // Iterate through all pokemon on the side and cure their status
    for poke_idx in 0..pokemon_count {
        if let Some(ally) = battle.pokemon_at_mut(source_side, poke_idx) {
            ally.cure_status();
        }
    }

    EventResult::Continue
}

