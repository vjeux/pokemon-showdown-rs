//! Moody Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	moody: {
//! 		onResidualOrder: 28,
//! 		onResidualSubOrder: 2,
//! 		onResidual(pokemon) {
//! 			let stats: BoostID[] = [];
//! 			const boost: SparseBoostsTable = {};
//! 			let statPlus: BoostID;
//! 			for (statPlus in pokemon.boosts) {
//! 				if (statPlus === 'accuracy' || statPlus === 'evasion') continue;
//! 				if (pokemon.boosts[statPlus] < 6) {
//! 					stats.push(statPlus);
//! 				}
//! 			}
//! 			let randomStat: BoostID | undefined = stats.length ? this.sample(stats) : undefined;
//! 			if (randomStat) boost[randomStat] = 2;
//! 
//! 			stats = [];
//! 			let statMinus: BoostID;
//! 			for (statMinus in pokemon.boosts) {
//! 				if (statMinus === 'accuracy' || statMinus === 'evasion') continue;
//! 				if (pokemon.boosts[statMinus] > -6 && statMinus !== randomStat) {
//! 					stats.push(statMinus);
//! 				}
//! 			}
//! 			randomStat = stats.length ? this.sample(stats) : undefined;
//! 			if (randomStat) boost[randomStat] = -1;
//! 
//! 			this.boost(boost, pokemon, pokemon);
//! 		},
//! 		flags: {},
//! 		name: "Moody",
//! 		rating: 5,
//! 		num: 141,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_RESIDUAL_ORDER: i32 = 28;
pub const ON_RESIDUAL_SUB_ORDER: i32 = 2;

/// onResidual(pokemon)
/// Randomly raises one stat by 2 and lowers another by 1 each turn
///
/// TODO: onResidual handler not yet implemented
/// TODO: Needs pokemon.boosts iteration, random sampling
/// When implemented, should:
/// 1. Build list of stats (excluding accuracy/evasion) that can be raised (< 6)
/// 2. Randomly select one stat and boost it by +2
/// 3. Build list of stats (excluding accuracy/evasion) that can be lowered (> -6) and != raised stat
/// 4. Randomly select one stat and boost it by -1
/// 5. Apply both boosts with battle.boost()
pub fn on_residual(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

