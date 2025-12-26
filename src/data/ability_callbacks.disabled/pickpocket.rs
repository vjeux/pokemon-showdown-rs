//! Pickpocket Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	pickpocket: {
//! 		onAfterMoveSecondary(target, source, move) {
//! 			if (source && source !== target && move?.flags['contact']) {
//! 				if (target.item || target.switchFlag || target.forceSwitchFlag || source.switchFlag === true) {
//! 					return;
//! 				}
//! 				const yourItem = source.takeItem(target);
//! 				if (!yourItem) {
//! 					return;
//! 				}
//! 				if (!target.setItem(yourItem)) {
//! 					source.item = yourItem.id;
//! 					return;
//! 				}
//! 				this.add('-enditem', source, yourItem, '[silent]', '[from] ability: Pickpocket', `[of] ${source}`);
//! 				this.add('-item', target, yourItem, '[from] ability: Pickpocket', `[of] ${source}`);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Pickpocket",
//! 		rating: 1,
//! 		num: 124,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onAfterMoveSecondary(target, source, move)
/// Steals the attacker's item when hit by a contact move
///
/// TODO: onAfterMoveSecondary handler not yet implemented
/// TODO: Needs move.flags['contact'], target.item, switchFlag, forceSwitchFlag, takeItem(), setItem()
/// When implemented, should:
/// 1. Skip if no source, source === target, or move doesn't have contact flag
/// 2. Skip if target has item, or either Pokemon is switching
/// 3. Take item from source using takeItem()
/// 4. If no item taken, return
/// 5. Try to set item on target
/// 6. If setItem fails, restore source.item and return
/// 7. Add enditem message for source and item message for target
pub fn on_after_move_secondary(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

