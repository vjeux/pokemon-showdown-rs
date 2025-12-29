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
    battle.field.set_terrain(ID::from("psychicterrain"), None);

    EventResult::Continue
}


