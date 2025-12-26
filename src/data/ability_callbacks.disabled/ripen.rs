//! Ripen Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	ripen: {
//! 		onTryHeal(damage, target, source, effect) {
//! 			if (!effect) return;
//! 			if (effect.name === 'Berry Juice' || effect.name === 'Leftovers') {
//! 				this.add('-activate', target, 'ability: Ripen');
//! 			}
//! 			if ((effect as Item).isBerry) return this.chainModify(2);
//! 		},
//! 		onChangeBoost(boost, target, source, effect) {
//! 			if (effect && (effect as Item).isBerry) {
//! 				let b: BoostID;
//! 				for (b in boost) {
//! 					boost[b]! *= 2;
//! 				}
//! 			}
//! 		},
//! 		onSourceModifyDamagePriority: -1,
//! 		onSourceModifyDamage(damage, source, target, move) {
//! 			if (target.abilityState.berryWeaken) {
//! 				target.abilityState.berryWeaken = false;
//! 				return this.chainModify(0.5);
//! 			}
//! 		},
//! 		onTryEatItemPriority: -1,
//! 		onTryEatItem(item, pokemon) {
//! 			this.add('-activate', pokemon, 'ability: Ripen');
//! 		},
//! 		onEatItem(item, pokemon) {
//! 			const weakenBerries = [
//! 				'Babiri Berry', 'Charti Berry', 'Chilan Berry', 'Chople Berry', 'Coba Berry', 'Colbur Berry', 'Haban Berry', 'Kasib Berry', 'Kebia Berry', 'Occa Berry', 'Passho Berry', 'Payapa Berry', 'Rindo Berry', 'Roseli Berry', 'Shuca Berry', 'Tanga Berry', 'Wacan Berry', 'Yache Berry',
//! 			];
//! 			// Record if the pokemon ate a berry to resist the attack
//! 			pokemon.abilityState.berryWeaken = weakenBerries.includes(item.name);
//! 		},
//! 		flags: {},
//! 		name: "Ripen",
//! 		rating: 2,
//! 		num: 247,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// Complex berry-enhancing ability
/// TODO: Needs item system, berry detection, boost manipulation, abilityState tracking
/// TODO: Multiple handlers: onTryHeal, onChangeBoost, onSourceModifyDamage, onTryEatItem, onEatItem
/// TODO: Doubles berry healing, doubles berry stat boosts, tracks berry-weaken effects
pub const ON_SOURCE_MODIFY_DAMAGE_PRIORITY: i32 = -1;
pub const ON_TRY_EAT_ITEM_PRIORITY: i32 = -1;

