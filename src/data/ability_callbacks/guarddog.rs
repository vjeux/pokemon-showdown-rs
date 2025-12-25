//! Guard Dog Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	guarddog: {
//! 		onDragOutPriority: 1,
//! 		onDragOut(pokemon) {
//! 			this.add('-activate', pokemon, 'ability: Guard Dog');
//! 			return null;
//! 		},
//! 		onTryBoostPriority: 2,
//! 		onTryBoost(boost, target, source, effect) {
//! 			if (effect.name === 'Intimidate' && boost.atk) {
//! 				delete boost.atk;
//! 				this.boost({atk: 1}, target, target, null, false, true);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Guard Dog",
//! 		rating: 2,
//! 		num: 275,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_DRAG_OUT_PRIORITY: i32 = 1;
pub const ON_TRY_BOOST_PRIORITY: i32 = 2;

/// onDragOut(pokemon)
/// Prevents forced switches (e.g. from Roar, Whirlwind, Dragon Tail)
/// TODO: onDragOut handler needs to be wired into battle engine
pub fn on_drag_out(battle: &mut Battle, pokemon: &Pokemon) -> AbilityHandlerResult {
    // this.add('-activate', pokemon, 'ability: Guard Dog');
    battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Guard Dog")]);
    // return null;
    AbilityHandlerResult::Null
}

/// onTryBoost(boost, target, source, effect)
/// Blocks Intimidate and boosts Attack by 1 stage instead
pub fn on_try_boost(battle: &mut Battle, boost: &mut std::collections::HashMap<String, i8>, target: &Pokemon, _source: Option<&Pokemon>, effect_name: &str, _has_secondaries: bool) -> AbilityHandlerResult {
    // if (effect.name === 'Intimidate' && boost.atk)
    if effect_name == "Intimidate" && boost.contains_key("atk") {
        // delete boost.atk;
        boost.remove("atk");
        // this.boost({atk: 1}, target, target, null, false, true);
        let target_ref = (target.side_index, target.position);
        battle.boost(&[("atk", 1)], target_ref, Some(target_ref), None);
    }
    AbilityHandlerResult::Undefined
}
