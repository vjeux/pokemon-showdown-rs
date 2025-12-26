//! Dauntless Shield Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	dauntlessshield: {
//! 		onStart(pokemon) {
//! 			if (pokemon.shieldBoost) return;
//! 			pokemon.shieldBoost = true;
//! 			this.boost({ def: 1 }, pokemon);
//! 		},
//! 		flags: {},
//! 		name: "Dauntless Shield",
//! 		rating: 3.5,
//! 		num: 235,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
    /// Raises Defense by 1 stage on first switch-in
    pub fn on_start(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
        // if (pokemon.shieldBoost) return;
        // Check if already boosted using ability state
        if pokemon.ability_state.data.get("shieldBoost").is_some() {
            return AbilityHandlerResult::Undefined;
        }
        // pokemon.shieldBoost = true;
        pokemon.ability_state.data.insert("shieldBoost".to_string(), serde_json::Value::Bool(true));
        // this.boost({ def: 1 }, pokemon);
        let pokemon_ref = (pokemon.side_index, pokemon.position);
        battle.boost(&[("def", 1)], pokemon_ref, Some(pokemon_ref), None);
        AbilityHandlerResult::Undefined
    }
