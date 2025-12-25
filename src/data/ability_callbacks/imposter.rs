//! Imposter Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	imposter: {
//! 		onSwitchIn(pokemon) {
//! 			// Imposter does not activate when Skill Swapped or when Neutralizing Gas leaves the field
//! 			// Imposter copies across in doubles/triples
//! 			// (also copies across in multibattle and diagonally in free-for-all,
//! 			// but side.foe already takes care of those)
//! 			const target = pokemon.side.foe.active[pokemon.side.foe.active.length - 1 - pokemon.position];
//! 			if (target) {
//! 				pokemon.transformInto(target, this.dex.abilities.get('imposter'));
//! 			}
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1 },
//! 		name: "Imposter",
//! 		rating: 5,
//! 		num: 150,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSwitchIn(...)
pub fn on_switch_in(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

