//! Receiver Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	receiver: {
//! 		onAllyFaint(target) {
//! 			if (!this.effectState.target.hp) return;
//! 			const ability = target.getAbility();
//! 			if (ability.flags['noreceiver'] || ability.id === 'noability') return;
//! 			this.effectState.target.setAbility(ability, target);
//! 		},
//! 		flags: { failroleplay: 1, noreceiver: 1, noentrain: 1, notrace: 1 },
//! 		name: "Receiver",
//! 		rating: 0,
//! 		num: 222,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onAllyFaint(...)
pub fn on_ally_faint(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

