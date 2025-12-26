//! Wind Rider Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	windrider: {
//! 		onStart(pokemon) {
//! 			if (pokemon.side.sideConditions['tailwind']) {
//! 				this.boost({ atk: 1 }, pokemon, pokemon);
//! 			}
//! 		},
//! 		onTryHit(target, source, move) {
//! 			if (target !== source && move.flags['wind']) {
//! 				if (!this.boost({ atk: 1 }, target, target)) {
//! 					this.add('-immune', target, '[from] ability: Wind Rider');
//! 				}
//! 				return null;
//! 			}
//! 		},
//! 		onSideConditionStart(side, source, sideCondition) {
//! 			const pokemon = this.effectState.target;
//! 			if (sideCondition.id === 'tailwind') {
//! 				this.boost({ atk: 1 }, pokemon, pokemon);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Wind Rider",
//! 		rating: 3.5,
//! 		num: 274,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onStart(pokemon)
/// Boosts Attack if Tailwind is active
pub fn on_start(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (pokemon.side.sideConditions['tailwind'])
    if battle.sides[pokemon.side_index].has_side_condition(&ID::new("tailwind")) {
        // this.boost({ atk: 1 }, pokemon, pokemon);
        let pokemon_ref = (pokemon.side_index, pokemon.position);
        battle.boost(&[("atk", 1)], pokemon_ref, Some(pokemon_ref), None);
    }
    AbilityHandlerResult::Undefined
}

/// onTryHit(target, source, move)
/// Immune to wind moves and boosts Attack
pub fn on_try_hit(battle: &mut Battle, target: &mut Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (target !== source && move.flags['wind'])
    if (target.side_index != source.side_index || target.position != source.position) && move_.flags.wind {
        // if (!this.boost({ atk: 1 }, target, target))
        let target_ref = (target.side_index, target.position);
        if !battle.boost(&[("atk", 1)], target_ref, Some(target_ref), None) {
            // this.add('-immune', target, '[from] ability: Wind Rider');
            battle.add("-immune", &[Arg::Pokemon(target), Arg::Str("[from] ability: Wind Rider")]);
        }
        // return null;
        return AbilityHandlerResult::Null;
    }
    AbilityHandlerResult::Undefined
}

/// onSideConditionStart(side, source, sideCondition)
/// Boosts Attack when Tailwind starts
pub fn on_side_condition_start(battle: &mut Battle, side_index: usize, _source: Option<&Pokemon>, side_condition_id: &str, pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (sideCondition.id === 'tailwind')
    if side_condition_id == "tailwind" {
        // this.boost({ atk: 1 }, pokemon, pokemon);
        let pokemon_ref = (pokemon.side_index, pokemon.position);
        battle.boost(&[("atk", 1)], pokemon_ref, Some(pokemon_ref), None);
    }
    AbilityHandlerResult::Undefined
}
