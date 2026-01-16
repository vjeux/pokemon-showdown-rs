//! G-Max Stonesurge Move
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
/// 				for (const side of source.side.foeSidesWithConditions()) {
/// 					side.addSideCondition('stealthrock');
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
    use crate::dex_data::ID;

    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // for (const side of source.side.foeSidesWithConditions()) {
    //     side.addSideCondition('stealthrock');
    // }

    let source_side_index = source.0;

    // Add stealth rock to all foe sides (sides that are not the source's side)
    // Use battle.add_side_condition to properly set effect_order for sorting
    let num_sides = battle.sides.len();
    for side_idx in 0..num_sides {
        if side_idx != source_side_index {
            battle.add_side_condition(
                side_idx,
                ID::from("stealthrock"),
                Some(source),
                None,
            );
        }
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
    ///                 for (const side of source.side.foeSidesWithConditions()) {
    ///                   side.addSideCondition("stealthrock");
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
        use crate::dex_data::ID;

        // for (const side of source.side.foeSidesWithConditions()) {
        //     side.addSideCondition("stealthrock");
        // }

        let source_side_index = source_pos.0;

        // Add stealth rock to all foe sides (sides that are not the source's side)
        // Use battle.add_side_condition to properly set effect_order for sorting
        let num_sides = battle.sides.len();
        for side_idx in 0..num_sides {
            if side_idx != source_side_index {
                battle.add_side_condition(
                    side_idx,
                    ID::from("stealthrock"),
                    Some(source_pos),
                    None,
                );
            }
        }

        EventResult::Continue
    }
}
