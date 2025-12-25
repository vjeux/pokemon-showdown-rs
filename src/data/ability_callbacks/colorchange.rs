//! Color Change Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	colorchange: {
//! 		onAfterMoveSecondary(target, source, move) {
//! 			if (!target.hp) return;
//! 			const type = move.type;
//! 			if (
//! 				target.isActive && move.effectType === 'Move' && move.category !== 'Status' &&
//! 				type !== '???' && !target.hasType(type)
//! 			) {
//! 				if (!target.setType(type)) return false;
//! 				this.add('-start', target, 'typechange', type, '[from] ability: Color Change');
//! 
//! 				if (target.side.active.length === 2 && target.position === 1) {
//! 					// Curse Glitch
//! 					const action = this.queue.willMove(target);
//! 					if (action && action.move.id === 'curse') {
//! 						action.targetLoc = -1;
//! 					}
//! 				}
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Color Change",
//! 		rating: 0,
//! 		num: 16,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onAfterMoveSecondary(target, source, move)
    /// Changes the user's type to the type of the move that hit it
    pub fn on_after_move_secondary(battle: &mut Battle, target: &mut Pokemon, _source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        // if (!target.hp) return;
        if target.hp == 0 {
            return AbilityHandlerResult::Undefined;
        }
        // const type = move.type;
        let move_type = &move_.move_type;
        // if (target.isActive && move.effectType === 'Move' && move.category !== 'Status' &&
        //     type !== '???' && !target.hasType(type))
        if target.is_active && move_.category != MoveCategory::Status
            && move_type != "???" && !target.has_type(move_type)
        {
            // if (!target.setType(type)) return false;
            // In Rust, set_type takes Vec<String> and returns ()
            target.set_type(vec![move_type.clone()]);
            // this.add('-start', target, 'typechange', type, '[from] ability: Color Change');
            battle.add("-start", &[
                Arg::Pokemon(target),
                Arg::Str("typechange"),
                Arg::Str(move_type),
                Arg::Str("[from] ability: Color Change"),
            ]);
            // Curse glitch handling is complex and doubles-only, skipped for now
        }
        AbilityHandlerResult::Undefined
    }
