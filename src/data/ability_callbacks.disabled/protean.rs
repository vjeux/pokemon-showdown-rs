//! Protean Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	protean: {
//! 		onPrepareHit(source, target, move) {
//! 			if (this.effectState.protean) return;
//! 			if (move.hasBounced || move.flags['futuremove'] || move.sourceEffect === 'snatch' || move.callsMove) return;
//! 			const type = move.type;
//! 			if (type && type !== '???' && source.getTypes().join() !== type) {
//! 				if (!source.setType(type)) return;
//! 				this.effectState.protean = true;
//! 				this.add('-start', source, 'typechange', type, '[from] ability: Protean');
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Protean",
//! 		rating: 4,
//! 		num: 168,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onPrepareHit(source, target, move)
/// Changes user's type to match move type before attacking
///
/// TODO: onPrepareHit handler not yet implemented
/// TODO: Needs effectState.protean, move.hasBounced, move.flags['futuremove'], move.sourceEffect, move.callsMove, source.getTypes(), source.setType()
/// When implemented, should:
/// 1. Skip if already activated this turn (effectState.protean)
/// 2. Skip if move hasBounced, is futuremove, from snatch, or callsMove
/// 3. Get move type, skip if ??? or already that type
/// 4. Set user type to move type
/// 5. Set effectState.protean = true to prevent multiple activations
/// 6. Add typechange message
pub fn on_prepare_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

