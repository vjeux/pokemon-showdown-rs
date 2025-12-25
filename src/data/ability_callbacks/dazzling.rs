//! Dazzling Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	dazzling: {
//! 		onFoeTryMove(target, source, move) {
//! 			const targetAllExceptions = ['perishsong', 'flowershield', 'rototiller'];
//! 			if (move.target === 'foeSide' || (move.target === 'all' && !targetAllExceptions.includes(move.id))) {
//! 				return;
//! 			}
//! 
//! 			const dazzlingHolder = this.effectState.target;
//! 			if ((source.isAlly(dazzlingHolder) || move.target === 'all') && move.priority > 0.1) {
//! 				this.attrLastMove('[still]');
//! 				this.add('cant', dazzlingHolder, 'ability: Dazzling', move, `[of] ${target}`);
//! 				return false;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Dazzling",
//! 		rating: 2.5,
//! 		num: 219,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// Moves that are exceptions to the priority blocking
    const TARGET_ALL_EXCEPTIONS: &[&str] = &["perishsong", "flowershield", "rototiller"];

    /// onFoeTryMove(target, source, move)
    /// Blocks priority moves used by opponents
    pub fn on_foe_try_move(battle: &mut Battle, dazzling_holder: &Pokemon, target: &Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        // if (move.target === 'foeSide' || (move.target === 'all' && !targetAllExceptions.includes(move.id)))
        if move_.target == MoveTargetType::FoeSide || (move_.target == MoveTargetType::All && !TARGET_ALL_EXCEPTIONS.contains(&move_.id.as_str())) {
            return AbilityHandlerResult::Undefined;
        }
        // if ((source.isAlly(dazzlingHolder) || move.target === 'all') && move.priority > 0.1)
        let is_ally = source.side_index == dazzling_holder.side_index;
        if (is_ally || move_.target == MoveTargetType::All) && move_.priority > 0 {
            // this.attrLastMove('[still]');
            // this.add('cant', dazzlingHolder, 'ability: Dazzling', move, `[of] ${target}`);
            battle.add("cant", &[
                Arg::Pokemon(dazzling_holder),
                Arg::Str("ability: Dazzling"),
                Arg::Str(move_.id.as_str()),
                Arg::Str(&format!("[of] {}", target.name)),
            ]);
            return AbilityHandlerResult::False;
        }
        AbilityHandlerResult::Undefined
    }
