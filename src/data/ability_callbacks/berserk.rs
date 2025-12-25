//! Berserk Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	berserk: {
//! 		onDamage(damage, target, source, effect) {
//! 			if (
//! 				effect.effectType === "Move" &&
//! 				!effect.multihit &&
//! 				!(effect.hasSheerForce && source.hasAbility('sheerforce'))
//! 			) {
//! 				this.effectState.checkedBerserk = false;
//! 			} else {
//! 				this.effectState.checkedBerserk = true;
//! 			}
//! 		},
//! 		onTryEatItem(item) {
//! 			const healingItems = [
//! 				'aguavberry', 'enigmaberry', 'figyberry', 'iapapaberry', 'magoberry', 'sitrusberry', 'wikiberry', 'oranberry', 'berryjuice',
//! 			];
//! 			if (healingItems.includes(item.id)) {
//! 				return this.effectState.checkedBerserk;
//! 			}
//! 			return true;
//! 		},
//! 		onAfterMoveSecondary(target, source, move) {
//! 			this.effectState.checkedBerserk = true;
//! 			if (!source || source === target || !target.hp || !move.totalDamage) return;
//! 			const lastAttackedBy = target.getLastAttackedBy();
//! 			if (!lastAttackedBy) return;
//! 			const damage = move.multihit && !move.smartTarget ? move.totalDamage : lastAttackedBy.damage;
//! 			if (target.hp <= target.maxhp / 2 && target.hp + damage > target.maxhp / 2) {
//! 				this.boost({ spa: 1 }, target, target);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Berserk",
//! 		rating: 2,
//! 		num: 201,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamage(damage, target, source, effect)
pub fn on_damage(_damage: u32, target: &mut Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (
    //     effect.effectType === "Move" &&
    //     !effect.multihit &&
    //     !(effect.hasSheerForce && source.hasAbility('sheerforce'))
    // )
    if effect.effect_type == "Move" {
        // Note: multihit and hasSheerForce checks are not available in current Effect struct
        // Simplified: assume non-multihit for now
        // this.effectState.checkedBerserk = false;
        target.ability_state.data.insert("checkedBerserk".to_string(), serde_json::Value::Bool(false));
    } else {
        // this.effectState.checkedBerserk = true;
        target.ability_state.data.insert("checkedBerserk".to_string(), serde_json::Value::Bool(true));
    }
    AbilityHandlerResult::Undefined
}

/// onTryEatItem(item)
pub fn on_try_eat_item(target: &Pokemon, item_id: &ID) -> AbilityHandlerResult {
    // const healingItems = [
    //     'aguavberry', 'enigmaberry', 'figyberry', 'iapapaberry', 'magoberry', 'sitrusberry', 'wikiberry', 'oranberry', 'berryjuice',
    // ];
    let healing_items = [
        "aguavberry", "enigmaberry", "figyberry", "iapapaberry", "magoberry",
        "sitrusberry", "wikiberry", "oranberry", "berryjuice",
    ];

    // if (healingItems.includes(item.id))
    if healing_items.contains(&item_id.as_str()) {
        // return this.effectState.checkedBerserk;
        let checked = target.ability_state.data.get("checkedBerserk")
            .and_then(|v| v.as_bool())
            .unwrap_or(true);
        if checked {
            return AbilityHandlerResult::True;
        } else {
            return AbilityHandlerResult::False;
        }
    }
    // return true;
    AbilityHandlerResult::True
}

/// onAfterMoveSecondary(target, source, move)
pub fn on_after_move_secondary(battle: &mut Battle, target: &mut Pokemon, source: Option<&Pokemon>, _move: &MoveDef) -> AbilityHandlerResult {
    // this.effectState.checkedBerserk = true;
    target.ability_state.data.insert("checkedBerserk".to_string(), serde_json::Value::Bool(true));

    // if (!source || source === target || !target.hp || !move.totalDamage) return;
    let source = match source {
        Some(s) => s,
        None => return AbilityHandlerResult::Undefined,
    };

    if source.side_index == target.side_index && source.position == target.position {
        return AbilityHandlerResult::Undefined;
    }

    if target.hp == 0 {
        return AbilityHandlerResult::Undefined;
    }

    // const lastAttackedBy = target.getLastAttackedBy();
    // if (!lastAttackedBy) return;
    // const damage = move.multihit && !move.smartTarget ? move.totalDamage : lastAttackedBy.damage;
    // Note: getLastAttackedBy is not fully implemented, using target.last_damage as approximation
    let damage = target.last_damage;

    if damage == 0 {
        return AbilityHandlerResult::Undefined;
    }

    // if (target.hp <= target.maxhp / 2 && target.hp + damage > target.maxhp / 2)
    if target.hp <= target.maxhp / 2 && target.hp + damage > target.maxhp / 2 {
        // this.boost({ spa: 1 }, target, target);
        let target_ref = (target.side_index, target.position);
        battle.boost(&[("spa", 1)], target_ref, Some(target_ref), None);
    }

    AbilityHandlerResult::Undefined
}

