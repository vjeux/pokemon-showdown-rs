//! Power of Alchemy Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	powerofalchemy: {
//! 		onAllyFaint(target) {
//! 			if (!this.effectState.target.hp) return;
//! 			const ability = target.getAbility();
//! 			if (ability.flags['noreceiver'] || ability.id === 'noability') return;
//! 			this.effectState.target.setAbility(ability, target);
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1 },
//! 		name: "Power of Alchemy",
//! 		rating: 0,
//! 		num: 223,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onAllyFaint(target)
/// Copies the ability of a fainted ally
///
/// TODO: onAllyFaint handler not yet implemented
/// TODO: Needs effectState.target.hp, target.getAbility(), ability.flags['noreceiver'], effectState.target.setAbility()
/// When implemented, should:
/// 1. Skip if ability holder is fainted
/// 2. Get the fainted ally's ability
/// 3. Skip if ability has noreceiver flag or is noability
/// 4. Set the ability on this Pokemon
pub fn on_ally_faint(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

