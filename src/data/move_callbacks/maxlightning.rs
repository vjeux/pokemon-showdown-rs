//! Max Lightning Move
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
/// 				this.field.setTerrain('electricterrain');
/// 			},
///
/// 		}
/// ```
pub fn on_hit(battle: &mut Battle, source_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // if (!source.volatiles['dynamax']) return;
    let has_dynamax = {
        let source = match battle.pokemon_at(source_pos.0, source_pos.1) {
            Some(p) => p,
            None => return EventResult::Continue,
        };
        source.has_volatile(&ID::from("dynamax"))
    };

    if !has_dynamax {
        return EventResult::Continue;
    }

    // this.field.setTerrain('electricterrain');
    battle.set_terrain(ID::from("electricterrain"), None);

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
    ///                 this.field.setTerrain("electricterrain");
    ///               }
    ///     },
    /// }
    /// ```
    pub fn on_hit(
        battle: &mut Battle,
        _target_pos: (usize, usize),
        _source_pos: Option<(usize, usize)>,
    ) -> EventResult {
        // TODO: Implement 1-to-1 from JS
        EventResult::Continue
    }
}
