//! Analytic Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	analytic: {
//! 		onBasePowerPriority: 21,
//! 		onBasePower(basePower, pokemon) {
//! 			let boosted = true;
//! 			for (const target of this.getAllActive()) {
//! 				if (target === pokemon) continue;
//! 				if (this.queue.willMove(target)) {
//! 					boosted = false;
//! 					break;
//! 				}
//! 			}
//! 			if (boosted) {
//! 				this.debug('Analytic boost');
//! 				return this.chainModify([5325, 4096]);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Analytic",
//! 		rating: 2.5,
//! 		num: 148,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

// onBasePowerPriority: 21,
    pub const ON_BASE_POWER_PRIORITY: i32 = 21;

    /// onBasePower(basePower, pokemon)
    pub fn on_base_power(battle: &mut Battle, _base_power: u32, pokemon: &Pokemon) -> AbilityHandlerResult {
        // let boosted = true;
        let mut boosted = true;
        // for (const target of this.getAllActive())
        for (side_idx, slot, target) in battle.get_all_active(false) {
            // if (target === pokemon) continue;
            if target.is_same(pokemon) {
                continue;
            }
            // if (this.queue.willMove(target))
            if battle.queue.will_move(side_idx, slot).is_some() {
                // boosted = false;
                boosted = false;
                // break;
                break;
            }
        }
        // if (boosted)
        if boosted {
            // this.debug('Analytic boost');
            battle.debug("Analytic boost");
            // return this.chainModify([5325, 4096]);
            return AbilityHandlerResult::ChainModify(5325, 4096); // ~1.3x
        }
        AbilityHandlerResult::Undefined
    }
