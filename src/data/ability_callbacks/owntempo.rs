//! Own Tempo Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	owntempo: {
//! 		onUpdate(pokemon) {
//! 			if (pokemon.volatiles['confusion']) {
//! 				this.add('-activate', pokemon, 'ability: Own Tempo');
//! 				pokemon.removeVolatile('confusion');
//! 			}
//! 		},
//! 		onTryAddVolatile(status, pokemon) {
//! 			if (status.id === 'confusion') return null;
//! 		},
//! 		onHit(target, source, move) {
//! 			if (move?.volatileStatus === 'confusion') {
//! 				this.add('-immune', target, 'confusion', '[from] ability: Own Tempo');
//! 			}
//! 		},
//! 		onTryBoost(boost, target, source, effect) {
//! 			if (effect.name === 'Intimidate' && boost.atk) {
//! 				delete boost.atk;
//! 				this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Own Tempo', `[of] ${target}`);
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Own Tempo",
//! 		rating: 1.5,
//! 		num: 20,
//! 	},
//! ```

use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// onUpdate(pokemon)
/// Cures confusion
pub fn on_update(battle: &mut Battle, pokemon: &mut Pokemon) -> AbilityHandlerResult {
    // if (pokemon.volatiles['confusion'])
    if pokemon.has_volatile(&ID::new("confusion")) {
        // this.add('-activate', pokemon, 'ability: Own Tempo');
        battle.add("-activate", &[Arg::Pokemon(pokemon), Arg::Str("ability: Own Tempo")]);
        // pokemon.removeVolatile('confusion');
        pokemon.remove_volatile(&ID::new("confusion"));
    }
    AbilityHandlerResult::Undefined
}

/// onTryAddVolatile(status, pokemon)
/// Prevents confusion
pub fn on_try_add_volatile(_battle: &mut Battle, status: &Status, _pokemon: &Pokemon) -> AbilityHandlerResult {
    // if (status.id === 'confusion') return null;
    if status.id == "confusion" {
        return AbilityHandlerResult::Null;
    }
    AbilityHandlerResult::Undefined
}

/// onHit(target, source, move)
/// Shows immune message for confusion moves
///
/// TODO: onHit handler not yet implemented
/// TODO: Needs move.volatileStatus
/// When implemented, should:
/// 1. Check if move.volatileStatus === 'confusion'
/// 2. Add immune message
pub fn on_hit(battle: &mut Battle, /* TODO: Add parameters */) -> AbilityHandlerResult {
    // TODO: Implement 1-to-1 from JS
    AbilityHandlerResult::Undefined
}

/// onTryBoost(boost, target, source, effect)
/// Blocks Intimidate
pub fn on_try_boost(battle: &mut Battle, boost: &mut std::collections::HashMap<String, i8>, target: &Pokemon, _source: Option<&Pokemon>, effect_id: &str, _has_secondaries: bool) -> AbilityHandlerResult {
    // if (effect.name === 'Intimidate' && boost.atk)
    if effect_id == "intimidate" && boost.contains_key("atk") {
        // delete boost.atk;
        boost.remove("atk");
        // this.add('-fail', target, 'unboost', 'Attack', '[from] ability: Own Tempo', `[of] ${target}`);
        battle.add("-fail", &[
            Arg::Pokemon(target),
            Arg::Str("unboost"),
            Arg::Str("Attack"),
            Arg::Str("[from] ability: Own Tempo"),
            Arg::Str(&format!("[of] {}", target.position))
        ]);
    }
    AbilityHandlerResult::Undefined
}

