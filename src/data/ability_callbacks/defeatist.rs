//! Defeatist Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	defeatist: {
//! 		onModifyAtkPriority: 5,
//! 		onModifyAtk(atk, pokemon) {
//! 			if (pokemon.hp <= pokemon.maxhp / 2) {
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		onModifySpAPriority: 5,
//! 		onModifySpA(atk, pokemon) {
//! 			if (pokemon.hp <= pokemon.maxhp / 2) {
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Defeatist",
//! 		rating: -1,
//! 		num: 129,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onModifyAtkPriority: 5
    pub const ON_MODIFY_ATK_PRIORITY: i32 = 5;
    /// onModifySpAPriority: 5
    pub const ON_MODIFY_SPA_PRIORITY: i32 = 5;

    /// onModifyAtk(atk, pokemon)
    /// Halves Attack when HP is at or below 50%
    pub fn on_modify_atk(_atk: i32, pokemon: &Pokemon) -> AbilityHandlerResult {
        // if (pokemon.hp <= pokemon.maxhp / 2)
        if pokemon.hp <= pokemon.maxhp / 2 {
            // return this.chainModify(0.5);
            return AbilityHandlerResult::ChainModify(2048, 4096);
        }
        AbilityHandlerResult::Undefined
    }

    /// onModifySpA(atk, pokemon)
    /// Halves Special Attack when HP is at or below 50%
    pub fn on_modify_sp_a(_spa: i32, pokemon: &Pokemon) -> AbilityHandlerResult {
        // if (pokemon.hp <= pokemon.maxhp / 2)
        if pokemon.hp <= pokemon.maxhp / 2 {
            // return this.chainModify(0.5);
            return AbilityHandlerResult::ChainModify(2048, 4096);
        }
        AbilityHandlerResult::Undefined
    }
