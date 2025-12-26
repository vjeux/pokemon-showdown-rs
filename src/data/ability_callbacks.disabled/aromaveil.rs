//! Aroma Veil Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	aromaveil: {
//! 		onAllyTryAddVolatile(status, target, source, effect) {
//! 			if (['attract', 'disable', 'encore', 'healblock', 'taunt', 'torment'].includes(status.id)) {
//! 				if (effect.effectType === 'Move') {
//! 					const effectHolder = this.effectState.target;
//! 					this.add('-block', target, 'ability: Aroma Veil', `[of] ${effectHolder}`);
//! 				}
//! 				return null;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Aroma Veil",
//! 		rating: 2,
//! 		num: 165,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::move_types::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// Status effects blocked by Aroma Veil
    const BLOCKED_STATUS: &[&str] = &["attract", "disable", "encore", "healblock", "taunt", "torment"];

    /// onAllyTryAddVolatile(status, target, source, effect)
    /// Blocks certain volatile status conditions from allies
    pub fn on_ally_try_add_volatile(battle: &mut Battle, status_id: &str, target: &Pokemon, _source: Option<&Pokemon>, effect_type: &str, effect_holder: &Pokemon) -> AbilityHandlerResult {
        if BLOCKED_STATUS.contains(&status_id) {
            if effect_type == "Move" {
                battle.add("-block", &[Arg::Pokemon(target), Arg::Str("ability: Aroma Veil"), Arg::Str(&format!("[of] {}", effect_holder.name))]);
            }
            return AbilityHandlerResult::Null;
        }
        AbilityHandlerResult::Undefined
    }
