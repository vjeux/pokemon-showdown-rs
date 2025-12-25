//! Solar Power Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	solarpower: {
//! 		onModifySpAPriority: 5,
//! 		onModifySpA(spa, pokemon) {
//! 			if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather())) {
//! 				return this.chainModify(1.5);
//! 			}
//! 		},
//! 		onWeather(target, source, effect) {
//! 			if (target.hasItem('utilityumbrella')) return;
//! 			if (effect.id === 'sunnyday' || effect.id === 'desolateland') {
//! 				this.damage(target.baseMaxhp / 8, target, target);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Solar Power",
//! 		rating: 2,
//! 		num: 94,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_MODIFY_SP_A_PRIORITY: i32 = 5;

/// onModifySpA(spa, pokemon)
/// Boosts Special Attack by 1.5x in sunny weather
pub fn on_modify_sp_a(battle: &mut Battle, _spa: u32, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (['sunnyday', 'desolateland'].includes(pokemon.effectiveWeather()))
    let weather = battle.field.effective_weather();
    if *weather == ID::new("sunnyday") || *weather == ID::new("desolateland") {
        // return this.chainModify(1.5);
        return AbilityHandlerResult::ChainModify(6144, 4096); // 1.5x
    }
    AbilityHandlerResult::Undefined
}

/// onWeather(target, source, effect)
/// Damages the Pokemon by 1/8 max HP each turn in sun
///
/// TODO: onWeather handler not yet implemented in battle system
/// When implemented, should:
/// 1. Check if target has Utility Umbrella item (return early if so)
/// 2. Check if effect.id is 'sunnyday' or 'desolateland'
/// 3. Call battle.damage(target.baseMaxhp / 8, target, target)
pub fn on_weather(_battle: &mut Battle, _target: &Pokemon, _source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (target.hasItem('utilityumbrella')) return;
    // TODO: Item checking not yet available

    // if (effect.id === 'sunnyday' || effect.id === 'desolateland')
    if effect.id == "sunnyday" || effect.id == "desolateland" {
        // this.damage(target.baseMaxhp / 8, target, target);
        // TODO: Damage call needs implementation
    }
    AbilityHandlerResult::Undefined
}

