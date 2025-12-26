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
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_RESIDUAL_ORDER: i32 = 28;
pub const ON_RESIDUAL_SUB_ORDER: i32 = 2;

/// onResidual(pokemon)
/// Randomly raises one stat by 2 and lowers another by 1 each turn
pub fn on_residual(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    let pokemon_loc = (pokemon.side_index, pokemon.position);

    // Build list of stats that can be raised (< 6), excluding accuracy/evasion
    let mut stats_to_raise = Vec::new();
    if pokemon.boosts.atk < 6 { stats_to_raise.push("atk"); }
    if pokemon.boosts.def < 6 { stats_to_raise.push("def"); }
    if pokemon.boosts.spa < 6 { stats_to_raise.push("spa"); }
    if pokemon.boosts.spd < 6 { stats_to_raise.push("spd"); }
    if pokemon.boosts.spe < 6 { stats_to_raise.push("spe"); }

    // Randomly select stat to raise
    let raised_stat = if !stats_to_raise.is_empty() {
        let idx = battle.random(stats_to_raise.len() as u32) as usize;
        Some(stats_to_raise[idx])
    } else {
        None
    };

    // Build list of stats that can be lowered (> -6), excluding accuracy/evasion and the raised stat
    let mut stats_to_lower = Vec::new();
    if pokemon.boosts.atk > -6 && Some("atk") != raised_stat { stats_to_lower.push("atk"); }
    if pokemon.boosts.def > -6 && Some("def") != raised_stat { stats_to_lower.push("def"); }
    if pokemon.boosts.spa > -6 && Some("spa") != raised_stat { stats_to_lower.push("spa"); }
    if pokemon.boosts.spd > -6 && Some("spd") != raised_stat { stats_to_lower.push("spd"); }
    if pokemon.boosts.spe > -6 && Some("spe") != raised_stat { stats_to_lower.push("spe"); }

    // Randomly select stat to lower
    let lowered_stat = if !stats_to_lower.is_empty() {
        let idx = battle.random(stats_to_lower.len() as u32) as usize;
        Some(stats_to_lower[idx])
    } else {
        None
    };

    // Build boost array
    let mut boosts = Vec::new();
    if let Some(stat) = raised_stat {
        boosts.push((stat, 2));
    }
    if let Some(stat) = lowered_stat {
        boosts.push((stat, -1));
    }

    // Apply boosts if any
    if !boosts.is_empty() {
        battle.boost(&boosts, pokemon_loc, Some(pokemon_loc), None);
    }

    AbilityHandlerResult::Undefined
}

