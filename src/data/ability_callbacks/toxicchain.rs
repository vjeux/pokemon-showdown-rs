//! Toxic Chain Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	toxicchain: {
//! 		onSourceDamagingHit(damage, target, source, move) {
//! 			// Despite not being a secondary, Shield Dust / Covert Cloak block Toxic Chain's effect
//! 			if (target.hasAbility('shielddust') || target.hasItem('covertcloak')) return;
//! 
//! 			if (this.randomChance(3, 10)) {
//! 				target.trySetStatus('tox', source);
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Toxic Chain",
//! 		rating: 4.5,
//! 		num: 305,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceDamagingHit(damage, target, source, move)
/// 30% chance to badly poison target when source (with this ability) deals damage
pub fn on_source_damaging_hit(battle: &mut Battle, _damage: u32, target: &Pokemon, _source: &Pokemon, _move_: &MoveDef) -> AbilityHandlerResult {
    // Despite not being a secondary, Shield Dust / Covert Cloak block Toxic Chain's effect
    // if (target.hasAbility('shielddust') || target.hasItem('covertcloak')) return;
    if target.ability.as_str() == "shielddust" {
        return AbilityHandlerResult::Undefined;
    }
    // TODO: Item checking (hasItem('covertcloak')) not yet available

    // if (this.randomChance(3, 10))
    if battle.prng.random_chance(3, 10) {
        // target.trySetStatus('tox', source);
        let target_ref = (target.side_index, target.position);
        battle.sides[target_ref.0].pokemon[target_ref.1].try_set_status(ID::new("tox"), None);
    }

    AbilityHandlerResult::Undefined
}

