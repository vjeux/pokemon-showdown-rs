//! Mimicry Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	mimicry: {
//! 		onSwitchInPriority: -1,
//! 		onStart(pokemon) {
//! 			this.singleEvent('TerrainChange', this.effect, this.effectState, pokemon);
//! 		},
//! 		onTerrainChange(pokemon) {
//! 			let types;
//! 			switch (this.field.terrain) {
//! 			case 'electricterrain':
//! 				types = ['Electric'];
//! 				break;
//! 			case 'grassyterrain':
//! 				types = ['Grass'];
//! 				break;
//! 			case 'mistyterrain':
//! 				types = ['Fairy'];
//! 				break;
//! 			case 'psychicterrain':
//! 				types = ['Psychic'];
//! 				break;
//! 			default:
//! 				types = pokemon.baseSpecies.types;
//! 			}
//! 			const oldTypes = pokemon.getTypes();
//! 			if (oldTypes.join() === types.join() || !pokemon.setType(types)) return;
//! 			if (this.field.terrain || pokemon.transformed) {
//! 				this.add('-start', pokemon, 'typechange', types.join('/'), '[from] ability: Mimicry');
//! 				if (!this.field.terrain) this.hint("Transform Mimicry changes you to your original un-transformed types.");
//! 			} else {
//! 				this.add('-activate', pokemon, 'ability: Mimicry');
//! 				this.add('-end', pokemon, 'typechange', '[silent]');
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Mimicry",
//! 		rating: 0,
//! 		num: 250,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSwitchInPriority(...)
pub fn on_switch_in_priority(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onStart(...)
pub fn on_start(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onTerrainChange(...)
pub fn on_terrain_change(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

