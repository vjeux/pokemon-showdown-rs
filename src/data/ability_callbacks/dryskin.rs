//! Dry Skin Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	dryskin: {
//! 		onTryHit(target, source, move) {
//! 			if (target !== source && move.type === 'Water') {
//! 				if (!this.heal(target.baseMaxhp / 4)) {
//! 					this.add('-immune', target, '[from] ability: Dry Skin');
//! 				}
//! 				return null;
//! 			}
//! 		},
//! 		onSourceBasePowerPriority: 17,
//! 		onSourceBasePower(basePower, attacker, defender, move) {
//! 			if (move.type === 'Fire') {
//! 				return this.chainModify(1.25);
//! 			}
//! 		},
//! 		onWeather(target, source, effect) {
//! 			if (target.hasItem('utilityumbrella')) return;
//! 			if (effect.id === 'raindance' || effect.id === 'primordialsea') {
//! 				this.heal(target.baseMaxhp / 8);
//! 			} else if (effect.id === 'sunnyday' || effect.id === 'desolateland') {
//! 				this.damage(target.baseMaxhp / 8, target, target);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Dry Skin",
//! 		rating: 3,
//! 		num: 87,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryHit(target, source, move)
    pub fn on_try_hit(battle: &mut Battle, target: &mut Pokemon, source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        let target_ref = (target.side_index, target.position);
        if target_ref != (source.side_index, source.position) && move_.move_type == "Water" {
            let heal_amount = target.maxhp / 4;
            if battle.heal(heal_amount as i32, Some(target_ref), None, None) == Some(0) {
                battle.add("-immune", &[Arg::Pokemon(target), Arg::Str("[from] ability: Dry Skin")]);
            }
            return AbilityHandlerResult::Null;
        }
        AbilityHandlerResult::Undefined
    }

    pub const ON_SOURCE_BASE_POWER_PRIORITY: i32 = 17;

    /// onSourceBasePower(basePower, attacker, defender, move)
    pub fn on_source_base_power(_base_power: u32, _attacker: &Pokemon, _defender: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        if move_.move_type == "Fire" {
            // 1.25x = 5120/4096
            return AbilityHandlerResult::ChainModify(5120, 4096);
        }
        AbilityHandlerResult::Undefined
    }

    /// onWeather(target, source, effect)
    pub fn on_weather(battle: &mut Battle, target: &mut Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
        if target.has_item(&["utilityumbrella"]) {
            return AbilityHandlerResult::Undefined;
        }
        let target_ref = (target.side_index, target.position);
        if effect.id == "raindance" || effect.id == "primordialsea" {
            let heal_amount = target.maxhp / 8;
            battle.heal(heal_amount as i32, Some(target_ref), None, None);
        } else if effect.id == "sunnyday" || effect.id == "desolateland" {
            let damage_amount = target.maxhp / 8;
            battle.damage(damage_amount as i32, Some(target_ref), Some(target_ref), None, false);
        }
        AbilityHandlerResult::Undefined
    }
