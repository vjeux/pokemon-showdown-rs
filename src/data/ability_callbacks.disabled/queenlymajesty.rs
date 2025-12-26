//! Queenly Majesty Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	queenlymajesty: {
//! 		onFoeTryMove(target, source, move) {
//! 			const targetAllExceptions = ['perishsong', 'flowershield', 'rototiller'];
//! 			if (move.target === 'foeSide' || (move.target === 'all' && !targetAllExceptions.includes(move.id))) {
//! 				return;
//! 			}
//! 
//! 			const dazzlingHolder = this.effectState.target;
//! 			if ((source.isAlly(dazzlingHolder) || move.target === 'all') && move.priority > 0.1) {
//! 				this.attrLastMove('[still]');
//! 				this.add('cant', dazzlingHolder, 'ability: Queenly Majesty', move, `[of] ${target}`);
//! 				return false;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Queenly Majesty",
//! 		rating: 2.5,
//! 		num: 214,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onFoeTryMove(target, source, move)
/// Blocks priority moves from opponents and allies (similar to Dazzling)
pub fn on_foe_try_move(battle: &mut Battle, target: &Pokemon, source: &Pokemon, move_: &MoveDef, ability_holder: &Pokemon) -> AbilityHandlerResult {
    // const targetAllExceptions = ['perishsong', 'flowershield', 'rototiller'];
    const TARGET_ALL_EXCEPTIONS: &[&str] = &["perishsong", "flowershield", "rototiller"];

    // if (move.target === 'foeSide' || (move.target === 'all' && !targetAllExceptions.includes(move.id)))
    if move_.target == MoveTargetType::FoeSide
        || (move_.target == MoveTargetType::All && !TARGET_ALL_EXCEPTIONS.contains(&move_.id.as_str())) {
        // return;
        return AbilityHandlerResult::Undefined;
    }

    // const dazzlingHolder = this.effectState.target;
    let dazzling_holder = ability_holder;

    // if ((source.isAlly(dazzlingHolder) || move.target === 'all') && move.priority > 0.1)
    if (source.is_ally(dazzling_holder.side_index) || move_.target == MoveTargetType::All) && move_.priority > 0 {
        // this.attrLastMove('[still]');
        // this.add('cant', dazzlingHolder, 'ability: Queenly Majesty', move, `[of] ${target}`);
        battle.add("-cant", &[
            Arg::Pokemon(dazzling_holder),
            Arg::Str("ability: Queenly Majesty"),
            Arg::Str(&move_.id.to_string()),
            Arg::String(format!("[of] {}", target.get_slot()))
        ]);
        // return false;
        return AbilityHandlerResult::False;
    }

    AbilityHandlerResult::Undefined
}

