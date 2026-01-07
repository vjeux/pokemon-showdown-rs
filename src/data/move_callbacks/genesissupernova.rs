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
pub fn on_hit(battle: &mut Battle, _pokemon_pos: (usize, usize), _target_pos: Option<(usize, usize)>) -> EventResult {
    // this.field.setTerrain('psychicterrain');
    let source_effect = Some(crate::battle::Effect::move_("genesissupernova"));
    battle.set_terrain(ID::from("psychicterrain"), None, source_effect);

    EventResult::Continue
}


