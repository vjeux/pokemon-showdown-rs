//! Damp Ability
//!
//! Pokemon Showdown - http://pokemonshowdown.com/
//!
//! Generated from data/abilities.ts
//!
//! ```text
//! JS Source (data/abilities.ts):
//! 	damp: {
//! 		onAnyTryMove(target, source, effect) {
//! 			if (['explosion', 'mindblown', 'mistyexplosion', 'selfdestruct'].includes(effect.id)) {
//! 				this.attrLastMove('[still]');
//! 				this.add('cant', this.effectState.target, 'ability: Damp', effect, `[of] ${target}`);
//! 				return false;
//! 			}
//! 		},
//! 		onAnyDamage(damage, target, source, effect) {
//! 			if (effect && effect.name === 'Aftermath') {
//! 				return false;
//! 			}
//! 		},
//! 		flags: { breakable: 1 },
//! 		name: "Damp",
//! 		rating: 0.5,
//! 		num: 6,
//! 	},
//! ```


use crate::battle::{Battle, Arg};
use crate::data::moves::{MoveDef, MoveCategory, MoveTargetType};
use crate::pokemon::Pokemon;
use crate::dex_data::ID;
use super::{AbilityHandlerResult, Status, Effect};

/// Moves that are blocked by Damp
    const BLOCKED_MOVES: &[&str] = &["explosion", "mindblown", "mistyexplosion", "selfdestruct"];

    /// onAnyTryMove(target, source, effect)
    /// Blocks explosive moves from being used
    pub fn on_any_try_move(battle: &mut Battle, damp_pokemon: &Pokemon, target: &Pokemon, move_: &MoveDef) -> AbilityHandlerResult {
        // if (['explosion', 'mindblown', 'mistyexplosion', 'selfdestruct'].includes(effect.id))
        if BLOCKED_MOVES.contains(&move_.id.as_str()) {
            // this.attrLastMove('[still]');
            // this.add('cant', this.effectState.target, 'ability: Damp', effect, `[of] ${target}`);
            battle.add("cant", &[
                Arg::Pokemon(damp_pokemon),
                Arg::Str("ability: Damp"),
                Arg::Str(move_.id.as_str()),
                Arg::Str(&format!("[of] {}", target.name)),
            ]);
            // return false;
            return AbilityHandlerResult::False;
        }
        AbilityHandlerResult::Undefined
    }

    /// onAnyDamage(damage, target, source, effect)
    /// Blocks Aftermath damage
    pub fn on_any_damage(_damage: u32, _target: &Pokemon, _source: &Pokemon, effect_name: Option<&str>) -> AbilityHandlerResult {
        // if (effect && effect.name === 'Aftermath')
        if let Some(name) = effect_name {
            if name == "Aftermath" {
                return AbilityHandlerResult::False;
            }
        }
        AbilityHandlerResult::Undefined
    }
