//! Armor Tail Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	armortail: {
//! 		onFoeTryMove(target, source, move) {
//! 			const targetAllExceptions = ['perishsong', 'flowershield', 'rototiller'];
//! 			if (move.target === 'foeSide' || (move.target === 'all' && !targetAllExceptions.includes(move.id))) {
//! 				return;
//! 			}
//! 
//! 			const armorTailHolder = this.effectState.target;
//! 			if ((source.isAlly(armorTailHolder) || move.target === 'all') && move.priority > 0.1) {
//! 				this.attrLastMove('[still]');
//! 				this.add('cant', armorTailHolder, 'ability: Armor Tail', move, `[of] ${target}`);
//! 				return false;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Armor Tail",
//! 		rating: 2.5,
//! 		num: 296,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onFoeTryMove(source, target, move)
    /// Blocks priority moves from opponents
    pub fn on_foe_try_move(battle: &mut Battle, source: &Pokemon, target: &Pokemon, move_: &MoveDef, armor_tail_holder: &Pokemon) -> AbilityHandlerResult {
        // Check if source is on same side as holder (ally) or move targets all, and has priority
        if (source.side_index == armor_tail_holder.side_index || move_.target == crate::data::moves::MoveTargetType::All) && move_.priority > 0 {
            battle.add("cant", &[Arg::Pokemon(armor_tail_holder), Arg::Str("ability: Armor Tail"), Arg::Str(&move_.name), Arg::Str(&format!("[of] {}", target.name))]);
            return AbilityHandlerResult::False;
        }
        AbilityHandlerResult::Undefined
    }
