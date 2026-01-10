//! G-Max Sandblast Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::{Battle, Effect};
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onHit(source)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(source) {
/// 				for (const pokemon of source.foes()) {
/// 					pokemon.addVolatile('partiallytrapped', source, this.dex.getActiveMove('G-Max Sandblast'));
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
    // for (const pokemon of source.foes()) {
    //     pokemon.addVolatile('partiallytrapped', source, this.dex.getActiveMove('G-Max Sandblast'));
    // }

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    let foe_positions = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.foes(battle, false)
    };

    // JavaScript: this.dex.getActiveMove('G-Max Sandblast')
    // ID::from normalizes to lowercase alphanumeric, so this becomes "gmaxsandblast"
    let move_effect = Effect::move_(ID::from("G-Max Sandblast"));

    for foe_pos in foe_positions {
        Pokemon::add_volatile(battle, foe_pos, ID::from("partiallytrapped"), Some(source), Some(&move_effect), None, None);
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
    ///                 for (const pokemon of source.foes()) {
    ///                   pokemon.addVolatile("partiallytrapped", source, this.dex.getActiveMove("G-Max Sandblast"));
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
        // for (const pokemon of source.foes()) {
        //     pokemon.addVolatile("partiallytrapped", source, this.dex.getActiveMove("G-Max Sandblast"));
        // }

        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        let foe_positions = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.foes(battle, false)
        };

        // JavaScript: this.dex.getActiveMove("G-Max Sandblast")
        // ID::from normalizes to lowercase alphanumeric, so this becomes "gmaxsandblast"
        let move_effect = Effect::move_(ID::from("G-Max Sandblast"));

        for foe_pos in foe_positions {
            Pokemon::add_volatile(battle, foe_pos, ID::from("partiallytrapped"), Some(source), Some(&move_effect), None, None);
        }

        EventResult::Continue
    }
}
