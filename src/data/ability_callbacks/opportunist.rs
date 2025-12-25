//! Opportunist Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	opportunist: {
//! 		onFoeAfterBoost(boost, target, source, effect) {
//! 			if (effect?.name === 'Opportunist' || effect?.name === 'Mirror Herb') return;
//! 			if (!this.effectState.boosts) this.effectState.boosts = {} as SparseBoostsTable;
//! 			const boostPlus = this.effectState.boosts;
//! 			let i: BoostID;
//! 			for (i in boost) {
//! 				if (boost[i]! > 0) {
//! 					boostPlus[i] = (boostPlus[i] || 0) + boost[i]!;
//! 				}
//! 			}
//! 		},
//! 		onAnySwitchInPriority: -3,
//! 		onAnySwitchIn() {
//! 			if (!this.effectState.boosts) return;
//! 			this.boost(this.effectState.boosts, this.effectState.target);
//! 			delete this.effectState.boosts;
//! 		},
//! 		onAnyAfterMega() {
//! 			if (!this.effectState.boosts) return;
//! 			this.boost(this.effectState.boosts, this.effectState.target);
//! 			delete this.effectState.boosts;
//! 		},
//! 		onAnyAfterTerastallization() {
//! 			if (!this.effectState.boosts) return;
//! 			this.boost(this.effectState.boosts, this.effectState.target);
//! 			delete this.effectState.boosts;
//! 		},
//! 		onAnyAfterMove() {
//! 			if (!this.effectState.boosts) return;
//! 			this.boost(this.effectState.boosts, this.effectState.target);
//! 			delete this.effectState.boosts;
//! 		},
//! 		onResidualOrder: 29,
//! 		onResidual(pokemon) {
//! 			if (!this.effectState.boosts) return;
//! 			this.boost(this.effectState.boosts, this.effectState.target);
//! 			delete this.effectState.boosts;
//! 		},
//! 		onEnd() {
//! 			delete this.effectState.boosts;
//! 		},
//! 		flags: {},
//! 		name: "Opportunist",
//! 		rating: 3,
//! 		num: 290,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// Complex ability - copies positive stat boosts from opponents
/// TODO: Needs effectState.boosts tracking across multiple handlers
/// TODO: onFoeAfterBoost, onAnySwitchIn, onAnyAfterMega, onAnyAfterTerastallization, onAnyAfterMove, onResidual, onEnd
pub const ON_ANY_SWITCH_IN_PRIORITY: i32 = -3;
pub const ON_RESIDUAL_ORDER: i32 = 29;

