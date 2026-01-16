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
    ///
    /// NOTE: For self callbacks, the FIRST parameter receives the move USER (source),
    /// and the SECOND parameter receives the move TARGET (or None).
    /// The naming convention in dispatch_self_on_hit is misleading - it names them
    /// target_pos and source_pos, but actually passes source as first, target as second.
    pub fn on_hit(
        battle: &mut Battle,
        source_pos: (usize, usize),          // ACTUAL SOURCE (move user)
        _target_pos: Option<(usize, usize)>, // ACTUAL TARGET (move target)
        _source_effect: Option<&crate::battle::Effect>,
    ) -> EventResult {
        // for (const ally of source.side.pokemon) {
        //     ally.cureStatus();
        // }

        let source_side = source_pos.0;

        // Get the number of pokemon on the source's side
        let pokemon_count = battle.sides[source_side].pokemon.len();

        // Iterate through all pokemon on the side and cure their status
        for poke_idx in 0..pokemon_count {
            Pokemon::cure_status(battle, (source_side, poke_idx), false);
        }

        EventResult::Continue
    }
}
