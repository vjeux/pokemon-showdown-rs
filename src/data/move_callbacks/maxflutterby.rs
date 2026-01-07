//! Max Flutterby Move
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
/// 				for (const pokemon of source.foes()) {
/// 					this.boost({ spa: -1 }, pokemon);
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
    let source = match source_pos {
        Some(pos) => pos,
        None => return EventResult::Continue,
    };

    // if (!source.volatiles['dynamax']) return;
    let has_dynamax = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.has_volatile(&ID::from("dynamax"))
    };

    if !has_dynamax {
        return EventResult::Continue;
    }

    // for (const pokemon of source.foes()) {
    //     this.boost({ spa: -1 }, pokemon);
    // }
    let foe_positions = {
        let source_pokemon = match battle.pokemon_at(source.0, source.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source_pokemon.foes(battle, false)
    };

    for foe_pos in foe_positions {
        battle.boost(&[("spa", -1)], foe_pos, Some(source), None, false, false);
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
    ///                 if (!source.volatiles["dynamax"]) return;
    ///                 for (const pokemon of source.foes()) {
    ///                   this.boost({ spa: -1 }, pokemon);
    ///                 }
    ///               }
    ///     },
    /// }
    /// ```
    pub fn on_hit(
        battle: &mut Battle,
        _target_pos: (usize, usize),
        source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        let source = match source_pos {
            Some(pos) => pos,
            None => return EventResult::Continue,
        };

        // if (!source.volatiles["dynamax"]) return;
        let has_dynamax = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.has_volatile(&ID::from("dynamax"))
        };

        if !has_dynamax {
            return EventResult::Continue;
        }

        // for (const pokemon of source.foes()) {
        //     this.boost({ spa: -1 }, pokemon);
        // }
        let foe_positions = {
            let source_pokemon = match battle.pokemon_at(source.0, source.1) {
                Some(p) => p,
                None => return EventResult::Continue,
            };
            source_pokemon.foes(battle, false)
        };

        for foe_pos in foe_positions {
            battle.boost(&[("spa", -1)], foe_pos, Some(source), None, false, false);
        }

        EventResult::Continue
    }
}
