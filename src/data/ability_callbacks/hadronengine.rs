//! Hadron Engine Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	hadronengine: {
//! 		onStart(pokemon) {
//! 			if (!this.field.setTerrain('electricterrain') && this.field.isTerrain('electricterrain')) {
//! 				this.add('-activate', pokemon, 'ability: Hadron Engine');
//! 			}
//! 		},
//! 		onModifySpAPriority: 5,
//! 		onModifySpA(atk, attacker, defender, move) {
//! 			if (this.field.isTerrain('electricterrain')) {
//! 				this.debug('Hadron Engine boost');
//! 				return this.chainModify([5461, 4096]);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Hadron Engine",
//! 		rating: 4.5,
//! 		num: 289,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_SPA_PRIORITY: i32 = 5;

/// onStart(pokemon)
/// Sets Electric Terrain on switch-in
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (!this.field.setTerrain('electricterrain') && this.field.isTerrain('electricterrain'))
    let terrain_changed = battle.field.set_terrain(ID::new("electricterrain"), None);
    if !terrain_changed && battle.field.is_terrain("electricterrain") {
        // this.add('-activate', pokemon, 'ability: Hadron Engine');
        battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Hadron Engine")]);
    }
    AbilityHandlerResult::Undefined
}

/// onModifySpA(atk, attacker, defender, move)
/// Boosts Special Attack by 1.3330... (5461/4096) in Electric Terrain
pub fn on_modify_sp_a(battle: &mut Battle, _spa: u32, _attacker: &Pokemon, _defender: Option<&Pokemon>, _move: Option<&MoveDef>) -> AbilityHandlerResult {
    // if (this.field.isTerrain('electricterrain'))
    if battle.field.is_terrain("electricterrain") {
        // this.debug('Hadron Engine boost');
        // return this.chainModify([5461, 4096]);
        return AbilityHandlerResult::ChainModify(5461, 4096); // ~1.333x
    }
    AbilityHandlerResult::Undefined
}
