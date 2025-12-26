//! Liquid Ooze Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	liquidooze: {
//! 		onSourceTryHeal(damage, target, source, effect) {
//! 			this.debug(`Heal is occurring: ${target} <- ${source} :: ${effect.id}`);
//! 			const canOoze = ['drain', 'leechseed', 'strengthsap'];
//! 			if (canOoze.includes(effect.id)) {
//! 				this.damage(damage);
//! 				return 0;
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Liquid Ooze",
//! 		rating: 2.5,
//! 		num: 64,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onSourceTryHeal(damage, target, source, effect)
/// Damages draining opponent instead of healing them
///
/// TODO: onSourceTryHeal handler not yet called by battle engine
pub fn on_source_try_heal(battle: &mut Battle, damage: u32, target: &Pokemon, source: &Pokemon, effect: &Effect) -> AbilityHandlerResult {
    // this.debug(`Heal is occurring: ${target} <- ${source} :: ${effect.id}`);
    // const canOoze = ['drain', 'leechseed', 'strengthsap'];
    const CAN_OOZE: &[&str] = &["drain", "leechseed", "strengthsap"];

    // if (canOoze.includes(effect.id))
    if CAN_OOZE.contains(&effect.id.as_str()) {
        // this.damage(damage);
        let source_ref = (source.side_index, source.position);
        let target_ref = (target.side_index, target.position);
        battle.damage(damage as i32, Some(source_ref), Some(target_ref), None, false);

        // return 0;
        return AbilityHandlerResult::Number(0);
    }

    AbilityHandlerResult::Undefined
}

