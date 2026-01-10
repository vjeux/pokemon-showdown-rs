//! G-Max Sweetness Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::event::EventResult;
use crate::Pokemon;

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
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
        _source_effect: Option<&crate::battle::Effect>,
) -> EventResult {
    // for (const ally of source.side.pokemon) {
    //     ally.cureStatus();
    // }

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let source_side = source.0;

    // Get the number of pokemon on the source's side
    let pokemon_count = battle.sides[source_side].pokemon.len();

    // Iterate through all pokemon on the side and cure their status
    for poke_idx in 0..pokemon_count {
        Pokemon::cure_status(battle, (source_side, poke_idx), false);
    }

    EventResult::Continue
}


/// Self-targeting callbacks
/// These callbacks target the move user (source), not the move target
pub mod self_callbacks {
    use super::*;

    /// self.onHit(source)
    ///
    /// ```text
    /// JS Source (data/moves.ts):
    /// self: {
    ///     onHit(source) {
    ///         onHit(source) {
    ///                 for (const ally of source.side.pokemon) {
    ///                   ally.cureStatus();
    ///                 }
    ///               }
    ///     },
    /// }
    /// ```
    pub fn on_hit(
        battle: &mut Battle,
        _target_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
        _source_effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        // for (const ally of source.side.pokemon) {
        //     ally.cureStatus();
        // }

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let source_side = source.0;

        // Get the number of pokemon on the source's side
        let pokemon_count = battle.sides[source_side].pokemon.len();

        // Iterate through all pokemon on the side and cure their status
        for poke_idx in 0..pokemon_count {
            Pokemon::cure_status(battle, (source_side, poke_idx), false);
        }

        EventResult::Continue
    }
}
