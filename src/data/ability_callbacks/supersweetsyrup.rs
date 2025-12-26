//! Supersweet Syrup Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	supersweetsyrup: {
//! 		onStart(pokemon) {
//! 			if (pokemon.syrupTriggered) return;
//! 			pokemon.syrupTriggered = true;
//! 			this.add('-ability', pokemon, 'Supersweet Syrup');
//! 			for (const target of pokemon.adjacentFoes()) {
//! 				if (target.volatiles['substitute']) {
//! 					this.add('-immune', target);
//! 				} else {
//! 					this.boost({ evasion: -1 }, target, pokemon, null, true);
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Supersweet Syrup",
//! 		rating: 1.5,
//! 		num: 306,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Lowers Evasion of adjacent foes by 1 on switch-in (once per switch)
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (pokemon.syrupTriggered) return;
    let holder_ref = (pokemon.side_index, pokemon.position);
    if let Some(triggered_value) = battle.sides[holder_ref.0].pokemon[holder_ref.1].ability_state.data.get("syrupTriggered") {
        if triggered_value.as_bool() == Some(true) {
            return AbilityHandlerResult::Undefined;
        }
    }

    // pokemon.syrupTriggered = true;
    battle.sides[holder_ref.0].pokemon[holder_ref.1].ability_state.data.insert(
        "syrupTriggered".to_string(),
        serde_json::json!(true)
    );

    // this.add('-ability', pokemon, 'Supersweet Syrup');
    battle.add("-ability", &[
        Arg::Pokemon(pokemon),
        Arg::Str("Supersweet Syrup")
    ]);

    // for (const target of pokemon.adjacentFoes())
    let foe_side_index = 1 - pokemon.side_index;

    // Collect foe data first
    let mut foes: Vec<(usize, usize, bool, String)> = Vec::new(); // (side, position, has_substitute, name)

    if let Some(foe_side) = battle.sides.get(foe_side_index) {
        for foe in foe_side.pokemon.iter().filter(|p| p.is_active && !p.fainted) {
            // if (target.volatiles['substitute'])
            let has_substitute = foe.has_volatile(&ID::new("substitute"));
            foes.push((foe.side_index, foe.position, has_substitute, foe.name.clone()));
        }
    }

    // Apply evasion drop to foes
    for (side_idx, pos, has_substitute, target_name) in foes {
        if has_substitute {
            // this.add('-immune', target);
            battle.add("-immune", &[Arg::String(target_name)]);
        } else {
            // this.boost({ evasion: -1 }, target, pokemon, null, true);
            battle.boost(&[("evasion", -1)], (side_idx, pos), Some(holder_ref), None);
        }
    }

    AbilityHandlerResult::Undefined
}

