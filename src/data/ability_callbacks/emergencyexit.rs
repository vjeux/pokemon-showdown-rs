//! Emergency Exit Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	emergencyexit: {
//! 		onEmergencyExit(target) {
//! 			if (!this.canSwitch(target.side) || target.forceSwitchFlag || target.switchFlag) return;
//! 			for (const side of this.sides) {
//! 				for (const active of side.active) {
//! 					active.switchFlag = false;
//! 				}
//! 			}
//! 			target.switchFlag = true;
//! 			this.add('-activate', target, 'ability: Emergency Exit');
//! 		},
//! 		flags: {},
//! 		name: "Emergency Exit",
//! 		rating: 1,
//! 		num: 194,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onEmergencyExit(target)
    pub fn on_emergency_exit(battle: &mut Battle, target: &mut Pokemon) -> AbilityHandlerResult {
        if battle.can_switch(target.side_index) == 0 || target.force_switch_flag || target.switch_flag {
            return AbilityHandlerResult::Undefined;
        }
        // Set switch flag on the target (other switch flags would be cleared in battle engine)
        target.switch_flag = true;
        battle.add("-activate", &[Arg::Pokemon(target), Arg::Str("ability: Emergency Exit")]);
        AbilityHandlerResult::Undefined
    }
