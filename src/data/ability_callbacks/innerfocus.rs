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
pub fn on_try_add_volatile(_battle: &mut Battle, status: &Status, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (status.id === 'flinch') return null;
    if status.id == "flinch" {
        return AbilityHandlerResult::Null;
    }
    AbilityHandlerResult::Undefined
}

/// onTryBoost(boost, target, source, effect)
/// Prevents Attack reduction from Intimidate
pub fn on_try_boost(battle: &mut Battle, boost: &mut std::collections::HashMap<String, i8>, target: &Pokemon, _source: Option<&Pokemon>, effect_id: &str, _has_secondaries: bool) -> AbilityHandlerResult {
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

