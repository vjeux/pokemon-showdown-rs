//! Genesis Supernova Move
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/moves.ts

use crate::battle::Battle;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onHit(...)
///
/// ```text
/// JS Source (data/moves.ts):
/// onHit() {
/// 					this.field.setTerrain('psychicterrain');
/// 				},
/// 			},
///
/// 		}
/// ```
pub fn on_hit(battle: &mut Battle, _target_pos: (usize, usize), source_pos: Option<(usize, usize)>) -> EventResult {
    // onHit() { this.field.setTerrain('psychicterrain'); }
    // target_pos = Pokemon hit by the move (not used)
    // source_pos = Pokemon using the move (used as terrain source)
    let source_effect = Some(crate::battle::Effect::move_("genesissupernova"));
    battle.set_terrain(ID::from("psychicterrain"), source_pos, source_effect);

    EventResult::Continue
}


