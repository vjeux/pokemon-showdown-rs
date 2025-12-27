//! Eviolite Item
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/items.ts
//!
//! ```text
//! JS Source (data/items.ts):
//! 	eviolite: {
//! 		name: "Eviolite",
//! 		spritenum: 130,
//! 		fling: {
//! 			basePower: 40,
//! 		},
//! 		onModifyDefPriority: 2,
//! 		onModifyDef(def, pokemon) {
//! 			if (pokemon.baseSpecies.nfe) {
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		onModifySpDPriority: 2,
//! 		onModifySpD(spd, pokemon) {
//! 			if (pokemon.baseSpecies.nfe) {
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		num: 538,
//! 		gen: 5,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use crate::event::EventResult;

/// onModifyDefPriority(...)
pub fn on_modify_def_priority(battle: &mut Battle, /* TODO: Add parameters */) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifyDef(def, pokemon)
pub fn on_modify_def(battle: &Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // JS: if (pokemon.baseSpecies.nfe)
    if let Some(species) = battle.dex.get_species(pokemon.species_id.as_str()) {
        // Check if species can still evolve (nfe = not fully evolved)
        if !species.evos.is_empty() {
            // JS: return this.chainModify(1.5);
            return EventResult::Number(6144); // 1.5x in 4096 basis points
        }
    }
    EventResult::Continue
}

/// onModifySpDPriority(...)
pub fn on_modify_sp_d_priority(battle: &mut Battle, /* TODO: Add parameters */) -> EventResult {
    // TODO: Implement 1-to-1 from JS
    EventResult::Continue
}

/// onModifySpD(spd, pokemon)
pub fn on_modify_sp_d(battle: &Battle, pokemon_pos: (usize, usize)) -> EventResult {
    let pokemon = match battle.pokemon_at(pokemon_pos.0, pokemon_pos.1) {
        Some(p) => p,
        None => return EventResult::Continue,
    };

    // JS: if (pokemon.baseSpecies.nfe)
    if let Some(species) = battle.dex.get_species(pokemon.species_id.as_str()) {
        // Check if species can still evolve (nfe = not fully evolved)
        if !species.evos.is_empty() {
            // JS: return this.chainModify(1.5);
            return EventResult::Number(6144); // 1.5x in 4096 basis points
        }
    }
    EventResult::Continue
}
