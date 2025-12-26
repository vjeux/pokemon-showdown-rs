//! Embody Aspect (Teal) Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	embodyaspectteal: {
//! 		onStart(pokemon) {
//! 			if (pokemon.baseSpecies.name === 'Ogerpon-Teal-Tera' && pokemon.terastallized &&
//! 				!this.effectState.embodied) {
//! 				this.effectState.embodied = true;
//! 				this.boost({ spe: 1 }, pokemon);
//! 			}
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1, failskillswap: 1, notransform: 1 },
//! 		name: "Embody Aspect (Teal)",
//! 		rating: 3.5,
//! 		num: 301,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
    pub fn on_start(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
        // Check if Ogerpon-Teal-Tera and terastallized
        if pokemon.species_id == ID::new("ogerpontealtera")
            && pokemon.terastallized.is_some()
            && !pokemon.ability_state.data.contains_key("embodied")
        {
            pokemon.ability_state.data.insert("embodied".to_string(), serde_json::Value::Bool(true));
            let pokemon_ref = (pokemon.side_index, pokemon.position);
            battle.boost(&[("spe", 1)], pokemon_ref, None, None);
        }
        AbilityHandlerResult::Undefined
    }
