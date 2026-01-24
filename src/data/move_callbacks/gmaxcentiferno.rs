//! G-Max Centiferno Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;
use crate::pokemon::Pokemon;

/// onHit(source)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit(source) {
/// 				for (const pokemon of source.foes()) {
/// 					pokemon.addVolatile('partiallytrapped', source, this.dex.getActiveMove('G-Max Centiferno'));
/// 				}
/// 			},
///
/// 		}
/// ```
pub fn on_hit(
    battle: &mut Battle,
    _target_pos: (usize, usize),
    source_pos: Option<(usize, usize)>,
) -> EventResult {
    // for (const pokemon of source.foes()) {
    //     pokemon.addVolatile('partiallytrapped', source, this.dex.getActiveMove('G-Max Centiferno'));
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

    // JavaScript: this.dex.getActiveMove('G-Max Centiferno')
    // ID::from normalizes to lowercase alphanumeric, so this becomes "gmaxcentiferno"
    let move_effect = battle.make_move_effect(&ID::from("gmaxcentiferno"));

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
    ///                   pokemon.addVolatile("partiallytrapped", source, this.dex.getActiveMove("G-Max Centiferno"));
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
        // for (const pokemon of source.foes()) {
        //     pokemon.addVolatile("partiallytrapped", source, this.dex.getActiveMove("G-Max Centiferno"));
        // }

        let foe_positions = {
            let source_pokemon = match battle.pokemon_at(source_pos.0, source_pos.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.foes(battle, false)
        };

        // JavaScript: this.dex.getActiveMove("G-Max Centiferno")
        // ID::from normalizes to lowercase alphanumeric, so this becomes "gmaxcentiferno"
        let move_effect = battle.make_move_effect(&ID::from("gmaxcentiferno"));

        for foe_pos in foe_positions {
            Pokemon::add_volatile(battle, foe_pos, ID::from("partiallytrapped"), Some(source_pos), Some(&move_effect), None, None);
        }

        EventResult::Continue
    }
}
