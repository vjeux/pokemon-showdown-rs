//! Wind Power Ability - Gets charged when hit by wind moves
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	windpower: {
//! 		onDamagingHitOrder: 1,
//! 		onDamagingHit(damage, target, source, move) {
//! 			if (move.flags['wind']) {
//! 				target.addVolatile('charge');
//! 			}
//! 		},
//! 		onSideConditionStart(target, source, sideCondition) {
//! 			const pokemon = this.effectState.target;
//! 			if (sideCondition.id === 'tailwind') {
//! 				pokemon.addVolatile('charge');
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Wind Power",
//! 		rating: 1,
//! 		num: 277,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

pub const ON_DAMAGING_HIT_ORDER: i32 = 1;

/// onDamagingHit(damage, target, source, move)
/// Gets charged when hit by wind moves
pub fn on_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, _source: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
    // if (move.flags['wind'])
    if move_.flags.wind {
        // target.addVolatile('charge');
        let target_ref = (target.side_index, target.position);
        battle.sides[target_ref.0].pokemon[target_ref.1].add_volatile(ID::new("charge"));
    }
    AbilityHandlerResult::Undefined
}

/// onSideConditionStart(target, source, sideCondition)
/// Gets charged when Tailwind is set on user's side
pub fn on_side_condition_start(battle: &mut Battle, _side_index: usize, _source: Option<&Pokemon>, side_condition_id: &str, ability_holder: &Pokemon) -> AbilityHandlerResult {
    // const pokemon = this.effectState.target;
    // if (sideCondition.id === 'tailwind')
    if side_condition_id == "tailwind" {
        // pokemon.addVolatile('charge');
        let holder_ref = (ability_holder.side_index, ability_holder.position);
        battle.sides[holder_ref.0].pokemon[holder_ref.1].add_volatile(ID::new("charge"));
    }
    AbilityHandlerResult::Undefined
}

