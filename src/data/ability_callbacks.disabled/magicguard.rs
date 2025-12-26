//! Magic Guard Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	magicguard: {
//! 		onDamage(damage, target, source, effect) {
//! 			if (effect.effectType !== 'Move') {
//! 				if (effect.effectType === 'Ability') this.add('-activate', source, 'ability: ' + effect.name);
//! 				return false;
//! 			}
//! 		},
//! 		flags: {},
//! 		name: "Magic Guard",
//! 		rating: 4,
//! 		num: 98,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onDamage(damage, target, source, effect)
/// Prevents indirect damage (only takes damage from moves)
pub fn on_damage(battle: &mut Battle, _damage: u32, _target: &Pokemon, source: Option<&Pokemon>, effect: &Effect) -> AbilityHandlerResult {
    // if (effect.effectType !== 'Move')
    if effect.effect_type != "Move" {
        // if (effect.effectType === 'Ability')
        if effect.effect_type == "Ability" {
            if let Some(src) = source {
                // this.add('-activate', source, 'ability: ' + effect.name);
                battle.add("-activate", &[
                    Arg::Pokemon(src),
                    Arg::Str(&format!("ability: {}", effect.id))
                ]);
            }
        }
        // return false;
        return AbilityHandlerResult::False;
    }
    AbilityHandlerResult::Undefined
}

