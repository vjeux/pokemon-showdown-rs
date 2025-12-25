//! Inner Focus Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	innerfocus: {
//! 		onTryAddVolatile(status, pokemon) {
//! 			if (status.id === 'flinch') return null;
//! 		},
//! 		onTryBoost(boost, target, source, effect) {
//! 			if (effect.name === 'Intimidate' && boost.atk) {
//! 				delete boost.atk;
//! 				this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Inner Focus', `[of] ${target}`);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Inner Focus",
//! 		rating: 1,
//! 		num: 39,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onTryAddVolatile(status, pokemon)
/// Prevents flinching
///
/// TODO: onTryAddVolatile handler not yet implemented in battle system
/// When implemented, should:
/// 1. Check if status.id === 'flinch'
/// 2. Return null to prevent the volatile status
pub fn on_try_add_volatile(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    // Requires volatile status system
    AbilityHandlerResult::Undefined
}

/// onTryBoost(boost, target, source, effect)
/// Prevents Attack reduction from Intimidate
pub fn on_try_boost(battle: &mut Battle, boost: &mut std::collections::HashMap<String, i8>, target: &Pokemon, source: Option<&Pokemon>, effect_id: &str, has_secondaries: bool) -> AbilityHandlerResult {
    // if (effect.name === 'Intimidate' && boost.atk)
    if effect_id == "intimidate" {
        if let Some(&atk_boost) = boost.get("atk") {
            if atk_boost < 0 {
                // delete boost.atk;
                boost.remove("atk");
                // this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Inner Focus', `[of] ${target}`);
                battle.add("-fail", &[
                    Arg::Pokemon(target),
                    Arg::Str("unboost"),
                    Arg::Str("Attack"),
                    Arg::Str("[from] ability: Inner Focus"),
                    Arg::Str(&format!("[of] {}", target.name))
                ]);
            }
        }
    }
    AbilityHandlerResult::Undefined
}

